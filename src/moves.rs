use super::board::*;
use super::piece::Piece::*;
use super::piece::*;
use crate::color::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Move {
    pub source: Coord,
    pub target: Coord,
    pub piece: Piece,
    pub promoted_piece: Option<Piece>,
    pub capture: bool,
    pub double_push: bool,
    pub enpassant: bool,
    pub castling: bool,
}

impl Move {
    pub fn new(source: Coord, target: Coord, piece: Piece, promoted_piece: Option<Piece>) -> Self {
        Self {
            source,
            target,
            piece,
            promoted_piece,
            capture: false,
            castling: false,
            double_push: false,
            enpassant: false,
        }
    }
    pub fn capture(&mut self, capture: bool) -> Self {
        self.capture = capture;
        *self
    }
    pub fn castling(&mut self, castling: bool) -> Self {
        self.castling = castling;
        *self
    }
    pub fn double_push(&mut self, double_push: bool) -> Self {
        self.double_push = double_push;
        *self
    }
    pub fn enpassant(&mut self, enpassant: bool) -> Self {
        self.enpassant = enpassant;
        *self
    }
    // specific intialitzers for comfort
    pub fn new_pawn_double_push(color: Color, source: Coord) -> Self {
        Self::new(
            source,
            //TODO: this is now saturating, just in case. Find a better fix
            if color == Color::White {
                source
                    .next_up()
                    .unwrap_or(source)
                    .next_up()
                    .unwrap_or(source)
            } else {
                source
                    .next_down()
                    .unwrap_or(source)
                    .next_down()
                    .unwrap_or(source)
            },
            Pawn(color),
            None,
        )
        .double_push(true)
        // this is not necesary, but let's leave it for now
        .capture(false)
        .castling(false)
        .enpassant(false)
    }
    pub fn new_pawn_push(color: Color, source: Coord) -> Self {
        Self::new(
            source,
            //TODO: this is now saturating, just in case. Find a better fix
            if color == Color::White {
                source.next_up().unwrap_or(source)
            } else {
                source.next_down().unwrap_or(source)
            },
            Pawn(color),
            None,
        )
        // this is not necesary, but let's leave it for now
        .capture(false)
        .castling(false)
        .enpassant(false)
        .double_push(false)
    }
    pub fn new_promotion(color: Color, source: Coord, piece: Piece) -> Self {
        Self::new(
            source,
            //TODO: this is now saturating, just in case. Find a better fix
            if color == Color::White {
                source.next_up().unwrap_or(source)
            } else {
                source.next_down().unwrap_or(source)
            },
            Pawn(color),
            Some(piece),
        )
        // this is not necesary, but let's leave it for now
        .capture(false)
        .castling(false)
        .enpassant(false)
        .double_push(false)
    }
    pub fn new_knight_move(source: Coord, target: Coord, color: Color, capture: bool) -> Self {
        Move::new(source, target, Knight(color), None)
            .capture(capture)
            // this is not necesary, but let's leave it for now
            .castling(false)
            .enpassant(false)
            .double_push(false)
    }
    pub fn new_bishop_move(source: Coord, target: Coord, color: Color, capture: bool) -> Self {
        Move::new(source, target, Bishop(color), None)
            .capture(capture)
            // this is not necesary, but let's leave it for now
            .castling(false)
            .enpassant(false)
            .double_push(false)
    }
    pub fn new_rook_move(source: Coord, target: Coord, color: Color, capture: bool) -> Self {
        Move::new(source, target, Rook(color), None)
            .capture(capture)
            // this is not necesary, but let's leave it for now
            .double_push(false)
            .castling(false)
            .enpassant(false)
    }
    pub fn new_castling(source: Coord, target: Coord, color: Color) -> Self {
        Move::new(source, target, King(color), None)
            .capture(false)
            .double_push(false)
            .castling(true)
            .enpassant(false)
    }
    pub fn set_promotion(&mut self, prom: Option<Piece>) {
        self.promoted_piece = prom;
    }
}

pub fn print_movelist(movelist: &[Move]) {
    println!("move\tpiece\tprom.\tcapture\tdouble\tenpass.\tcastling\n\r");
    for m in movelist {
        println!("{}", m);
    }
}

use core::fmt::{Display, Formatter, Result};

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.promoted_piece.is_some() {
            write!(
                f,
                "{}{}\t{}\t{}\t{}\t{}\t{}\t{}",
                self.source,
                self.target,
                self.piece,
                self.promoted_piece.unwrap(),
                self.capture,
                self.double_push,
                self.enpassant,
                self.castling
            )
        } else {
            write!(
                f,
                "{}{}\t{}\tNone\t{}\t{}\t{}\t{}",
                self.source,
                self.target,
                self.piece,
                self.capture,
                self.double_push,
                self.enpassant,
                self.castling
            )
        }
    }
}

// MOVE RECORD
//

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct MoveRecord {
    pub name: String,
    pub count: u128,
}
