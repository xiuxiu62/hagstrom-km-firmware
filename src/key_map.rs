use crate::{command, key, shift};
use lazy_static::lazy_static;
use std::collections::HashMap;

macro_rules! key_map {
    ($($key:expr => $val:expr),*) => {{
        let mut map = HashMap::new();
        $(map.insert($key, $val);)*

        map
    }};
}

lazy_static! {
    // (Press, Release)
    pub static ref KEY_MAP: HashMap<KeyCode, (u8, u8)> = key_map! {
        KeyCode::One => (2, 130),
        KeyCode::Two => (3, 131),
        KeyCode::Three => (4, 132),
        KeyCode::Four => (5, 133),
        KeyCode::Five => (6, 134),
        KeyCode::Six => (7, 135),
        KeyCode::Seven => (8, 136),
        KeyCode::Eight => (9, 137),
        KeyCode::Nine => (10, 138),
        KeyCode::Zero => (11, 139),

        KeyCode::A => (31, 159),
        KeyCode::B => (50, 178),
        KeyCode::C => (48, 176),
        KeyCode::D => (33, 161),
        KeyCode::E => (19, 147),
        KeyCode::F => (34, 162),
        KeyCode::G => (35, 163),
        KeyCode::H => (36, 164),
        KeyCode::I => (24, 152),
        KeyCode::J => (37, 165),
        KeyCode::K => (38, 166),
        KeyCode::L => (39, 167),
        KeyCode::M => (52, 180),
        KeyCode::N => (51, 179),
        KeyCode::O => (25, 153),
        KeyCode::P => (26, 154),
        KeyCode::Q => (17, 145),
        KeyCode::R => (20, 148),
        KeyCode::S => (32, 160),
        KeyCode::T => (21, 149),
        KeyCode::U => (23, 151),
        KeyCode::V => (49, 177),
        KeyCode::W => (18, 146),
        KeyCode::X => (47, 175),
        KeyCode::Y => (22, 150),
        KeyCode::Z => (46, 174),

        KeyCode::Tilde => (1, 129),
        KeyCode::Space => (61, 189),
        KeyCode::Dash => (12, 140),
        KeyCode::Equal => (13, 141),
        KeyCode::LBracket => (27, 155),
        KeyCode::RBracket => (28, 156),
        KeyCode::BackSlash => (29, 157),
        KeyCode::SemiColon => (40, 168),
        KeyCode::Quote => (41, 169),
        KeyCode::Comma => (53, 181),
        KeyCode::Period => (54, 182),
        KeyCode::ForwardSlash => (55, 183),
        KeyCode::BackSpace => (15, 143),
        KeyCode::Tab => (16, 144),
        KeyCode::Caps => (30, 158),
        KeyCode::Enter => (43, 171),
        KeyCode::Shift => (44, 172),
        KeyCode::Control => (58, 186),
        KeyCode::Alt => (60, 188),
        KeyCode::Super => (70, 198),
        KeyCode::Escape => (110, 238),
        KeyCode::Left => (79, 207),
        KeyCode::Up => (83, 211),
        KeyCode::Down => (84, 212),
        KeyCode::Right => (89, 217),

        KeyCode::F1 => (112, 240),
        KeyCode::F2 => (113, 241),
        KeyCode::F3 => (114, 242),
        KeyCode::F4 => (115, 243),
        KeyCode::F5 => (116, 244),
        KeyCode::F6 => (117, 245),
        KeyCode::F7 => (118, 246),
        KeyCode::F8 => (119, 247),
        KeyCode::F9 => (120, 248),
        KeyCode::F10 => (121, 249),
        KeyCode::F11 => (122, 250),
        KeyCode::F12 => (123, 251)
    };
}

#[derive(Hash, PartialEq, Eq)]
pub enum KeyCode {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Tilde,
    Space,
    Dash,
    Equal,
    LBracket,
    RBracket,
    BackSlash,
    SemiColon,
    Quote,
    Comma,
    Period,
    ForwardSlash,
    BackSpace,
    Tab,
    Caps,
    Enter,
    Shift,
    Control,
    Alt,
    Super,
    Escape,
    Left,
    Up,
    Down,
    Right,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

#[macro_export]
macro_rules! message {
    ($data:expr) => {
        crate::key_map::_message($data)
    };
}

pub fn _message(data: &str) -> Vec<u8> {
    data.chars()
        .map(|char| character_packet(char).unwrap())
        .flatten()
        .collect()
}

fn character_packet(char: char) -> Result<Vec<u8>, String> {
    let packet = match char {
        '1' => key!(KeyCode::One),
        '!' => shift!(KeyCode::One),
        '2' => key!(KeyCode::Two),
        '@' => shift!(KeyCode::Two),
        '3' => key!(KeyCode::Three),
        '#' => shift!(KeyCode::Three),
        '4' => key!(KeyCode::Four),
        '$' => shift!(KeyCode::Four),
        '5' => key!(KeyCode::Five),
        '%' => shift!(KeyCode::Five),
        '6' => key!(KeyCode::Six),
        '^' => shift!(KeyCode::Six),
        '7' => key!(KeyCode::Seven),
        '&' => shift!(KeyCode::Seven),
        '8' => key!(KeyCode::Eight),
        '*' => shift!(KeyCode::Eight),
        '9' => key!(KeyCode::Nine),
        '(' => shift!(KeyCode::Nine),
        '0' => key!(KeyCode::Zero),
        ')' => shift!(KeyCode::Zero),
        '-' => key!(KeyCode::Dash),
        '_' => shift!(KeyCode::Dash),
        '=' => key!(KeyCode::Equal),
        '+' => shift!(KeyCode::Equal),

        'a' => key!(KeyCode::A),
        'A' => shift!(KeyCode::A),
        'b' => key!(KeyCode::B),
        'B' => shift!(KeyCode::B),
        'c' => key!(KeyCode::C),
        'C' => shift!(KeyCode::C),
        'd' => key!(KeyCode::D),
        'D' => shift!(KeyCode::D),
        'e' => key!(KeyCode::E),
        'E' => shift!(KeyCode::E),
        'f' => key!(KeyCode::F),
        'F' => shift!(KeyCode::F),
        'g' => key!(KeyCode::G),
        'G' => shift!(KeyCode::G),
        'h' => key!(KeyCode::H),
        'H' => shift!(KeyCode::H),
        'i' => key!(KeyCode::I),
        'I' => shift!(KeyCode::I),
        'j' => key!(KeyCode::J),
        'J' => shift!(KeyCode::J),
        'k' => key!(KeyCode::K),
        'K' => shift!(KeyCode::K),
        'l' => key!(KeyCode::L),
        'L' => shift!(KeyCode::L),
        'm' => key!(KeyCode::M),
        'M' => shift!(KeyCode::M),
        'n' => key!(KeyCode::N),
        'N' => shift!(KeyCode::N),
        'o' => key!(KeyCode::O),
        'O' => shift!(KeyCode::O),
        'p' => key!(KeyCode::P),
        'P' => shift!(KeyCode::P),
        'q' => key!(KeyCode::Q),
        'Q' => shift!(KeyCode::Q),
        'r' => key!(KeyCode::R),
        'R' => shift!(KeyCode::R),
        's' => key!(KeyCode::S),
        'S' => shift!(KeyCode::S),
        't' => key!(KeyCode::T),
        'T' => shift!(KeyCode::T),
        'u' => key!(KeyCode::U),
        'U' => shift!(KeyCode::U),
        'v' => key!(KeyCode::V),
        'V' => shift!(KeyCode::V),
        'w' => key!(KeyCode::W),
        'W' => shift!(KeyCode::W),
        'x' => key!(KeyCode::X),
        'X' => shift!(KeyCode::X),
        'y' => key!(KeyCode::Y),
        'Y' => shift!(KeyCode::Y),
        'z' => key!(KeyCode::Z),
        'Z' => shift!(KeyCode::Z),

        '[' => key!(KeyCode::LBracket),
        '{' => shift!(KeyCode::LBracket),
        ']' => key!(KeyCode::RBracket),
        '}' => shift!(KeyCode::RBracket),
        '\\' => key!(KeyCode::BackSlash),
        '|' => shift!(KeyCode::BackSlash),
        ';' => key!(KeyCode::SemiColon),
        ':' => shift!(KeyCode::SemiColon),
        '\'' => key!(KeyCode::Quote),
        '\"' => shift!(KeyCode::Quote),
        ',' => key!(KeyCode::Comma),
        '<' => shift!(KeyCode::Comma),
        '.' => key!(KeyCode::Period),
        '>' => shift!(KeyCode::Period),
        '/' => key!(KeyCode::ForwardSlash),
        '?' => shift!(KeyCode::ForwardSlash),

        ' ' => command!(KeyCode::Space),
        '\n' | '\r' => command!(KeyCode::Enter),
        '\t' => command!(KeyCode::Tab),

        char => return Err(format!("Unimplemented: Char({char})")),
    };

    Ok(packet)
}

#[macro_export]
macro_rules! key {
    ($key_code: expr) => {
        _text(&vec![$key_code])
    };

    ($($key_code: expr),*) => {{
        _text(&vec![$($key_code),*])
    }};
}

#[macro_export]
macro_rules! shift {
    ($key_code: expr) => {
        command![KeyCode::Shift, $key_code]
    };

    ($($key_code: expr),*) => {{
        command![$(KeyCode::Shift, $key_code),*]
    }};
}
#[macro_export]
macro_rules! command {
    ($key_code: expr) => {
        crate::key_map::_command(&vec![$key_code])
    };

    ($($key_code: expr),*) => {{
        crate::key_map::_command(&vec![$($key_code),*])
    }};
}

fn _text(keys: &[KeyCode]) -> Vec<u8> {
    keys.iter()
        .map(|keycode| vec![press(keycode), release(keycode)])
        .flatten()
        .collect()
}

pub fn _command(keys: &[KeyCode]) -> Vec<u8> {
    keys.iter()
        .map(|key| press(key))
        .chain(keys.iter().map(|key| release(key)))
        .collect()
}

fn press(keycode: &KeyCode) -> u8 {
    KEY_MAP.get(keycode).unwrap().0
}

fn release(keycode: &KeyCode) -> u8 {
    KEY_MAP.get(keycode).unwrap().1
}
