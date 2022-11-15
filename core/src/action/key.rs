use crate::action::{key_map::KEY_MAP, KeyCode};

#[macro_export]
macro_rules! message {
    ($data:expr) => {
        $crate::action::key::create_message($data)
    };
}

#[macro_export]
macro_rules! command {
    ($($key_code: expr),*) => {{
        $crate::action::key::create_command(vec![$($key_code),*])
    }};
}

macro_rules! key {
    ($($key_code: expr),*) => {{
        create_text(vec![$($key_code),*])
    }};
}
pub(crate) use key;

macro_rules! shift {
    ($($key_code: expr),*) => {{
        command![$(KeyCode::Shift, $key_code),*]
    }};
}
pub(crate) use shift;

pub fn create_message(data: &str) -> Vec<u8> {
    data.chars()
        .flat_map(|char| character_packet(char).unwrap())
        .collect()
}

pub fn create_command(keys: Vec<KeyCode>) -> Vec<u8> {
    // let presses: Vec<u8> = keys.iter().map(press).collect();
    // keys.reverse();

    keys.iter()
        .map(press)
        .chain(keys.iter().rev().map(release))
        .collect()

    // presses
    //     .into_iter()
    //     .chain(keys.iter().map(release))
    //     .collect()
}

fn create_text(keys: Vec<KeyCode>) -> Vec<u8> {
    keys.iter()
        .flat_map(|keycode| vec![press(keycode), release(keycode)])
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

fn press(keycode: &KeyCode) -> u8 {
    KEY_MAP.get(keycode).unwrap().0
}

fn release(keycode: &KeyCode) -> u8 {
    KEY_MAP.get(keycode).unwrap().1
}
