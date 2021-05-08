// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use crate::Highlight;

pub struct Parse {
    pub(crate) highlight: Highlight,

    pub(crate) tokens   : Vec<String>,

    pub(crate) is_data  : bool
}

pub mod parse {
    pub(crate) static TOKENS: &'static [char] = &[
        '<',
        '>',
        '(',
        ')',
        '*',
        '[',
        ']'
    ];
}

impl Parse {
    pub fn init(&mut self, data: Highlight) {
        self.highlight = data;
    }

    pub fn parse(&mut self, data: String) -> String {
        let mut generated = String::new();

        for line in data.split('\n').map(|s| s.to_string()).collect::<Vec<String>>() {
            let keep = self.tokenize(&self.make_alpha(&line));

            for token in keep {
                self.tokens.push(token);
            }

            self.tokens.push("\n".to_string());
        }

        for token in &self.tokens {
            if token.is_empty() { continue; }

            if self.is_data {
                if token.ends_with('"') {
                    generated.push_str(&token);

                    continue;
                }

                self.is_data = false;
            }

            if token.starts_with('"') || token.ends_with('"') {
                self.is_data = true;

                generated.push_str(format!("{} ", token).as_str());

                continue;
            }

            generated.push_str(self.highlight.colorize(&token).as_str());
        }

        generated
    }

    pub fn tokenize(&self, data: &String) -> Vec<String> {
        data.split(' ').map(|s| s.to_string()).collect()
    }

    pub fn make_alpha(&self, data: &String) -> String {
        let mut temporary = String::from(data);

        for character in parse::TOKENS {
            temporary = self.replace_with(&temporary, *character);
        }

        temporary
    }

    pub fn replace_with(&self, token: &String, character: char) -> String {
        token.replace(character, format!(" {} ", character).as_str()).to_string()
    }
}