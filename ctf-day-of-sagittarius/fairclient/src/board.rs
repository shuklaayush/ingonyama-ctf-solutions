use std::{collections::HashMap, io::{self, Write}};

use rand::{thread_rng, Rng};
use sagittarius_game::{types::{Position, HitType}, NUM_SHIPS, state::{Ship, GameCheck, ShipDirection, GameState}, BOARD_SIZE, SHIP_SPANS};

use crate::{interface::update_player_game_board, LOGGER};

pub type HitMap = HashMap<Position, HitType>;

fn create_random_ships() -> [Ship; NUM_SHIPS] {
    // randomly place 5 ships on the board
    let mut rng = thread_rng();
    let mut game_check = GameCheck::new();

    let ships: [Ship; NUM_SHIPS] = array_init::array_init(|i| {
        loop {
            // pick a random starting point on the board
            let x = rng.gen_range(0..BOARD_SIZE - 1);
            let y = rng.gen_range(0..BOARD_SIZE - 1);

            // pick between 0 and 1 for randomized ship placement
            let dir = if rng.gen::<bool>() {
                ShipDirection::Horizontal
            } else {
                ShipDirection::Vertical
            };

            let ship = Ship::new(x as u32, y as u32, dir);

            // does it fit on the board
            let span = SHIP_SPANS[i];
            if !ship.check(span) {
                continue;
            }

            // does it cross any other ship
            if !game_check.check(&ship, span, false) {
                continue;
            }

            // mark the ship as taken
            game_check.commit(&ship, span);

            return ship;
        }
    });
    ships
}

pub fn create_random_state() -> GameState {
    let ships = create_random_ships();
    GameState {
        ships,
        salt: rand::thread_rng().gen(),
    }
}

pub fn choose_board() -> GameState {
    let mut state = create_random_state();
    loop {
        update_player_game_board(&state);

        LOGGER.log("Are you satisfied with this board? (Y/n)");
        let mut input = String::new();
        print!(">: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        if input.trim() != "n" {
            break;           
        }
        state = create_random_state();
    };

    state
}

pub fn create_empty_shots() -> HitMap {
    HashMap::new()
}