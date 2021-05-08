// ShiRu
// Extensible syntax highlighting engine, written in Rust (Implementation of ShiC++)
//
// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

mod parse;
mod shiru_cpp;

pub struct LanguageData {
    pub(crate) keywords: Vec<String>,
    pub(crate) colors  : Vec<String>
}

pub struct Highlight {
    pub(crate) data: LanguageData
}

impl Highlight {
    pub fn init(&mut self, data: LanguageData) {
        self.data = data;
    }

    pub fn colorize(&mut self, data: &String) -> String {
        let temporary = String::from(data);
        let mut i: usize = 0;

        for token in &self.data.keywords {
            if token == &temporary {
                return format!("{}{}\x1b[0m ", self.data.colors.get(i).unwrap(), temporary);
            }

            i += 1;
        }

        temporary
    }
}

mod tests {
    use crate::{
        Highlight,
        LanguageData,

        shiru_cpp::c_plus_plus,

        parse::{ Parse }
    };

    #[test]
    fn cpp_test() {
        let mut parse   : Parse = Parse {
            highlight: Highlight {
                data: LanguageData {
                    keywords: vec![],
                    colors: vec![]
                }
            },
            tokens: vec![],
            is_data: false
        };

        let mut highlight: Highlight = Highlight {
            data: LanguageData {
                keywords: vec![],
                colors: vec![]
            }
        };

        highlight.init(
            LanguageData {
                keywords: c_plus_plus::init_keywords(),
                colors  : c_plus_plus::init_colors()
            }
        );

        parse.init(highlight);

        println!("{}",parse.parse(format!("{}",
                    "\
                    #include <iostream>\n \
                    \n                                        \
                    int main(int argc) {\n                    \
                        std::cout << \"Hello, world!\\n\";\n  \
                    }\
                    ")));
    }
}