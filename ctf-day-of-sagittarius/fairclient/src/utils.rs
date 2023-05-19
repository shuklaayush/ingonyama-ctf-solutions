use crate::{board::HitMap, LOGGER};

use sagittarius_game::types::{Position, HitType};

use std::io::Write;

pub fn get_coords_from_stdin(hit_map: Option<&HitMap>) -> Position {
    loop {
        let mut input = String::new();
        print!("Enter coordinates (col row): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        let mut parts = input.trim().split_whitespace();

        let col_part = parts.next();
        let row_part = parts.next();

        if col_part.is_none() || row_part.is_none() {
            LOGGER.log("Please provide exactly 2 values (column letter and row number).");
            continue;
        }

        let col_str = col_part.unwrap();
        let row_str = row_part.unwrap();

        if col_str.len() != 1 || !col_str.chars().all(char::is_alphabetic) {
            LOGGER.log("Column should be a single letter.");
            continue;
        }

        let row: u32 = match row_str.parse() {
            Ok(n) => n,
            Err(_) => {
                LOGGER.log("Row should be a number.");
                continue;
            }
        };

        let x = col_str.chars().next().unwrap().to_ascii_lowercase() as u32 - 'a' as u32;
        let y = row - 1;

        let pos = Position::new(x, y);

        if !pos.check() {
            LOGGER.log("Coords are out of board");
            continue;
        }

        if let Some(hit_map) = hit_map {
            if let Some(hit) = hit_map.get(&pos) {
                match hit {
                    HitType::Revealed => {
                        return pos;
                    },
                    _ => {
                        LOGGER.log("Hit exists");
                        continue;
                    }
                }
            }
        }
        return pos;
    }
}