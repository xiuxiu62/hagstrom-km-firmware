// 0x00 [X MS] [X LS] [Y MS] [Y LS] [SCROLL]

#[repr(u8)]
#[derive(PartialEq)]
pub enum ScrollDirection {
    Up = 0x80,
    Down = 0x00,
}

#[repr(u8)]
#[derive(PartialEq)]
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
#[derive(PartialEq)]
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
            Self::Move(x, y) => Self::create_packet(
                x,
                y,
                Self::create_scroll_byte(
                    ScrollDirection::Up,
                    ScrollMagnitude::Zero,
                    false,
                    false,
                    false,
                ),
            ),
            Self::LeftClick | Self::MiddleClick | Self::RightClick => Self::click(self),
            Self::Scroll(direction, magnitude) => Self::create_packet(
                0,
                0,
                Self::create_scroll_byte(direction, magnitude, false, false, false),
            ),
        }
    }

    fn click(direction: Self) -> Vec<u8> {
        Self::create_packet(
            0,
            0,
            Self::create_scroll_byte(
                ScrollDirection::Up,
                ScrollMagnitude::Zero,
                direction == Self::LeftClick,
                direction == Self::MiddleClick,
                direction == Self::RightClick,
            ),
        )
        .into_iter()
        .chain(
            Self::create_packet(
                0,
                0,
                Self::create_scroll_byte(
                    ScrollDirection::Up,
                    ScrollMagnitude::Zero,
                    false,
                    false,
                    false,
                ),
            )
            .into_iter(),
        )
        .collect()
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
            | ((middle as u8) << 2)
            | ((right as u8) << 1)
            | (left as u8)
    }

    fn create_packet(x: u16, y: u16, scroll: u8) -> Vec<u8> {
        let split_point = |point: u16| -> (u8, u8) { ((point >> 8) as u8, point as u8) };
        let (xms, xls) = split_point(x);
        let (yms, yls) = split_point(y);

        vec![0x00, xms, xls, yms, yls, scroll]
    }
}
