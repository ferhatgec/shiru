extern crate shiru;

use shiru::{
    Highlight,
    LanguageData,

    shiru_python::python,

    parse::{ Parse }
};

fn main() {
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

    // Initialize Python keywords, colors etc.
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
