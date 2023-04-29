use colored::Colorize;
use std::error::Error as StdError;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub enum ArgumentError {
    ArgsError(ArgsError),
    CommandError(CommandError),
    OptionError(OptionError),
}

#[derive(Debug)]
pub enum ArgsError {
    InvalidArgs,
    MissingArgs,
}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    InvalidCommand,
}

#[derive(Debug)]
pub enum OptionError {
    InvalidOption,
}
#[derive(Debug)]
pub enum RunTimeError {
    APIError(String),
    GitError(String),
    GitCommitError(String),
    TerminalError(Box<dyn StdError>),
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ArgumentError::ArgsError(ArgsError::InvalidArgs) => {
                write!(f, "{}", "Invalid Argsument".red())
            }
            ArgumentError::ArgsError(ArgsError::MissingArgs) => {
                write!(f, "{}", "Missing Argsument".red())
            }
            ArgumentError::CommandError(CommandError::InvalidCommand) => {
                write!(f, "{}", "Invalid Command".red())
            }
            ArgumentError::OptionError(OptionError::InvalidOption) => {
                write!(f, "{}", "Invalid Option".red())
            }
        }
    }
}
impl fmt::Display for RunTimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RunTimeError::APIError(ref err) => write!(f, "API defaild: {}", err.red()),
            RunTimeError::GitError(ref err) => write!(f, "git diff failed: {}", err.red()),
            RunTimeError::GitCommitError(ref err) => write!(f, "git commit failed: {}", err.red()),
            RunTimeError::TerminalError(ref err) => {
                write!(f, "TerminalError {}", err.to_string().red())
            }
        }
    }
}

pub fn handle_error(error: &ArgumentError) {
    println!("An error occurred: {}", error);
}

impl From<Box<dyn StdError>> for RunTimeError {
    fn from(error: Box<dyn StdError>) -> Self {
        RunTimeError::TerminalError(error)
    }
}
