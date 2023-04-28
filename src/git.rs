use crate::errors::RunTimeError;
use std::process::{Command, Stdio};

pub fn get_diff(is_cached: bool) -> String {
    let arg: Vec<String> = if is_cached {
        vec!["--no-pager", "diff", "--cached"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    } else {
        vec!["--no-pager", "diff"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    };

    let result = Command::new("git")
        .args(&arg)
        .stderr(Stdio::null())
        .output()
        .expect("git is not found");

    String::from_utf8_lossy(&result.stdout).to_string()
}
pub fn git_commit(commit_message: String, is_staged: bool) -> Result<bool, RunTimeError> {
    let op = match is_staged {
        true => "-m",
        false => "-am",
    };

    let status = Command::new("git")
        .args(["commit", op, &commit_message])
        .status()
        .expect("git commit failed");

    if status.success() {
        let err_msg = format!("commit succeeded: {}", commit_message);
        return Err(RunTimeError::GitError(err_msg));
    }
    Ok(status.success())
}
