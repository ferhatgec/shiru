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
            ok("\x1b[0;31m"), // int
            ok("\x1b[0;31m"), // float
            ok("\x1b[0;31m"), // double
            ok("\x1b[0;31m"), // long

            ok("\x1b[0;31m"), // int8_t
            ok("\x1b[0;31m"), // int16_t
            ok("\x1b[0;31m"), // int32_t
            ok("\x1b[0;31m"), // int64_t

            ok("\x1b[0;31m"), // uint8_t
            ok("\x1b[0;31m"), // uint16_t
            ok("\x1b[0;31m"), // uint32_t
            ok("\x1b[0;31m"), // uint64_t
        ]
    }

    pub fn ok(data: &str) -> String { data.to_string() }
}