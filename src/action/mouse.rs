// 0x00 [X MS] [X LS] [Y MS] [Y LS] [SCROLL]

pub enum MouseAction {
    Move(u16, u16),
    LeftClick,
    MiddleClick,
    RightClick,
    Scroll(ScrollDirection, ScrollMagnitude),
}

impl MouseAction {
    pub fn as_packet(self) -> Vec<u8> {
        match self {
            Self::Move(x, y) => create_packet(
                x,
                y,
                create_scroll_byte(
                    ScrollDirection::Up,
                    ScrollMagnitude::Zero,
                    false,
                    false,
                    false,
                ),
            ),
            Self::LeftClick => create_packet(
                0,
                0,
                create_scroll_byte(
                    ScrollDirection::Up,
                    ScrollMagnitude::Zero,
                    true,
                    false,
                    false,
                ),
            )
            .into_iter()
            .chain(
                create_packet(
                    0,
                    0,
                    create_scroll_byte(
                        ScrollDirection::Up,
                        ScrollMagnitude::Zero,
                        false,
                        false,
                        false,
                    ),
                )
                .into_iter(),
            )
            .collect(),
            Self::MiddleClick => create_packet(
                0,
                0,
                create_scroll_byte(
                    ScrollDirection::Up,
                    ScrollMagnitude::Zero,
                    false,
                    true,
                    false,
                ),
            )
            .into_iter()
            .chain(
                create_packet(
                    0,
                    0,
                    create_scroll_byte(
                        ScrollDirection::Up,
                        ScrollMagnitude::Zero,
                        false,
                        false,
                        false,
                    ),
                )
                .into_iter(),
            )
            .collect(),
            Self::RightClick => create_packet(
                0,
                0,
                create_scroll_byte(
                    ScrollDirection::Up,
                    ScrollMagnitude::Zero,
                    false,
                    false,
                    true,
                ),
            )
            .into_iter()
            .chain(
                create_packet(
                    0,
                    0,
                    create_scroll_byte(
                        ScrollDirection::Up,
                        ScrollMagnitude::Zero,
                        false,
                        false,
                        false,
                    ),
                )
                .into_iter(),
            )
            .collect(),
            Self::Scroll(direction, magnitude) => create_packet(
                0,
                0,
                create_scroll_byte(direction, magnitude, false, false, false),
            ),
        }
    }
}

fn create_packet(/* action: MouseAction, */ x: u16, y: u16, scroll: u8) -> Vec<u8> {
    let split_point = |point: u16| -> (u8, u8) { ((point >> 8) as u8, point as u8) };
    let (xms, xls) = split_point(x);
    let (yms, yls) = split_point(y);
    // vec![0x00].into_iter().chain()

    vec![0x00, xms, xls, yms, yls, scroll]
}

#[repr(u8)]
pub enum ScrollDirection {
    Up = 0x80,
    Down = 0x00,
}

#[repr(u8)]
pub enum ScrollMagnitude {
    Seven = 0x70,
    Six = 0x60,
    Five = 0x50,
    Four = 0x40,
    Three = 0x30,
    Two = 0x20,
    One = 0x10,
    Zero = 0x00,
}

fn create_scroll_byte(
    direction: ScrollDirection,
    magnitude: ScrollMagnitude,
    left: bool,
    middle: bool,
    right: bool,
) -> u8 {
    (direction as u8)
        | (magnitude as u8)
        | 0b00001000
        | ((left as u8) << 2)
        | ((middle as u8) << 1)
        | (right as u8)
}
