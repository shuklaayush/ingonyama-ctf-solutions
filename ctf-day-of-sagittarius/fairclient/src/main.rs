mod board;
mod logger;
mod net_utils;
mod interface;
mod session;
mod utils;

use std::io::Write;

use lazy_static::lazy_static;
use crate::logger::BufferLogger;
use interface::draw_night_sky;
use net_utils::{chunk_read, write_message};
use sagittarius_game::BOARD_SIZE;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::Result;

use risc0_zkvm::{Receipt, serde};
use sagittarius_game::state::GameState;
use sagittarius_game::types::{Digest};
use sagittarius_zk::{create_init_proof, check_init_proof};

use crate::session::GameSession;
use crate::board::choose_board;
use crate::net_utils::{chunk_write, read_message};

lazy_static! {
    pub static ref LOGGER: BufferLogger = BufferLogger::new((BOARD_SIZE+1) * 2 - 1);
}

async fn verify_enemy_board_init(socket: &mut TcpStream) -> Result<Digest> {
    let receipt_bytes = chunk_read(socket).await?;
    let receipt: Receipt = serde_cbor::from_slice(&receipt_bytes).expect("Unable to deserialize receipt");

    if !check_init_proof(&receipt).is_ok() {
        write_message(socket, "Invalid init proof!".as_bytes()).await?;
        return Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Error"));
    }

    let journal = receipt.get_journal_bytes(); 
    let digest = serde::from_slice::<Digest, u8>(&journal).unwrap();

    Ok(digest)
}

async fn prove_board_init(state: &GameState, socket: &mut TcpStream) -> Result<()> {
    LOGGER.log("Generating proof... This might take a while");
    let receipt = create_init_proof(state).expect("Failed");
    let rec_vec = serde_cbor::to_vec(&receipt).expect("Error while serializing receipt");
    chunk_write(socket, &rec_vec).await.expect("Error while sending receipt");
    Ok(())
}

async fn join_game() -> Result<()> {
    LOGGER.log("Enter host:port");
    let mut input = String::new();
    print!(">: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    let mut stream = TcpStream::connect(input.trim()).await?;

    draw_night_sky(Some(1), Some(20));

    LOGGER.log("Connected to server");

    let state = choose_board();
    prove_board_init(&state, &mut stream).await?;

    let response = read_message(&mut stream).await?;
    LOGGER.log(&response);

    let enemy_state = verify_enemy_board_init(&mut stream).await?;

    LOGGER.log("Server has valid setup. The game is starting!");
    let mut session = GameSession::new(stream, state, enemy_state, true);
    session.game_cycle().await?;

    Ok(())
}

async fn host_game() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Can't bind to 8080");
    LOGGER.log("Started server at :8080 ... awaiting connections");

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        LOGGER.log(&format!("Got connection from {:?}", socket.peer_addr().unwrap().ip()));
        tokio::spawn(async move {
            let enemy_state = verify_enemy_board_init(&mut socket).await.unwrap();
            
            write_message(&mut socket, "Everything good. Sending my board...".as_bytes()).await.unwrap();
            let state = choose_board();

            LOGGER.log("Generating init proof...");
            prove_board_init(&state, &mut socket).await.unwrap();

            let mut session = GameSession::new(socket, state, enemy_state, false);
            session.game_cycle().await.unwrap();
        });
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    log::set_max_level(log::LevelFilter::Info);

    loop {
        LOGGER.log("1. Join game");
        LOGGER.log("2. Host game");
        LOGGER.log("3. Exit");
        let mut input = String::new();
        print!(">: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        let option = input.trim().parse::<u8>();
        
        match option {
            Ok(n) => match n {
                1 => {
                    join_game().await?;
                },
                2 => {
                    host_game().await?;
                },
                3 => {
                    std::process::exit(0);
                },
                _ => {
                    LOGGER.log("Enter a number from 1 to 3");
                    continue;
                }
            },
            Err(_) => {
                LOGGER.log("Enter a number from 1 to 3");
                continue;
            }
        }
    }
}