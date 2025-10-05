extern crate dotenv;

use reqwest::blocking::{Client, Response};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;
use clipboard::{ClipboardContext, ClipboardProvider};

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ApiMessage>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: ApiMessage,
}

#[derive(Deserialize, Debug)]
struct APIError {
    error: ErrorDetail,
}

#[derive(Deserialize, Debug)]
struct ErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
}

struct GitDiff {
    diff: String,
    file_names: Vec<String>,
}

fn main() {
    let api_key_exists = check_api_key_existence();

    if !api_key_exists {
        println!("\x1b[93mDEEPSEEK_API_KEY environment variable is not set. Aborting...\x1b[0m");
        return;
    }

    let is_git_repo = check_git_repository_existence();

    if !is_git_repo {
        println!("\x1b[93mThis directory is not a Git repository.\x1b[0m");
        return;
    }

    let git_diff = get_git_diff();

    if git_diff.file_names.len() == 0 {
        println!("\x1b[93mNo staged changes detected.\x1b[0m");
        return;
    }

    println!("Staged files:");
    for file_name in &git_diff.file_names {
        println!("{}", file_name);
    }

    let api_response = make_api_request(&git_diff);

    let messages = api_response
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>();

    if messages.is_empty() {
        println!("\x1b[93mNo commit messages generated.\x1b[0m");
        return;
    }

    println!("\nGenerated commit messages:");

    for (i, message) in messages.iter().enumerate() {
        println!("{}. {}", i + 1, message.trim());
    }

    // Optionally, you can prompt the user to select one of the generated messages

    println!("\nSelect a commit message by number (or press Enter to skip):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    if let Ok(choice) = input.parse::<usize>() {
        if choice > 0 && choice <= messages.len() {
            let selected_message = messages[choice - 1];
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(selected_message.to_string()).unwrap();

            println!("\x1b[92mSelected commit message copied to clipboard:\x1b[0m");
        } else {
            println!("\x1b[93mInvalid selection.\x1b[0m");
        }
    } else if !input.is_empty() {
        println!("\x1b[93mInvalid input.\x1b[0m");
    } else {
        println!("No commit message selected.");
    }
}

fn check_git_repository_existence() -> bool {
    use std::process::Command;

    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .expect("Failed to execute git command");

    if output.status.success() {
        let is_inside = String::from_utf8_lossy(&output.stdout);
        if is_inside.trim() != "true" {
            return false;
        }
    } else {
        println!(
            "Error checking Git repository: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    true
}

fn get_git_diff() -> GitDiff {
    use std::process::Command;

    let diff = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .arg("--diff-algorithm=minimal")
        .output()
        .expect("Failed to execute git diff command");

    let names = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .arg("--diff-algorithm=minimal")
        .arg("--name-only")
        .output()
        .expect("Failed to execute git diff command");

    let mut git_diff = GitDiff {
        diff: String::new(),
        file_names: Vec::new(),
    };

    if diff.status.success() {
        git_diff.diff = String::from_utf8_lossy(&diff.stdout).to_string();
    }

    if names.status.success() {
        let names_str = String::from_utf8_lossy(&names.stdout);
        git_diff.file_names = names_str.lines().map(|s| s.to_string()).collect();
    }

    git_diff
}

fn check_api_key_existence() -> bool {
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

fn get_api_key() -> String {
    env::var("DEEPSEEK_API_KEY").unwrap_or_else(|_| String::new())
}
fn make_api_request(diff: &GitDiff) -> String {
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
