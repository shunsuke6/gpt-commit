use crate::errors::{ArgsError, ArgumentError, CommandError, OptionError};
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Command {
    Run,
    Config,
}

#[derive(PartialEq, Debug)]
pub enum Option {
    NoCache,
    Not,
    ApiKey,
    Language,
    URL,
    Model,
}

impl FromStr for Command {
    type Err = CommandError;
    fn from_str(s: &str) -> Result<Self, CommandError> {
        match s {
            "run" => Ok(Command::Run),
            "config" => Ok(Command::Config),
            _ => Err(CommandError::InvalidCommand),
        }
    }
}

pub fn parse_command(s: &str) -> Result<Command, CommandError> {
    s.parse()
}

impl FromStr for Option {
    type Err = OptionError;
    fn from_str(s: &str) -> Result<Self, OptionError> {
        match s {
            "-n" | "--no-cache" | "--no-cached" => Ok(Option::NoCache),
            "not" => Ok(Option::Not),
            "-k" | "--api-key" => Ok(Option::ApiKey),
            "-l" | "--language" => Ok(Option::Language),
            "-m" | "--model" => Ok(Option::Model),
            "-u" | "--url" => Ok(Option::URL),
            _ => Err(OptionError::InvalidOption),
        }
    }
}

impl fmt::Display for Option {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Option::ApiKey => write!(f, "API Key"),
            Option::Language => write!(f, "Language"),
            Option::URL => write!(f, "OpenAI URL"),
            Option::Model => write!(f, "OpenAI Model"),
            _ => Ok(()),
        }
    }
}
pub fn parse_option(s: &str) -> Result<Option, OptionError> {
    s.parse()
}

fn argument_confirmation(cmd: &Command, op: &Option) -> bool {
    match cmd {
        Command::Run => matches!(op, Option::NoCache | Option::Not),
        Command::Config => matches!(
            op,
            Option::Not | Option::Language | Option::ApiKey | Option::Model | Option::URL
        ),
    }
}
fn get_args(cmd: &str, op: &str, val: &str) -> Result<Args, ArgumentError> {
    let _command: Command = parse_command(cmd)
        .map_err(|_| ArgumentError::CommandError(CommandError::InvalidCommand))?;

    let _option: Option =
        parse_option(op).map_err(|_| ArgumentError::OptionError(OptionError::InvalidOption))?;

    if !argument_confirmation(&_command, &_option) {
        return Err(ArgumentError::ArgsError(ArgsError::InvalidArgs));
    }

    if !val.is_empty() && !needs_value(&_command, &_option) {
        return Err(ArgumentError::ArgsError(ArgsError::InvalidArgs));
    }

    Ok(Args {
        command: _command,
        option: _option,
        value: val.to_string(),
    })
}

fn needs_value(cmd: &Command, op: &Option) -> bool {
    cmd == &Command::Config && !matches!(op, Option::NoCache | Option::Not)
}

pub struct Args {
    command: Command,
    option: Option,
    value: String,
}

impl Args {
    pub fn new(args: &Vec<String>) -> Result<Args, ArgumentError> {
        if args.len() < 2 {
            return Err(ArgumentError::ArgsError(ArgsError::MissingArgs));
        }
        match args.len() {
            2 => get_args(&args[1], "not", ""),
            3 => get_args(&args[1], &args[2], ""),
            4 => get_args(&args[1], &args[2], &args[3]),
            _ => Err(ArgumentError::ArgsError(ArgsError::InvalidArgs)),
        }
    }
    pub fn command(&self) -> &Command {
        &self.command
    }

    pub fn option(&self) -> &Option {
        &self.option
    }
    pub fn option_is_not(&self) -> bool {
        self.option == Option::Not
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}
