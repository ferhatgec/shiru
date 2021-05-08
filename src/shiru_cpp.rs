// What?
// C++ syntax interface for ShiRu.
//
// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod c_plus_plus {
    use crate::colorful::{ colorful, Colors, Types };

    pub fn init_keywords() -> Vec<String> {
        vec![
            ok("int"),
            ok("float"),
            ok("double"),
            ok("long"),

            ok("int8_t"),
            ok("int16_t"),
            ok("int32_t"),
            ok("int64_t"),

            ok("uint8_t"),
            ok("uint16_t"),
            ok("uint32_t"),
            ok("uint64_t")
        ]
    }

    pub fn init_colors() -> Vec<String> {
        vec![
            colorful::ok(Colors::Red, Types::Light), // int
            colorful::ok(Colors::Red, Types::Light), // float
            colorful::ok(Colors::Red, Types::Light), // double
            colorful::ok(Colors::Red, Types::Light), // long
            colorful::ok(Colors::Red, Types::Light), // char

            colorful::ok(Colors::Red, Types::Light), // int8_t
            colorful::ok(Colors::Red, Types::Light), // int16_t
            colorful::ok(Colors::Red, Types::Light), // int32_t
            colorful::ok(Colors::Red, Types::Light), // int64_t

            colorful::ok(Colors::Red, Types::Light), // uint8_t
            colorful::ok(Colors::Red, Types::Light), // uint16_t
            colorful::ok(Colors::Red, Types::Light), // uint32_t
            colorful::ok(Colors::Red, Types::Light), // uint64_t
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