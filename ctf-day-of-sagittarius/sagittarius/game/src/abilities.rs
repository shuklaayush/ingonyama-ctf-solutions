extern crate alloc;

use serde::{Deserialize, Serialize};

use crate::{types::{Position, Digest, HitType}, state::{GameState, ShipDirection}, NUM_SHIPS, SHIP_SPANS, SUNK_MASKS};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClusterBombParams {
    pub state: GameState,
    pub upper_left_coordinates: Position,
    pub down_right_coordinates: Position,
    pub seed: u8,
}

impl ClusterBombParams {
    pub fn new(state: GameState, ul: Position, dr: Position, seed: u8) -> Self {
        ClusterBombParams {
            state,
            upper_left_coordinates: ul,
            down_right_coordinates: dr,
            seed
        }
    }

    pub fn check(&self) -> usize {
        let width = self.down_right_coordinates.x - self.upper_left_coordinates.x + 1;
        let height = self.down_right_coordinates.y - self.upper_left_coordinates.y + 1;
        assert!(width > 0 && height > 0);

        match width * height {
            4 => 1,
            6..=8 => 2,
            9..=14 => 3,
            _ => panic!("The area is too big!")
        }
    }

    fn check_in_range(&self, position: &Position) {
        assert!(position.x >= self.upper_left_coordinates.x && position.x <= self.down_right_coordinates.x);
        assert!(position.y >= self.upper_left_coordinates.y && position.y <= self.down_right_coordinates.y);
    }

    pub fn process(&self, positions: &alloc::vec::Vec<Position>) -> ClusterResults {
        let mut state = self.state.clone();
        let mut hits = alloc::vec![];

        for pos in positions {
            self.check_in_range(&pos);

            let params = ShotParams::new(state.clone(), pos.x, pos.y);
            let result = params.process();
            state = result.state;
            hits.push(result.hit);
        }

        ClusterResults { state, hits }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClusterResults {
    pub state: GameState,
    pub hits: alloc::vec::Vec<HitType>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClusterCommit {
    pub old_state_digest: Digest,
    pub new_state_digest: Digest,
    pub config: ClusterBombParams,
    pub shots: alloc::vec::Vec<Position>,
    pub hits: alloc::vec::Vec<HitType>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScoutParams {
    pub state: GameState,
    pub shot: Position,
}

impl ScoutParams {
    pub fn process(&self) -> ScoutResult {
        let mut cells = [HitType::Miss, HitType::Miss, HitType::Miss, HitType::Miss];
        let positions = [Position::new(self.shot.x - 1, self.shot.y),
                         Position::new(self.shot.x, self.shot.y - 1),
                         Position::new(self.shot.x + 1, self.shot.y),
                         Position::new(self.shot.x, self.shot.y + 1)];

        for (pos, cell) in positions.iter().zip(cells.iter_mut()) {
            for i in 0..NUM_SHIPS {
                let ship = &self.state.ships[i];
                let span = SHIP_SPANS[i] as u32;
                let x = ship.pos.x;
                let y = ship.pos.y;
                let hit = match ship.dir {
                    ShipDirection::Horizontal => {
                        if pos.y == y && pos.x >= x && pos.x < x + span {
                            HitType::Revealed
                        } else {
                            HitType::Miss
                        }
                    }
                    ShipDirection::Vertical => {
                        if pos.x == x && pos.y >= y && pos.y < y + span {
                            HitType::Revealed
                        } else {
                            HitType::Miss
                        }
                    }
                };

                match hit {
                    HitType::Revealed => {
                        *cell = hit;
                        break;
                    },
                    _ => {},
                }
            }
        }

        ScoutResult {
            cells,
            shot: self.shot.clone()
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScoutResult {
    pub cells: [HitType; 4],
    pub shot: Position,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ShotParams {
    pub state: GameState,
    pub shot: Position,
}

impl ShotParams {
    pub fn new(state: GameState, x: u32, y: u32) -> Self {
        ShotParams {
            state,
            shot: Position::new(x, y),
        }
    }

    pub fn process(&self) -> ShotResult {
        let mut state = self.state.clone();
        let shot = &self.shot;
        assert!(shot.check());
        enum HitShift {
            Hit(u32),
            Miss,
        }
        for i in 0..NUM_SHIPS {
            let ship = &mut state.ships[i];
            let span = SHIP_SPANS[i] as u32;
            let x = ship.pos.x;
            let y = ship.pos.y;
            let hit_shift = match ship.dir {
                ShipDirection::Horizontal => {
                    if shot.y == y && shot.x >= x && shot.x < x + span {
                        HitShift::Hit(shot.x - x)
                    } else {
                        HitShift::Miss
                    }
                }
                ShipDirection::Vertical => {
                    if shot.x == x && shot.y >= y && shot.y < y + span {
                        HitShift::Hit(shot.y - y)
                    } else {
                        HitShift::Miss
                    }
                }
            };
            if let HitShift::Hit(shift) = hit_shift {
                ship.hit_mask |= 1 << shift;
                let hit = if ship.hit_mask == SUNK_MASKS[i] {
                    HitType::Destroyed(i as u8)
                } else {
                    HitType::Hit
                };
                return ShotResult::new(state, hit);
            }
        }
        ShotResult {
            state,
            hit: HitType::Miss,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ShotResult {
    pub state: GameState,
    pub hit: HitType,
}

impl ShotResult {
    pub fn new(state: GameState, hit: HitType) -> Self {
        ShotResult { state, hit }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ShotCommit {
    pub old_state_digest: Digest,
    pub new_state_digest: Digest,
    pub shot: Position,
    pub hit: HitType,
}