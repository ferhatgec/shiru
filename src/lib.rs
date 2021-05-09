// ShiRu
// Extensible syntax highlighting engine, written in Rust (Implementation of ShiC++)
//
// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod parse;
pub mod colorful;
pub mod highlight;

pub mod shiru_cpp;
pub mod shiru_python;

use crate::{
    highlight::Highlight,
    Builtins::{ SingleLineComment, VariableData }
};

pub enum GlobalOperators {
    Addition   = 0, // x + y
    Subtraction   , // x - y
    Division      , // x / y
    Multiplication, // x * y
    Modulo        , // x % y

    GreaterThan   , // x > y
    LessThan      , // x < y

    Not           , // !(x == y)

    AndBit        , // &
    OrBit         , // |
    XorBit        , // ^
    NotBit        , // ~

    Assignment      // x = y
}

pub enum Builtins {
    SingleLineComment,
    VariableData
}

pub struct LanguageData {
    pub keywords      : Vec<String>,
    pub colors        : Vec<String>,

    pub builtins      : Vec<String>,
    pub builtin_colors: Vec<String>,

    pub global_colors : Vec<String>
}

mod tests {
    use crate::{
        Highlight,
        LanguageData,

        shiru_cpp::c_plus_plus,
        shiru_python::python,

        parse::{ Parse }
    };

    #[test]
    fn cpp_test() {
        let mut parse   : Parse = Parse {
            highlight: Highlight {
                data: LanguageData {
                    keywords: vec![],
                    colors: vec![],
                    builtins: vec![],
                    builtin_colors: vec![],
                    global_colors: vec![]
                }
            },
            tokens: vec![],
            is_data: false,
            is_comment: false
        };

        let mut highlight: Highlight = Highlight {
            data: LanguageData {
                keywords: vec![],
                colors: vec![],
                builtins: vec![],
                builtin_colors: vec![],
                global_colors: vec![]
            }
        };

        highlight.init(
            LanguageData {
                keywords: c_plus_plus::init_keywords(),
                colors  : c_plus_plus::init_colors(),
                builtins: c_plus_plus::built_in_keywords(),
                builtin_colors: c_plus_plus::built_in_colors(),
                global_colors: c_plus_plus::init_op_colors()
            }
        );

        parse.init(highlight);

        println!("{}",
                 parse.parse(format!("{}",
                 "\
                 #include <iostream>\n \
                 // Hello, world\n \
                 int main(int argc, char** argv) {\n \
                     int test = 10 + 20;
                     std::cout << \"10 + 20 = \" << test << '\\n';\n \
                 }\
                 ")));
    }

    #[test]
    fn python() {
        let mut parse   : Parse = Parse {
            highlight: Highlight {
                data: LanguageData {
                    keywords: vec![],
                    colors: vec![],
                    builtins: vec![],
                    builtin_colors: vec![],
                    global_colors: vec![]
                }
            },
            tokens: vec![],
            is_data: false,
            is_comment: false
        };

        let mut highlight: Highlight = Highlight {
            data: LanguageData {
                keywords: vec![],
                colors: vec![],
                builtins: vec![],
                builtin_colors: vec![],
                global_colors: vec![]
            }
        };

        highlight.init(
            LanguageData {
                keywords: python::init_keywords(),
                colors  : python::init_colors(),
                builtins: python::built_in_keywords(),
                builtin_colors: python::built_in_colors(),
                global_colors: python::init_op_colors()
            }
        );

        parse.init(highlight);

        println!("{}",
                 parse.parse(format!("{}",
                 "# Hello, world\n \
                 import sys\n \
                 test = 10 + 20\n \
                 if test == 30:\n \
                    print('10 + 20 =' , test)\
                 ")));
    }
}