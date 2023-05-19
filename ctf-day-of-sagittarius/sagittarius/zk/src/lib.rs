use std::fs::File;
use std::io::Write;
use sagittarius_game::{state::{GameState, ShipDirection}, abilities::{ShotParams, ShotCommit, ClusterBombParams, ClusterCommit, ScoutParams, ScoutResult}, types::{Position, Digest, HitType}, BOARD_SIZE, SHIP_SPANS};
use sagittarius_methods::{INIT_ID, INIT_ELF, TURN_ID, TURN_ELF, CLUSTER_ID, CLUSTER_ELF, SCOUT_ELF, SCOUT_ID};
use risc0_zkvm::{Result, Receipt, serde, Prover};

fn empty_board() -> [[String; BOARD_SIZE]; BOARD_SIZE] {
    let mut board: [[String; BOARD_SIZE]; BOARD_SIZE] = Default::default();

    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            board[i][j] = "   ".to_string();
        }
    }

    board
}

fn render_player_cells(state: &GameState) -> [[String; BOARD_SIZE]; BOARD_SIZE] {
    let mut board = empty_board();

    for (i, ship) in state.ships.iter().enumerate() {
        let ship_symbol = format!(" {} ", (b'A' + i as u8) as char);
        for j in 0..SHIP_SPANS[i] {
            let (x, y) = match ship.dir {
                ShipDirection::Horizontal => (ship.pos.x as usize + j, ship.pos.y as usize),
                ShipDirection::Vertical => (ship.pos.x as usize, ship.pos.y as usize + j),
            };

            if x < BOARD_SIZE && y < BOARD_SIZE {
                if ship.hit_mask & (1 << j) != 0 {
                    board[y][x] = " X ".to_string();
                } else {
                    board[y][x] = ship_symbol.clone();
                }
            }
        }
    }

    board
}

fn create_board_representation(title: String, cells: [[String; BOARD_SIZE]; BOARD_SIZE]) -> Vec<String> {
    let mut result = Vec::new();

    // Calculate total width of the board
    let total_width = BOARD_SIZE * 4 + 4;  // 2 characters per cell + 1 for the vertical divider

    // Print title
    let padded_title = format!("{0: ^1$}", title, total_width - 2);  // Center the title with padding
    let title_frame = format!("┏{}┓", "━".repeat(total_width - 2));
    let title_frame_bot = format!("┣━━┳{}━━━┫", "━━━┳".repeat(BOARD_SIZE - 1));
    let title_line = format!("┃{}┃", padded_title);
    result.push(title_frame);
    result.push(title_line);
    result.push(title_frame_bot);

    // Print column letters
    let letters_line = (b'a'..=b'a' + (BOARD_SIZE - 1) as u8).map(char::from).map(|c| format!(" {} ┃", c)).collect::<String>();
    result.push(format!("┃  ┃{}", letters_line));

    // Print board
    for (i, row) in cells.iter().enumerate() {
        let row_line = format!("┃ {}┃{}┃", i + 1, row.join("│"));
        result.push("┣━━╋".to_string() + &"───┼".repeat(BOARD_SIZE - 1) + "───┨");
        result.push(row_line);
    }

    // Add bottom border
    result.push("┗━━┷".to_string() + &"━━━┷".repeat(BOARD_SIZE - 1) + "━━━┛");

    result
}

// Init

pub fn create_init_proof(input: &GameState) -> Result<Receipt> {
    let mut prover = Prover::new(&INIT_ELF)?;
    let vec = serde::to_vec(&input).unwrap();
    prover.add_input_u32_slice(vec.as_slice());
    prover.run()
}

pub fn check_init_proof(receipt: &Receipt) -> Result<()> {
    receipt.verify(&INIT_ID)
}

// Simple shot proof

pub fn create_turn_proof(input: &ShotParams) -> Result<Receipt> {
    let mut prover = Prover::new(&TURN_ELF)?;
    let vec = serde::to_vec(&input).unwrap();
    prover.add_input_u32_slice(vec.as_slice());
    prover.run()
}

pub fn check_turn_proof(receipt: Receipt, shot: &Position, old_state: &Digest) -> Result<(HitType, Digest)> {
    receipt.verify(&TURN_ID)?;
    let journal = receipt.get_journal_bytes(); 
    let commit = serde::from_slice::<ShotCommit, u8>(&journal).unwrap();
    // Make sure the prior state matches the current state
    assert!(old_state == &commit.old_state_digest);
    // Make sure the response matches the prior shot
    assert!(commit.shot == shot.clone());
    
    Ok((commit.hit, commit.new_state_digest))
}

// Scout
pub fn create_scout_proof(input: &ScoutParams) -> Result<Receipt> {
    let mut prover = Prover::new(&SCOUT_ELF)?;
    let vec = serde::to_vec(&input).unwrap();
    prover.add_input_u32_slice(vec.as_slice());
    prover.run()
}

pub fn check_scout_proof(receipt: Receipt, shot: &Position) -> Result<Vec<HitType>> {
    receipt.verify(&SCOUT_ID)?;
    let journal = receipt.get_journal_bytes(); 
    let results = serde::from_slice::<ScoutResult, u8>(&journal).unwrap();

    assert!(&results.shot == shot);
    Ok(results.cells.to_vec())
}

// Cluster bomb proof

pub fn create_cluster_proof(input: &ClusterBombParams) -> Result<(Receipt, Vec<Position>)> {
    let mut prover = Prover::new(&CLUSTER_ELF)?;
    let vec = serde::to_vec(&input).unwrap();
    prover.add_input_u32_slice(vec.as_slice());
    let receipt = prover.run()?;

    let journal = receipt.get_journal_bytes(); 
    let commit = serde::from_slice::<ClusterCommit, u8>(&journal).unwrap();

    Ok((receipt, commit.shots))
}

pub fn check_cluster_proof(receipt: Receipt, ul: Position, dr: Position, seed: u8, old_state: Digest) -> Result<(Vec<Position>, Vec<HitType>, Digest)> {
    receipt.verify(&CLUSTER_ID)?;
    let journal = receipt.get_journal_bytes(); 
    let commit = serde::from_slice::<ClusterCommit, u8>(&journal).unwrap();
    // Make sure the prior state matches the current state
    assert!(old_state == commit.old_state_digest);
    // Make sure the response matches the Cluster config
    assert!(commit.config.upper_left_coordinates == ul);
    assert!(commit.config.down_right_coordinates == dr);
    assert!(commit.config.seed == seed);

    let path = "enemy.txt";
    let mut output = File::create(path)?;
    let player_board = create_board_representation("Enemy Board".to_string(), render_player_cells(&commit.config.state));
    for player_line in player_board.iter() {
        writeln!(output, "{}", player_line)?;
    }
    
    Ok((commit.shots, commit.hits, commit.new_state_digest))
}