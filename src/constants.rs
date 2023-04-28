pub const DEFAULT_PROMPT: &str = "
Use the following rules to display the commit message.
Use Conventional Commits for the title.
No additional messages are needed beyond the title.
Provide 3 commit message suggestions, if possible.
Write the reason for choosing a specific commit message below it.
Start the commit message with a verb.
Write the commit message in English.
Display the reasons for the commit message as a list.
Start the <Description> with a capital letter.
";

pub const TRANSLATION_SETTING: &str = "
Write the reason in {}.
Please make sure to provide the reasons in {}.
";

pub const OUTPUT_TEMPLATE: &str = "
## template
1. <type>: <Description>

reason:

-
";
pub const CONFIG_FILE_NAME: &str = "config.yaml";

pub const DEFAULT_MODEL: &str = "gpt-3.5-turbo";

pub const DEFAULT_OPEN_AI_URL: &str = "https://api.openai.com/v1/chat/completions";

pub const USAGE: &str = "
\x1b[34m
Usage:
  gptcommit <command> [options]

Commands:
  run     Run gptcommit
  config   Set configuration

Command-specific options:
  run:
    -n, --no-chache, --no-chached Run without Cache

  config:
    -k, --api-key [api key]            Set the OpenAI API key
    -l, --language [language]          Response Message Language Setting (e.g., English, Japanese) (default: English)
    -m , --model [model]               Set OpenAI model for use (e.g., gpt-3.5-turbo, gpt-4) (default: gpt-3.5-turbo)
    -u , --url [url]                   When using other models, URL setting is possible

e.g.:
  gtpcommit run
  gtpcommit config -l
\x1b[0m
";
