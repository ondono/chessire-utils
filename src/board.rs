/***
*** Provides a "board" struct used to represent the state of the board.
*** This state is not designed to be memory efficient or practical for search and evaluation
*** It's only purpose is to simplify the design of interfaces for GUIs or other programs

*** It also provides the main representations of the board for debugging.
***/

use anyhow::*;

use super::color::Color::{self, White};
use super::piece::Piece;
use std::fmt;

/*********
** Tile **
**********/

pub type Tile = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coord {
    file: usize,
    rank: usize,
}

#[inline(always)]
fn to_char(num: usize) -> char {
    (num as u8 + b'a').to_ascii_lowercase() as char
}

impl Coord {
    pub fn new(file: usize, rank: usize) -> Self {
        Self { file, rank }
    }
    pub fn from_tile(t: Tile) -> Self {
        Self {
            file: t % 8,
            rank: t / 8,
        }
    }
    pub fn to_usize(&self) -> usize {
        self.file + self.rank * 8
    }
    pub fn next_up(&self) -> Option<Self> {
        if self.rank < 7 {
            Some(Self::new(self.file, self.rank + 1))
        } else {
            None
        }
    }
    pub fn next_down(&self) -> Option<Self> {
        if self.rank > 0 {
            Some(Self::new(self.file, self.rank - 1))
        } else {
            None
        }
    }
    pub fn next_left(&self) -> Option<Self> {
        if self.file > 0 {
            Some(Self::new(self.file - 1, self.rank))
        } else {
            None
        }
    }
    pub fn next_right(&self) -> Option<Self> {
        if self.file < 7 {
            Some(Self::new(self.file + 1, self.rank))
        } else {
            None
        }
    }
}

use std::str::FromStr;

impl FromStr for Coord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        let file = (s.chars().nth(0).unwrap().to_ascii_lowercase() as u8) - b'a';
        let rank = (s.chars().nth(1).unwrap().to_ascii_lowercase() as u8) - b'0' - 1;

        let coord = Coord::new(file.into(), rank.into());
        Ok(coord)
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", to_char(self.file), self.rank + 1)
    }
}

#[cfg(test)]
mod coord_tests {
    use crate::*;

    #[test]
    fn test_parsing() {
        let valid_coords = ["a1", "b2", "c3", "d4", "e5", "f6", "g7", "h8"];

        for c in valid_coords {
            assert_eq!(c, c.parse::<Coord>().unwrap().to_string())
        }
    }

    #[test]
    fn test_up() {
        let valid_names_up = ["a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8"];

        // direct instantiation
        for (i, name) in valid_names_up.iter().enumerate() {
            let coord = Coord::new(0, i);
            assert_eq!(coord.to_string(), *name);
        }
        // through next_up
        let mut coord = Coord::new(0, 0);
        let mut index = 0;
        loop {
            coord = if coord.next_up().is_some() {
                assert_eq!(coord.to_string(), valid_names_up[index]);
                println!("{}", coord);
                index += 1;
                coord.next_up().unwrap()
            } else {
                break;
            }
        }
        // test upper bound
        for i in 0..8 {
            let coord = Coord::new(i, 8);
            assert_eq!(coord.next_up(), None);
        }
    }
    #[test]
    fn test_right() {
        let valid_names_right = ["a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"];

        // direct instantiation
        for (i, name) in valid_names_right.iter().enumerate() {
            let coord = Coord::new(i, 7);
            assert_eq!(coord.to_string(), *name);
        }
        // through next_up
        let mut coord = Coord::new(0, 7);
        let mut index = 0;
        loop {
            coord = if coord.next_right().is_some() {
                assert_eq!(coord.to_string(), valid_names_right[index]);
                println!("{}", coord);
                index += 1;
                coord.next_right().unwrap()
            } else {
                break;
            }
        }
        // test right bound
        for i in 0..8 {
            let coord = Coord::new(8, i);
            assert_eq!(coord.next_right(), None);
        }
    }

    #[test]
    fn test_down() {
        let valid_names_down = ["h8", "h7", "h6", "h5", "h4", "h3", "h2", "h1"];

        // direct instantiation
        for (i, name) in valid_names_down.iter().rev().enumerate() {
            let coord = Coord::new(7, i);
            assert_eq!(coord.to_string(), *name);
        }
        // through next_up
        let mut coord = Coord::new(0, 0);
        let mut index = 0;
        loop {
            coord = if coord.next_down().is_some() {
                assert_eq!(coord.to_string(), valid_names_down[index]);
                println!("{}", coord);
                index += 1;
                coord.next_down().unwrap()
            } else {
                break;
            }
        }
        // test down bound
        for i in 0..8 {
            let coord = Coord::new(i, 0);
            assert_eq!(coord.next_down(), None);
        }
    }
    #[test]
    fn test_left() {
        let valid_names_left = ["h1", "g1", "f1", "e1", "d1", "c1", "b1", "a1"];

        // direct instantiation
        for (i, name) in valid_names_left.iter().rev().enumerate() {
            let coord = Coord::new(i, 0);
            assert_eq!(coord.to_string(), *name);
        }
        // through next_up
        let mut coord = Coord::new(0, 0);
        let mut index = 0;
        loop {
            coord = if coord.next_left().is_some() {
                assert_eq!(coord.to_string(), valid_names_left[index]);
                println!("{}", coord);
                index += 1;
                coord.next_left().unwrap()
            } else {
                break;
            }
        }
        // test left bound
        for i in 0..8 {
            let coord = Coord::new(0, i);
            assert_eq!(coord.next_left(), None);
        }
    }
}
/***************
** Selections **
****************/

#[derive(Clone, Copy, Debug)]
pub struct SelectionColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl SelectionColor {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

#[derive(Clone, Debug)]
pub struct Selection {
    squares: Vec<Tile>,
    color: SelectionColor,
}

impl fmt::Display for Selection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Selection {:?}:",
            termion::color::Rgb(self.color.red, self.color.green, self.color.blue)
        )?;
        for sq in &self.squares {
            write!(f, "{}", sq)?;
        }
        fmt::Result::Ok(())
    }
}

impl Selection {
    pub fn new(squares: Vec<Tile>, color: SelectionColor) -> Self {
        Self { squares, color }
    }
}

/**********
** Board **
***********/

#[derive(Clone, Debug)]
pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub selections: Vec<Selection>,
    pub perspective: Color,
}

use core::ops::{Index, IndexMut};

impl Index<Tile> for Board {
    type Output = Option<Piece>;

    fn index(&self, t: Tile) -> &Self::Output {
        &self.squares[t]
    }
}

impl IndexMut<Tile> for Board {
    fn index_mut(&mut self, t: Tile) -> &mut Self::Output {
        &mut self.squares[t]
    }
}

const DEFAULT_PIECE_PLACEMENT: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

impl Default for Board {
    fn default() -> Self {
        let mut ret = Self {
            squares: [None; 64],
            selections: vec![],
            perspective: White,
        };
        // DEFAULT_PIECE_PLACEMENT should never fail
        ret.set_position_from_fen(DEFAULT_PIECE_PLACEMENT).unwrap();
        ret
    }
}

impl Index<Coord> for Board {
    type Output = Option<Piece>;

    fn index(&self, t: Coord) -> &Self::Output {
        &self.squares[t.to_usize()]
    }
}
impl IndexMut<Coord> for Board {
    fn index_mut(&mut self, t: Coord) -> &mut Self::Output {
        &mut self.squares[t.to_usize()]
    }
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_position_from_fen(&mut self, piece_placement: &str) -> Result<(), anyhow::Error> {
        let blocks = piece_placement.split('/');

        if blocks.clone().count() != 8 {
            return Err(anyhow!(""));
        }

        for (i, rank_string) in blocks.enumerate() {
            let rank = 7 - i;
            let mut file = 0;

            for c in rank_string.chars() {
                if c.is_ascii_digit() {
                    let space = c.to_digit(10).unwrap() as usize;
                    file += space;
                } else {
                    // set a piece
                    let coord = Coord::new(file, rank);
                    self[coord] = Piece::new_from_fen_char(c);
                    file += 1;
                }
            }
        }
        Ok(())
    }
    pub fn clear(&mut self) {
        self.squares = [None; 64];
        self.selections.clear();
        self.perspective = White;
    }
    pub fn add_selection(&mut self, sel: Selection) {
        self.selections.push(sel);
    }
    pub fn clear_selections(&mut self) {
        self.selections.clear();
    }
}
#[cfg(test)]
mod tests {
    use crate::board::*;
    #[test]
    fn test_clear_board() {
        let mut b = Board::new();
        b.clear();
        for sq in b.squares {
            assert!(sq == None);
        }
    }
    // test fen strings
}

use termion::color;
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // print the board from white's perspective
        let rank_range = if self.perspective == Color::White {
            (0..8).rev().collect::<Vec<usize>>()
        } else {
            (0..8).collect::<Vec<usize>>()
        };
        write!(
            f,
            "{}{}\r\n    A  B  C  D  E  F  G  H\r\n",
            color::Fg(color::White),
            color::Bg(color::Reset),
        )?;
        for rank in rank_range {
            // at the start of the rank, set the rank name
            write!(
                f,
                "{}{} {} ",
                color::Fg(color::White),
                color::Bg(color::Reset),
                rank + 1
            )?;
            for file in 0..8 {
                let coord = Coord::new(file, rank);
                let sq = self[coord];
                let mut tile_color =
                    // this sets the tile white or black
                    if (file + rank) & 0x01 == 1 {
                        color::Rgb(200, 200, 200)
                    } else {
                        color::Rgb(100, 100, 100)
                    };

                for sel in &self.selections {
                    for selected_square in &sel.squares {
                        if coord.to_usize() == *selected_square {
                            if (file + rank) & 0x01 == 1 {
                                tile_color = color::Rgb(
                                    ((sel.color.red as u16 + 200) / 2) as u8,
                                    ((sel.color.green as u16 + 200) / 2) as u8,
                                    ((sel.color.blue as u16 + 200) / 2) as u8,
                                );
                            } else {
                                tile_color = color::Rgb(
                                    ((sel.color.red as u16 + 100) / 2) as u8,
                                    ((sel.color.green as u16 + 100) / 2) as u8,
                                    ((sel.color.blue as u16 + 100) / 2) as u8,
                                );
                            }
                        }
                    }
                }

                match sq {
                    Some(piece) => write!(f, "{} {} ", color::Bg(tile_color), piece)?,
                    _ => write!(f, "{}   ", color::Bg(tile_color))?,
                };
            }
            //end of line
            write!(f, "{}\r\n", color::Bg(color::Reset))?;
        }
        // add an empty line and clear all styling
        write!(
            f,
            "{}{}\r\n",
            color::Fg(color::Reset),
            color::Bg(color::Reset)
        )
    }
}
