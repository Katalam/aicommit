mod git;
mod api;
mod clipboard;

use crate::api::functions::{make_api_request, check_api_key_existence};
use crate::git::functions::{check_git_repository_existence, get_git_diff};
use crate::clipboard::functions::copy_to_clipboard;

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

    println!("\nSelect a commit message by number (or press Enter to skip):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    if let Ok(choice) = input.parse::<usize>() {
        if choice > 0 && choice <= messages.len() {
            copy_to_clipboard(messages[choice - 1]);
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

