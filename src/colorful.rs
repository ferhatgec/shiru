// What?
// Colodot- & Colorized-inspired colorize source crate.
//
// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//

pub enum Colors {
    Reset,
    Black = 30,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White, // 37
    Default = 39,

    // Light colors
    LightBlack = 90,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightWhite, // 97
    LightDefault = 99,

    // Background colors
    BackgroundBlack = 40,
    BackgroundRed,
    BackgroundGreen,
    BackgroundYellow,
    BackgroundBlue,
    BackgroundMagenta,
    BackgroundCyan,
    BackgroundWhite, // 47
    BackgroundDefault = 49,

    // Background light colors
    BackgroundLightBlack = 100,
    BackgroundLightRed,
    BackgroundLightGreen,
    BackgroundLightYellow,
    BackgroundLightBlue,
    BackgroundLightMagenta,
    BackgroundLightCyan,
    BackgroundLightWhite, // 107
    BackgroundLightDefault = 109
}

pub enum Types {
    Light     ,
    Bold      ,
    Dim       ,
    Italic    ,
    Underlined,
    Blink     ,
    RapidBlink,
    Reverse   ,
    Hidden     // 8
}

pub mod colorful {
    use crate::colorful::{ Colors, Types };

    static ESC: &'static str = "\x1b";

    pub fn ok(__color: Colors, __type: Types) -> String {
        format!("{}[{};{}m", ESC, __type as u8, __color as u16)
    }

    pub fn make(__color: Colors, __type: Types, data: &String) -> String {
        ok(__color, __type) + data
    }
}
