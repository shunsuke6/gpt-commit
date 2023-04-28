use colored::*;
use gpt_commit::argument::Args;
use gpt_commit::argument::Command;
use gpt_commit::config::Config;
use gpt_commit::constants::USAGE;
use gpt_commit::errors::ArgumentError;
use gpt_commit::run::gpt_commit_run;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argument_result: Result<Args, ArgumentError> = Args::new(&args);

    if let Err(error_message) = argument_result {
        println!("{}", error_message);
        display_usage();
        process::exit(1);
    }

    let argument: Args = argument_result.unwrap();

    let config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error reading config file: {}", e.red());
            std::process::exit(1);
        }
    };

    match argument.command() {
        Command::Run => {
            do_run(argument, config);
        }
        Command::Config => {
            do_config(argument, config);
        }
    }
}

fn display_usage() {
    print!("{}", USAGE);
}

fn do_run(args: Args, config: Config) {
    if let Err(error_message) = gpt_commit_run(args, config) {
        eprint!("{}", error_message);
        std::process::exit(1);
    }
}

fn do_config(args: Args, mut config: Config) {
    if args.option_is_not() {
        config.display();
        std::process::exit(0);
    }
    config.update_config(args.option(), args.value());
    println!("{}", "Update sucusess".blue());
    config.display();
}
