use crate::argument::Args;
use crate::argument::Option;
use crate::choices::get_choices;
use crate::config::Config;
use crate::errors::RunTimeError;
use crate::git::{get_diff, git_commit};
use crate::prompt::Prompt;
use colored::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{stdin, stdout, Write};
use tokio::runtime::Runtime;

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

async fn send_api(config: Config, prompt: &String) -> Result<String, Box<dyn std::error::Error>> {
    let message = json!({
        "role": "assistant",
        "content": prompt
    });
    let request_payload = json!({
        "model": config.model(),
        "messages": [message],
    });

    let client = Client::new();
    let response = client
        .post(config.url())
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", config.api_key()))
        .json(&request_payload)
        .send()
        .await?;

    if !response.status().is_success() {
        let error = response.status().to_string();
        return Err(error.into());
    }

    let responce_text = &response.json::<ApiResponse>().await?.choices[0]
        .message
        .content;

    println!("{}", &responce_text.clone().blue());

    Ok(responce_text.to_string())
}

pub fn gpt_commit_run(args: Args, config: Config) -> Result<bool, RunTimeError> {
    let is_using_cached: bool = is_cached(args);

    let change_description: String = get_diff(is_using_cached);
    if change_description.is_empty() {
        return Err(RunTimeError::GitError("Nothing to diff".to_string()));
    }

    let prompt = get_prompt(config.language(), change_description);

    let rt = Runtime::new().unwrap();
    println!("Requesting to ChatGPT...\nPlease wait a moment.");
    let commit_message_result = rt.block_on(async {
        match send_api(config, &prompt).await {
            Ok(api_result) => Ok(choose_commit(api_result)),
            Err(err) => Err(RunTimeError::APIError(err.to_string())),
        }
    });

    let commit_message = match commit_message_result {
        Ok(msg) => msg,
        Err(err) => return Err(err),
    };

    match git_commit(commit_message, is_using_cached) {
        Ok(_) => Ok(true),
        Err(err) => Err(RunTimeError::GitCommitError(err.to_string())),
    }
}
fn choose_commit(api_result_message: String) -> String {
    let choices = get_choices(api_result_message);
    stdout().flush().unwrap();

    let mut input = String::new();
    loop {
        print!("Please choose [1-{}]", choices.len());
        stdout().flush().unwrap();

        stdin().read_line(&mut input).expect("Error reading input");

        if let Ok(choice_number) = input.trim().parse::<usize>() {
            if (1..=choices.len()).contains(&choice_number) {
                return choices[choice_number - 1].clone();
            }
            println!("{}", "Invalid input. Please try again.".yellow());
            input.clear();
        }
    }
}

fn is_cached(args: Args) -> bool {
    *args.option() != Option::NoCache
}

fn get_prompt(language: String, change_description: String) -> String {
    Prompt::new(language, change_description)
        .message()
        .to_string()
}
