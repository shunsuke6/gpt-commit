use crate::constants::{DEFAULT_PROMPT, OUTPUT_TEMPLATE, TRANSLATION_SETTING};
use std::string::String;

pub struct Prompt {
    pub message: String,
}

impl Prompt {
    pub fn new(language: String, changes: String) -> Prompt {
        let mut str = String::from(DEFAULT_PROMPT);

        if is_translate(&language) {
            let s = TRANSLATION_SETTING.replace("{}", &language);
            str.push_str(&s);
        }

        str.push_str(OUTPUT_TEMPLATE);
        str.push_str(&changes);
        Prompt {
            message: str.to_string(),
        }
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

fn is_translate(language: &String) -> bool {
    !language.is_empty() && language != "English"
}
