extern crate shiru;

use shiru::{
    Highlight,
    LanguageData,

    shiru_cpp::c_plus_plus,

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

    // Initialize C++ keywords, colors etc.
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
