// What?
// Python syntax interface for ShiRu.
//
// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod python {
    use crate::colorful::{ colorful, Colors, Types };

    pub fn init_keywords() -> Vec<String> {
        vec![
            ok("int"),
            ok("float"),
            ok("str"),
            ok("char"),

            ok("if"),
            ok("elif"),
            ok("else"),

            ok("import"),
            ok("from")
        ]
    }

    pub fn init_colors() -> Vec<String> {
        vec![
            colorful::ok(Colors::Red, Types::Light), // int
            colorful::ok(Colors::Red, Types::Light), // float
            colorful::ok(Colors::Red, Types::Light), // str
            colorful::ok(Colors::Red, Types::Light), // char

            colorful::ok(Colors::LightRed, Types::Light), // if
            colorful::ok(Colors::LightRed, Types::Light), // elif
            colorful::ok(Colors::LightRed, Types::Light), // else

            colorful::ok(Colors::Yellow, Types::Light     ), // import
            colorful::ok(Colors::LightYellow, Types::Light)
        ]
    }

    pub fn built_in_keywords() -> Vec<String> {
        vec![
            ok("#"),
            ok("'")
        ]
    }

    pub fn built_in_colors() -> Vec<String> {
        vec![
            colorful::ok(Colors::LightBlack, Types::Light), // SingleLineComment,
            colorful::ok(Colors::White,      Types::Light)  // VariableData
        ]
    }

    pub fn init_op_colors() -> Vec<String> {
        vec![
            colorful::ok(Colors::Yellow, Types::Light), // GlobalOperators::Addition    0
            colorful::ok(Colors::Yellow, Types::Light), // GlobalOperators::Subtraction
            colorful::ok(Colors::Yellow, Types::Light), // GlobalOperators::Division
            colorful::ok(Colors::Yellow, Types::Light), // GlobalOperators::Multiplication
            colorful::ok(Colors::Yellow, Types::Light), // GlobalOperators::Modulo

            colorful::ok(Colors::Blue, Types::Light), // GlobalOperators::GreaterThan
            colorful::ok(Colors::Blue, Types::Light), // GlobalOperators::LessThan

            colorful::ok(Colors::LightRed, Types::Light), // GlobalOperators::Not

            colorful::ok(Colors::LightBlue, Types::Light), // GlobalOperators::AndBit
            colorful::ok(Colors::LightBlue, Types::Light), // GlobalOperators::OrBit
            colorful::ok(Colors::LightBlue, Types::Light), // GlobalOperators::XorBit
            colorful::ok(Colors::LightBlue, Types::Light), // GlobalOperators::NotBit

            colorful::ok(Colors::LightWhite, Types::Light)  // GlobalOperators::Assignment
        ]
    }

    pub fn ok(data: &str) -> String { data.to_string() }
}