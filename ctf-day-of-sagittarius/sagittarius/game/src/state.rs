use serde::{Deserialize, Serialize};

use crate::{types::Position, BOARD_SIZE, NUM_SHIPS, SHIP_SPANS};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ShipDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ship {
    pub pos: Position,
    pub dir: ShipDirection,
    pub hit_mask: u8,
}

impl Ship {
    pub fn new(x: u32, y: u32, dir: ShipDirection) -> Self {
        Ship {
            pos: Position::new(x, y),
            dir,
            hit_mask: 0,
        }
    }

    pub fn with_hit_mask(x: u32, y: u32, dir: ShipDirection, hit_mask: u8) -> Self {
        Ship {
            pos: Position::new(x, y),
            dir,
            hit_mask,
        }
    }

    pub fn check(&self, span: usize) -> bool {
        let limit = BOARD_SIZE - span;
        self.hit_mask == 0
            && self.pos.check()
            && match self.dir {
                ShipDirection::Horizontal => self.pos.x as usize <= limit,
                ShipDirection::Vertical => self.pos.y as usize <= limit,
            }
    }
}

pub struct GameCheck {
    board: [[bool; BOARD_SIZE]; BOARD_SIZE],
}

impl GameCheck {
    pub fn new() -> Self {
        let board: [[bool; BOARD_SIZE]; BOARD_SIZE] = [[false; BOARD_SIZE]; BOARD_SIZE];
        GameCheck { board }
    }

    pub fn check(&mut self, ship: &Ship, span: usize, commit: bool) -> bool {
        let x = ship.pos.x as usize;
        let y = ship.pos.y as usize;
        for i in 0..span {
            match ship.dir {
                ShipDirection::Horizontal => {
                    if self.board[y][x + i] {
                        return false;
                    }
                    if commit {
                        self.board[y][x + i] = true;
                    }
                }
                ShipDirection::Vertical => {
                    if self.board[y + i][x] {
                        return false;
                    }
                    if commit {
                        self.board[y + i][x] = true;
                    }
                }
            }
        }
        true
    }

    pub fn commit(&mut self, ship: &Ship, span: usize) {
        let x = ship.pos.x as usize;
        let y = ship.pos.y as usize;
        for i in 0..span {
            match ship.dir {
                ShipDirection::Horizontal => {
                    self.board[y][x + i] = true;
                }
                ShipDirection::Vertical => {
                    self.board[y + i][x] = true;
                }
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GameState {
    pub ships: [Ship; NUM_SHIPS],
    pub salt: u32,
}

impl GameState {
    pub fn check(&self) -> bool {
        let mut game_check = GameCheck::new();
        for i in 0..NUM_SHIPS {
            let ship = &self.ships[i];
            let span = SHIP_SPANS[i];
            if !ship.check(span) {
                return false;
            }
            if !game_check.check(&ship, span, true) {
                return false;
            }
        }
        true
    }
}