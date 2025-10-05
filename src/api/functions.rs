use std::env;
use std::time::Duration;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use crate::api::structs::{APIError, ApiMessage, ChatCompletionRequest, ChatCompletionResponse};
use crate::git::structs::GitDiff;
use std::fs;

pub fn make_api_request(diff: &GitDiff) -> String {
    let client = get_client();

    let response: Response = match client
        .post("https://api.deepseek.com/v1/chat/completions")
        .headers(get_headers())
        .body(get_json_body(diff))
        .send()
    {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error while requesting api: {}", e);
            return format!("{}", e);
        }
    };

    let status = response.status();

    let response_text = match response.text() {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error while reading response: {}", e);
            return format!("{}", e);
        }
    };

    if status.is_success() {
        match serde_json::from_str::<ChatCompletionResponse>(&response_text) {
            Ok(api_response) => {
                if let Some(choice) = api_response.choices.first() {
                    format!("{}", choice.message.content)
                } else {
                    "Error no content in response".to_string()
                }
            }
            Err(e) => {
                eprintln!("Error while deserialize api response: {}", e);
                format!("{}", e)
            }
        }
    } else {
        if let Ok(api_error) = serde_json::from_str::<APIError>(&response_text) {
            format!(
                "❌ API Error ({}): {}",
                api_error.error.error_type, api_error.error.message
            )
        } else {
            format!("❌ HTTP Error {}: {}", status, response_text)
        }
    }
}

pub fn get_api_key() -> String {
    env::var("DEEPSEEK_API_KEY").unwrap_or_else(|_| String::new())
}

pub fn check_api_key_existence() -> bool {
    match env::var("DEEPSEEK_API_KEY") {
        Ok(val) => {
            if val.is_empty() {
                false
            } else {
                true
            }
        }
        Err(_) => false,
    }
}

fn get_headers() -> HeaderMap {
    let api_key = match get_api_key() {
        key if !key.is_empty() => key,
        _ => {
            eprintln!("API key not configured.");
            return HeaderMap::new();
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .unwrap_or_else(|_| HeaderValue::from_static("Bearer invalid")),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    headers
}

fn get_client() -> Client {
    Client::builder().timeout(Duration::from_secs(30)).build().unwrap_or_else(|e| {
        eprintln!("Error while creating http client: {}", e);
        Client::new()
    })
}

fn get_request_body(diff: &GitDiff) -> ChatCompletionRequest {
    let system_message = get_system_message();
    let user_message = get_user_message(diff);

    ChatCompletionRequest {
        model: "deepseek-chat".to_string(),
        messages: vec![
            ApiMessage {
                role: "system".to_string(),
                content: system_message.to_string(),
            },
            ApiMessage {
                role: "user".to_string(),
                content: user_message,
            },
        ],
        temperature: Some(0.7),
        max_tokens: Some(2000),
        stream: false,
    }
}

fn get_json_body(diff: &GitDiff) -> String {
    serde_json::to_string_pretty(&get_request_body(diff)).unwrap_or_else(|e| {
        eprintln!("Error while serializing request body: {}", e);
        format!("{}", e)
    })
}

fn get_system_message() -> String {
    "You are an expert Git commit message writer specializing in analyzing code changes and creating precise, meaningful commit messages.".to_string()
}

fn get_user_message(diff: &GitDiff) -> String {
    let message_count = 10;
    let mut user_message = format!(
        "Your task is to generate exactly {} {} style commit message{} based on the provided git diff.",
        message_count,
        "Conventional Commits",
        if message_count > 1 { "s" } else { "" },
    );

    let format = fs::read_to_string("./src/api/prompt.md").expect("Unable to read prompt.md");

    user_message.push_str(&format);
    user_message.push_str(diff.diff.as_str());
    user_message.push_str("\n\nProvide only the commit messages without any additional text.");
    user_message
}