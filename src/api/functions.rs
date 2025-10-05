use std::env;
use std::time::Duration;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use crate::api::structs::{APIError, ApiMessage, ChatCompletionRequest, ChatCompletionResponse};
use crate::git::structs::GitDiff;

pub fn make_api_request(diff: &GitDiff) -> String {
    let api_key = match get_api_key() {
        key if !key.is_empty() => key,
        _ => {
            eprintln!("API key not configured.");
            return "".to_string();
        }
    };

    let client = match Client::builder().timeout(Duration::from_secs(30)).build() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error while creating http client: {}", e);
            return format!("{}", e);
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .unwrap_or_else(|_| HeaderValue::from_static("Bearer invalid")),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let system_message = "You are an expert Git commit message writer specializing in analyzing code changes and creating precise, meaningful commit messages.";

    let message_count = 10;
    let mut user_message = format!(
        "Your task is to generate exactly {} {} style commit message{} based on the provided git diff.",
        message_count,
        "Conventional Commits",
        if message_count > 1 { "s" } else { "" },
    );

    let format = [
        "## Requirements:",
        "1. Language: Write all messages in english",
        "2. Format: Strictly follow the conventional commit format:",
        "<type>: <description>",
        "3. Allowed Types:",
        "  - docs: 'Documentation only changes'",
        "  - style: 'Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)'",
        "  - refactor: 'A code change that neither fixes a bug nor adds a feature'",
        "  - perf: 'A code change that improves performance'",
        "  - test: 'Adding missing tests or correcting existing tests'",
        "  - build: 'Changes that affect the build system or external dependencies'",
        "  - ci: 'Changes to CI configuration files, scripts'",
        "  - chore: 'Other changes that don't modify src or test files'",
        "  - revert: 'Reverts a previous commit'",
        "  - feat: 'A new feature'",
        "  - fix: 'A bug fix'",
        "",
        "## Guidelines:",
        "- Subject line: Max ${maxLength} characters, imperative mood, no period",
        "- Analyze the diff to understand:",
        "  * What files were changed",
        "  * What functionality was added, modified, or removed",
        "  * The impact of changes",
        "- For the commit type, choose based on:",
        "  * feat: New functionality or feature",
        "  * fix: Bug fixes or error corrections",
        "  * refactor: Code restructuring without changing functionality",
        "  * docs: Documentation changes only",
        "  * style: Formatting, missing semi-colons, etc",
        "  * test: Adding or modifying tests",
        "  * chore: Maintenance tasks, dependency updates",
        "  * perf: Performance improvements",
        "  * build: Build system or external dependency changes",
        "  * ci: CI configuration changes",
        "- Body (when needed):",
        "  * Explain the motivation for the change",
        "  * Compare previous behavior with new behavior",
        "  * Note any breaking changes or important details",
        "- Footer: Include references to issues, breaking changes if applicable",
        "",
        "## Analysis Approach:",
        "1. Identify the primary purpose of the changes",
        "2. Group related changes together",
        "3. Determine the most appropriate type",
        "4. Write a clear, concise subject line",
        "5. Add body details for complex changes",
        "",
        "Remember: The commit message should help future developers understand WHY this change was made, not just WHAT was changed.",
        "Here is the git diff to analyze:",
        "",
    ];

    for line in format.iter() {
        user_message.push_str("\n");
        user_message.push_str(line);
    }
    user_message.push_str(diff.diff.as_str());
    user_message.push_str("\n\nProvide only the commit messages without any additional text.");

    let request_body = ChatCompletionRequest {
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
    };
    let json_body = match serde_json::to_string(&request_body) {
        Ok(body) => body,
        Err(e) => {
            eprintln!("Error while serializing request body: {}", e);
            return format!("{}", e);
        }
    };

    let response: Response = match client
        .post("https://api.deepseek.com/v1/chat/completions")
        .headers(headers)
        .body(json_body)
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
