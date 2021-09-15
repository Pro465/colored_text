pub type ColorType = (u8, u8, u8);

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub foreground: ColorType,
    pub background: ColorType,
}

impl Color {
    pub fn set(&self) {
        use std::io::{stdout, Write};

        print!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m",
            self.foreground.0,
            self.foreground.1,
            self.foreground.2,
            self.background.0,
            self.background.1,
            self.background.2,
        );

        stdout().flush().unwrap();
    }
}

pub mod colors;
