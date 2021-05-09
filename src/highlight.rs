// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use crate::{
    parse::parse,

    SingleLineComment,
    VariableData,

    LanguageData
};

pub struct Highlight {
    pub(crate) data: LanguageData
}

impl Highlight {
    pub fn init(&mut self, data: LanguageData) {
        self.data = data;
    }

    pub fn comment(&self, data: &String) -> String {
        format!("{}{}\x1b[0m ",
                self.data.builtin_colors.get(SingleLineComment as usize).unwrap(),
                data)
    }

    pub fn var_data(&self, data: &String) -> String {
        format!("{}{}\x1b[0m ",
                self.data.builtin_colors.get(VariableData as usize).unwrap(),
                data)
    }

    pub fn colorize(&mut self, data: &String) -> String {
        let temporary = String::from(data);
        let mut i: usize = 0;

        if temporary.len() > 1 {
            for token in &self.data.keywords {
                if token == &temporary {
                    return format!("{}{}\x1b[0m ", self.data.colors.get(i).unwrap(), temporary);
                }

                i += 1;
            }

            return temporary;
        }

        let temporary_op: char = temporary.chars().next().unwrap();

        for operator in parse::GLOBAL_OPERATORS {
            if operator == &temporary_op {
                return format!("{}{}\x1b[0m", self.data.global_colors.get(i).unwrap(), temporary);
            }

            i += 1;
        }

        temporary
    }
}