use termion::color;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        let (letter, col) = if *self == Color::White {
            ("W", color::Rgb(255, 255, 255))
        } else {
            ("B", color::Rgb(0, 0, 0))
        };
        write!(f, "{}{}{}", color::Fg(col), letter, color::Fg(color::Reset))
    }
}

use Color::*;

impl Color {
    pub fn opponent(self) -> Color {
        if self == White {
            Black
        } else {
            White
        }
    }
}

use std::ops::Index;
use std::ops::IndexMut;

impl<T> Index<Color> for [T] {
    type Output = T;

    fn index(&self, color: Color) -> &Self::Output {
        &self[color as usize]
    }
}

impl<T> IndexMut<Color> for [T] {
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        &mut self[color as usize]
    }
}
