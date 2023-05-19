use sagittarius_game::{state::{GameState, ShipDirection}, BOARD_SIZE, SHIP_SPANS, types::HitType};
use crossterm::{execute, terminal, terminal::Clear, terminal::ClearType};
use crossterm::style::{Colored, Stylize};
use std::{io::{stdout}};
use lazy_static::lazy_static;
use std::sync::Mutex;
use rand::Rng;
use std::time::Duration;
use std::thread;

use crate::{board::HitMap, LOGGER};

lazy_static! {
    static ref PLAYER_BOARD: Mutex<Vec<String>> = Mutex::new(vec![String::default(); (BOARD_SIZE + 1) * 2 + 3]);
    static ref ENEMY_BOARD: Mutex<Vec<String>> = Mutex::new(vec![String::default(); (BOARD_SIZE + 1) * 2 + 3]);
}

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

fn render_enemy_cells(shots: &HitMap) -> [[String; BOARD_SIZE]; BOARD_SIZE] {
    let mut board = empty_board();

    for (pos, hit) in shots {
        if pos.x < BOARD_SIZE as u32 && pos.y < BOARD_SIZE as u32 {
            board[pos.y as usize][pos.x as usize] = match hit {
                HitType::Miss => " M ".to_string(),
                HitType::Hit => " H ".to_string(),
                HitType::Revealed => " R ".to_string(),
                HitType::Destroyed(_) => " S ".to_string(),
            };
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

pub fn draw_night_sky(scroll: Option<u16>, sleep: Option<u32>) {
    let symbols = vec!['⭐', '🌟', '✨', '*'];
    let mut rng = rand::thread_rng();

    let (width, height) = terminal::size().unwrap();

    for _ in 0..height * scroll.unwrap_or(1) {
        for _ in 0..width {
            let symbol = rng.gen_range(0..symbols.len());
            let prob = rng.gen_range(0..100);

            // To create a realistic night sky, we will only draw a star 10% of the time
            if prob < 3 {
                print!("{}", symbols[symbol]);
            } else {
                print!(" ");
            }
        }
        println!();
        if let Some(sleep) = sleep {
            thread::sleep(Duration::from_millis(sleep as u64));
        }
    }
}

pub fn update_player_game_board(player_state: &GameState) {
    {
        let mut player_board = PLAYER_BOARD.lock().unwrap();
        *player_board = create_board_representation("Player".to_string(), render_player_cells(player_state));
    }
    update_hud();
}

pub fn update_enemy_game_board(enemy_shots: &HitMap) {
    {
        let mut enemy_board = ENEMY_BOARD.lock().unwrap();
        *enemy_board = create_board_representation("Enemy".to_string(), render_enemy_cells(enemy_shots));
    }
    update_hud();
}

pub fn update_hud() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();

    // draw_night_sky(None, None);
    println!("{}", DAY);
    println!("{}", OF);
    println!("{}", SAGITTARIUS);

    let player_board = PLAYER_BOARD.lock().unwrap();
    let enemy_board = ENEMY_BOARD.lock().unwrap();

    let mut log_box = vec![];
    // Calculate total width of the board
    let total_width = BOARD_SIZE * 10 + 4;  // 2 characters per cell + 1 for the vertical divider

    // Print title
    let padded_title = format!("{0: ^1$}", "Journal", total_width - 2);  // Center the title with padding
    let title_frame = format!("┏{}┓", "━".repeat(total_width - 2));
    let title_frame_bot = format!("┣{}┫", "━".repeat(total_width - 2));
    let title_line = format!("┃{}┃", padded_title);
    log_box.push(title_frame);
    log_box.push(title_line);
    log_box.push(title_frame_bot);

    let mut entries = LOGGER.get_messages();
    for _ in 0..(LOGGER.max_size - entries.len()) {
        entries.push_front("".to_string());
    }
    for msg in entries {
        log_box.push(format!("┃{0: <1$}┃", msg, total_width - 2));
    }

    log_box.push("┗".to_string() + &"━".repeat(total_width - 2) + "┛");

    for ((log_line, player_line), enemy_line) in log_box.iter().zip(player_board.iter()).zip(enemy_board.iter()) {
        println!("{}  {}  {}", log_line, player_line, enemy_line);
    }
}

const DAY: &str = r"
             ______   __  __     ______        _____     ______     __  __    
            /\__  _\ /\ \_\ \   /\  ___\      /\  __-.  /\  __ \   /\ \_\ \   
            \/_/\ \/ \ \  __ \  \ \  __\      \ \ \/\ \ \ \  __ \  \ \____ \  
               \ \_\  \ \_\ \_\  \ \_____\     \ \____-  \ \_\ \_\  \/\_____\ 
                \/_/   \/_/\/_/   \/_____/      \/____/   \/_/\/_/   \/_____/ 
";

const OF: &str = r"
                                 ______     ______  
                                /\  __ \   /\  ___\ 
                                \ \ \/\ \  \ \  __\ 
                                 \ \_____\  \ \_\   
                                  \/_____/   \/_/   
";

const SAGITTARIUS: &str = r"
    ███████╗ █████╗  ██████╗ ██╗████████╗████████╗ █████╗ ██████╗ ██╗██╗   ██╗███████╗    ██╗██╗   ██╗
    ██╔════╝██╔══██╗██╔════╝ ██║╚══██╔══╝╚══██╔══╝██╔══██╗██╔══██╗██║██║   ██║██╔════╝    ██║██║   ██║
    ███████╗███████║██║  ███╗██║   ██║      ██║   ███████║██████╔╝██║██║   ██║███████╗    ██║██║   ██║
    ╚════██║██╔══██║██║   ██║██║   ██║      ██║   ██╔══██║██╔══██╗██║██║   ██║╚════██║    ██║╚██╗ ██╔╝
    ███████║██║  ██║╚██████╔╝██║   ██║      ██║   ██║  ██║██║  ██║██║╚██████╔╝███████║    ██║ ╚████╔╝ 
    ╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝   ╚═╝      ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝ ╚═════╝ ╚══════╝    ╚═╝  ╚═══╝  
";