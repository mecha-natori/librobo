use alloc::boxed::Box;
use core::{
    error::Error,
    fmt::{Display, Formatter}
};

const UP: u16 = 0x0001;
const DOWN: u16 = 0x0002;
const RIGHT: u16 = 0x0004;
const LEFT: u16 = 0x0008;
const TRIANGLE: u16 = 0x0010;
const CROSS: u16 = 0x0020;
const CIRCLE: u16 = 0x0040;
const START: u16 = 0x0080;
const SQUARE: u16 = 0x0100;
const L1: u16 = 0x0200;
const L2: u16 = 0x0400;
const R1: u16 = 0x0800;
const R2: u16 = 0x1000;
const SELECT: u16 = 0x2000;
const L3: u16 = 0x4000;
const R3: u16 = 0x8000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonState {
    Push,
    Pushing,
    Release,
    Released
}

#[derive(Debug, Eq, PartialEq)]
pub struct Buttons {
    pub up: ButtonState,
    pub down: ButtonState,
    pub right: ButtonState,
    pub left: ButtonState,
    pub triangle: ButtonState,
    pub cross: ButtonState,
    pub circle: ButtonState,
    pub start: ButtonState,
    pub square: ButtonState,
    pub l1: ButtonState,
    pub l2: ButtonState,
    pub r1: ButtonState,
    pub r2: ButtonState,
    pub select: ButtonState,
    pub l3: ButtonState,
    pub r3: ButtonState
}

impl Buttons {
    pub fn from_word(word: u16) -> Self {
        use ButtonState::{Push, Released};
        Self {
            up: if word & UP == 0 { Released } else { Push },
            down: if word & DOWN == 0 { Released } else { Push },
            right: if word & RIGHT == 0 { Released } else { Push },
            left: if word & LEFT == 0 { Released } else { Push },
            triangle: if word & TRIANGLE == 0 { Released } else { Push },
            cross: if word & CROSS == 0 { Released } else { Push },
            circle: if word & CIRCLE == 0 { Released } else { Push },
            start: if word & START == 0 { Released } else { Push },
            square: if word & SQUARE == 0 { Released } else { Push },
            l1: if word & L1 == 0 { Released } else { Push },
            l2: if word & L2 == 0 { Released } else { Push },
            r1: if word & R1 == 0 { Released } else { Push },
            r2: if word & R2 == 0 { Released } else { Push },
            select: if word & SELECT == 0 { Released } else { Push },
            l3: if word & L3 == 0 { Released } else { Push },
            r3: if word & R3 == 0 { Released } else { Push }
        }
    }
}

impl Default for Buttons {
    fn default() -> Self {
        use ButtonState::Released;
        Self {
            up: Released,
            down: Released,
            right: Released,
            left: Released,
            triangle: Released,
            cross: Released,
            circle: Released,
            start: Released,
            square: Released,
            l1: Released,
            l2: Released,
            r1: Released,
            r2: Released,
            select: Released,
            l3: Released,
            r3: Released
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Sticks(pub (i8, i8), pub (i8, i8));

#[derive(Clone, Debug)]
pub struct InvalidHeaderError;

impl Display for InvalidHeaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid sbdbt header")
    }
}

impl Error for InvalidHeaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Clone, Debug)]
pub struct ChecksumError;

impl Display for ChecksumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "checksum mismatched")
    }
}

impl Error for ChecksumError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Default)]
pub struct Sbdbt {
    buttons: Buttons,
    sticks: Sticks,
    last_buttons: Buttons
}

impl Sbdbt {
    pub fn get_buttons(&self) -> &Buttons {
        &self.buttons
    }

    pub fn get_sticks(&self) -> &Sticks {
        &self.sticks
    }

    pub fn update_buttons(&mut self, data: [u8; 8]) -> Result<(), Box<dyn Error + 'static>> {
        Self::check_data(data)?;
        self.sticks.0.0 = (data[3] - 0b0100_0000) as i8;
        self.sticks.0.1 = (data[4] - 0b0100_0000) as i8;
        self.sticks.1.0 = (data[5] - 0b0100_0000) as i8;
        self.sticks.1.1 = (data[6] - 0b0100_0000) as i8;
        let word = (data[1] as u16) << 8 | data[0] as u16;
        self.buttons.up = Self::update_button(self.last_buttons.up, (word & UP) != 0);
        self.buttons.down = Self::update_button(self.last_buttons.down, (word & DOWN) != 0);
        self.buttons.right = Self::update_button(self.last_buttons.right, (word & RIGHT) != 0);
        self.buttons.left = Self::update_button(self.last_buttons.left, (word & LEFT) != 0);
        self.buttons.triangle =
            Self::update_button(self.last_buttons.triangle, (word & TRIANGLE) != 0);
        self.buttons.cross = Self::update_button(self.last_buttons.cross, (word & CROSS) != 0);
        self.buttons.circle = Self::update_button(self.last_buttons.circle, (word & CIRCLE) != 0);
        self.buttons.start = Self::update_button(self.last_buttons.start, (word & START) != 0);
        self.buttons.square = Self::update_button(self.last_buttons.square, (word & SQUARE) != 0);
        self.buttons.l1 = Self::update_button(self.last_buttons.l1, (word & L1) != 0);
        self.buttons.l2 = Self::update_button(self.last_buttons.l2, (word & L2) != 0);
        self.buttons.r1 = Self::update_button(self.last_buttons.r1, (word & R1) != 0);
        self.buttons.r2 = Self::update_button(self.last_buttons.r2, (word & R2) != 0);
        self.buttons.select = Self::update_button(self.last_buttons.select, (word & SELECT) != 0);
        self.buttons.l3 = Self::update_button(self.last_buttons.l3, (word & L3) != 0);
        self.buttons.r3 = Self::update_button(self.last_buttons.r3, (word & R3) != 0);
        Ok(())
    }

    fn check_data(data: [u8; 8]) -> Result<(), Box<dyn Error + 'static>> {
        if data[0] != 0x80 {
            return Err(Box::new(InvalidHeaderError));
        }
        if data[7] != data[1..=6].iter().sum::<u8>() & 0b0111_1111 {
            return Err(Box::new(ChecksumError));
        }
        Ok(())
    }

    fn update_button(state: ButtonState, is_push: bool) -> ButtonState {
        if state == ButtonState::Pushing || state == ButtonState::Push {
            return if is_push {
                ButtonState::Pushing
            } else {
                ButtonState::Release
            };
        }
        if (state == ButtonState::Released || state == ButtonState::Release) && is_push {
            return ButtonState::Push;
        }
        ButtonState::Released
    }
}
