# GPT-Commit

[Japanese](./README.jp.md)

A tool to make ChatGPT create a commit message based on a `git diff`.
This tool was originally intended for private use and has been modified for public use.

Commit message creation for [conversion-commits]

[convertional-commits]:(https://github.com/conventional-commits/conventionalcommits.org)

## Why use it?

- Making efficient - It is easy to change when there is a log of commit messages that are easy to understand during development, but it is a burden to come up with appropriate names.

- Validity - Avoid commit logs full of `Add` for feature additions, `modify` or `change` for changes, and `Fix` for bug fixes.

- Learning - Learning English expression skills, as my first language is not English.

## Notes

Ask ChatGPT to display three suggestions after choose the actual message to use

GPT-4 recommendation.
GPT-3.5 is not very accurate. 1/3 of commits often have commit messages that do not match the changes.

With proper commits, the output of the commit message will be better.

## Installation

```bash
cargo insatll gpt_commit
```

## Usage

Put the changes you want to commit into staging and run gpt-commit.

```bash
$ gptcommit run

Requesting to ChatGPT...
Please wait a moment.
# Actual result.
# 1
# 2
# 3
Please choose [1-3]
```

Pick the number of the commit message that you think is a good one from the results of the API.

If you want to commit the entire workspace instead of `git diff --cached`, use the `-n(--no-cached)` option.
In this case it will do `git commit -am "{choose message}"`.

```bash
gptcommit run -n
gptcommit run --no-cached
```

### Configuration

以下設定を用意しています。

| item     | Content                                   | Options       |
| -------- | ----------------------------------------- | ------------- |
| api_key  | ChatGPT's API key                         | -a,--api-key  |
| language | language of reply (except commit message) | -l,--language |
| url      | URL of ChatGPT's API                      | -u,--url      |
| model    | model of ChatGPT                          | -m,--model    |

The first time you run it, it will create a config file, so please enter your API key.

```bash
$ gptcommit config
Config file is not found
Create a Config file

Enter your ChatGPT API Key
{your API_Key}
```

Defaults are set the first time, so use the cofing command to set them if necessary.

```bash
gptcommit config -l Japanese
```

```bash
# Can be run without having to specify it
$ gptcommit config -l
Please enter language
> Spanish
```

## License

MIT

## Author

shunsuke6
