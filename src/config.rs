use crate::argument::Option;
use crate::constants::{CONFIG_FILE_NAME, DEFAULT_MODEL, DEFAULT_OPEN_AI_URL};
use colored::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Config {
    api_key: String,
    language: String,
    url: String,
    model: String,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let config_path = Self::get_config_path();
        println!("{}", config_path.display());
        if config_path.exists() {
            Self::read_config(config_path)
        } else {
            println!(
                "{}",
                "Config file is not found\n Create a Config file".yellow()
            );
            Self::create_config(config_path)
        }
    }

    fn create_config<P: AsRef<Path>>(path: P) -> Result<Config, String> {
        let _api_key = get_stdin("Enter your ChatGPT API Key");
        let config = Config {
            api_key: _api_key,
            language: "".to_string(),
            url: DEFAULT_OPEN_AI_URL.to_string(),
            model: DEFAULT_MODEL.to_string(),
        };
        write_config(&config, path).unwrap();
        Ok(config)
    }
    fn read_config<P: AsRef<Path>>(path: P) -> Result<Config, String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| e.to_string())?;
        serde_yaml::from_str(&contents).map_err(|e| e.to_string())
    }

    pub fn language(&self) -> String {
        self.language.clone()
    }

    pub fn api_key(&self) -> String {
        self.api_key.clone()
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn model(&self) -> String {
        self.model.clone()
    }

    pub fn update_config(&mut self, op: &Option, value: String) {
        let mut val = value;

        if val.is_empty() {
            let _ = &self.display_current_value(op);
            val = get_stdin(&op.to_string());
        }

        match &op {
            Option::Language => self.language = val,
            Option::ApiKey => self.api_key = val,
            Option::URL => self.url = val,
            Option::Model => self.model = val,
            _ => {}
        }
        write_config(self, Self::get_config_path()).unwrap();
    }
    pub fn display(&self) {
        println!("{}", "Config".green());
        println!("{}: {}", "API Key".green(), self.api_key.green());
        println!("{}: {}", "Language".green(), self.language.green());
        println!("{}: {}", "URL".green(), self.url.green());
        println!("{}: {}", "Model".green(), self.model.green());
    }

    fn display_current_value(&self, op: &Option) {
        let val = match &op {
            Option::Language => self.language(),
            Option::ApiKey => self.api_key(),
            Option::URL => self.url(),
            Option::Model => self.model(),
            _ => "".to_string(),
        };
        println!("The current setting is {}", val);
    }

    fn get_config_path() -> PathBuf {
        let executable_path = env::current_exe().expect("Failed to get executable path");
        let install_dir = executable_path
            .parent()
            .expect("Failed to get install directory");

        PathBuf::from(install_dir).join(CONFIG_FILE_NAME)
    }
}
fn get_stdin(text: &str) -> String {
    println!("{} {}", "Please enter".blue(), text.blue());
    let mut str = String::new();
    std::io::stdin().read_line(&mut str).unwrap();
    str.trim().to_string()
}

fn write_config<P: AsRef<Path>>(config: &Config, path: P) -> Result<(), String> {
    let contents = serde_yaml::to_string(config).map_err(|e| e.to_string())?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(|e| e.to_string())?;

    file.write_all(contents.as_bytes())
        .map_err(|e| e.to_string())
}
