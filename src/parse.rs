// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use crate::{
    highlight::Highlight,

    Builtins::{ VariableData, SingleLineComment }
};

pub struct Parse {
    pub highlight : Highlight,

    pub tokens    : Vec<String>,

    pub is_data   : bool       ,
    pub is_comment: bool
}

pub mod parse {
    pub static TOKENS: &'static [char] = &[
        '<',
        '>',
        '(',
        ')',
        '*',
        '[',
        ']'
    ];

    pub static GLOBAL_OPERATORS: &'static [char] = &[
        '+', // GlobalOperators::Addition
        '-', // GlobalOperators::Subtraction
        '/', // GlobalOperators::Division
        '*', // GlobalOperators::Multiplication
        '%', // GlobalOperators::Modulo

        '>', // GlobalOperators::GreaterThan
        '<', // GlobalOperators::LessThan

        '!', // GlobalOperators::Not

        '&', // GlobalOperators::AndBit
        '|', // GlobalOperators::OrBit
        '^', // GlobalOperators::XorBit
        '~', // GlobalOperators::NotBit

        '='  // GlobalOperators::Assignment
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

            if self.is_comment {
                if !token.ends_with('\n') {
                    generated.push_str(&self.highlight.comment(token));

                    continue;
                }

                self.is_comment = false;
            }

            if self.is_data {
                generated.push_str(&self.highlight.var_data(token));

                if !token.ends_with(self.highlight.data.builtins[VariableData as usize]
                    .chars().next().unwrap()) {
                    continue;
                }

                self.is_data = false;

                continue;
            }

            if token == &self.highlight.data.builtins[SingleLineComment as usize] {
                self.is_comment = true;

                generated.push_str(&self.highlight.comment(token));

                continue;
            }

            if token.starts_with(self.highlight.data.builtins[VariableData as usize]
                .chars().next().unwrap())
                || token.ends_with(self.highlight.data.builtins[VariableData as usize]
                .chars().next().unwrap()) {
                self.is_data = true;

                generated.push_str(format!("{} ", self.highlight.var_data(token)).as_str());

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