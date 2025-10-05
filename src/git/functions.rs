use crate::git::structs::GitDiff;

pub fn check_git_repository_existence() -> bool {
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

pub fn get_git_diff() -> GitDiff {
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
