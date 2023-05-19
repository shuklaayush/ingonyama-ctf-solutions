use risc0_zkvm::Receipt;
use sagittarius_game::{NUM_SHIPS, SUNK_MASKS};
use sagittarius_game::abilities::{ShotParams, ClusterBombParams, ScoutParams};
use sagittarius_game::types::{Position, HitType};
use sagittarius_game::{state::GameState, types::Digest};
use sagittarius_zk::{check_turn_proof, create_turn_proof, check_cluster_proof, create_cluster_proof, create_scout_proof, check_scout_proof};
use tokio::{net::TcpStream, io::AsyncWriteExt};
use tokio::io::{Result, AsyncReadExt};
use std::io::Write;

use crate::net_utils::{chunk_read, get_coords_from_server, chunk_write, ClusterMessage, get_cluster_message, read_message, write_message};
use crate::utils::get_coords_from_stdin;
use crate::{board::{HitMap, create_empty_shots}, interface::{update_enemy_game_board, update_player_game_board}, LOGGER};

const MAX_SCOUTS: u8 = 1;
const MAX_CLUSTER_BOMBS: u8 = 1;
const LOSS_MESSAGE: &str = "Thank you for the game!";

pub struct GameSession {
    stream: TcpStream,
    state: GameState,
    hits: HitMap,
    enemy_state: Digest,
    turn: bool,
    scouts: u8,
    clusters: u8,
    enemy_scouts: u8,
    enemy_cluster: u8,
}

impl GameSession {
    pub fn new(stream: TcpStream, state: GameState, enemy_state: Digest, turn: bool) -> Self {
        GameSession {
            stream,
            state,
            hits: create_empty_shots(),
            enemy_state,
            turn,
            scouts: MAX_SCOUTS,
            clusters: MAX_CLUSTER_BOMBS,
            enemy_scouts: MAX_SCOUTS,
            enemy_cluster: MAX_CLUSTER_BOMBS,
        }
    }

    fn check_for_win(&self) -> bool {
        self.hits.values().filter(|h| matches!(h, HitType::Destroyed(_))).count() == NUM_SHIPS
    }

    fn check_for_loss(&self) -> bool {
        self.state.ships.iter().enumerate().all(|(i, s)| s.hit_mask == SUNK_MASKS[i])
    }

    async fn read_win_message(&mut self) -> Result<()> {
        let msg = read_message(&mut self.stream).await?;
        LOGGER.log(format!("Enemy: {}", msg).as_str());
        Ok(())
    }

    async fn write_loss_message(&mut self) -> Result<()> {
        write_message(&mut self.stream, LOSS_MESSAGE.as_bytes()).await
    }

    pub async fn game_cycle(&mut self) -> Result<()> {
        loop {
            if self.turn {
                LOGGER.log("Doing player turn...");
                self.do_player_turn().await?;
                update_enemy_game_board(&self.hits);

                if self.check_for_win() {
                    LOGGER.log("You won! Let's see what the loser has to say:");
                    self.read_win_message().await?;
                    break;
                }
            } else {
                LOGGER.log("Waiting for enemy turn...");
                self.do_enemy_turn().await?;
                update_player_game_board(&self.state);
                if self.check_for_loss() {
                    LOGGER.log("All our ships have been destroyed... We lost");
                    self.write_loss_message().await?;
                    break;
                }
            }

            self.turn = !self.turn;
        }

        Ok(())
    }

    async fn do_simple_shot(&mut self) -> Result<()> {
        self.stream.write_u8(1).await?;

        // P1 input hit
        let pos = get_coords_from_stdin(Some(&self.hits));
        let pos_vec = serde_cbor::to_vec(&pos).expect("Failed serialization");
        
        // Send pos to P2
        self.stream.write(&pos_vec).await?;

        LOGGER.log("The shot is fired. Reading the proof from server...");
        // Recieve proof from P2
        let receipt_bytes = chunk_read(&mut self.stream).await?;
        let receipt: Receipt = serde_cbor::from_slice(&receipt_bytes).expect("Unable to deserialize receipt");

        // Check proof from P2
        let (hit, new_enemy_state) = check_turn_proof(receipt, &pos, &self.enemy_state).expect("Invalid proof from server");
        self.hits.insert(pos, hit);
        self.enemy_state = new_enemy_state;

        Ok(())
    }

    async fn do_scout(&mut self) -> Result<()> {
        self.stream.write_u8(2).await?;
        self.scouts -= 1;

        // P1 input hit
        let pos = get_coords_from_stdin(Some(&self.hits));
        let pos_vec = serde_cbor::to_vec(&pos).expect("Failed serialization");
        
        // Send pos to P2
        self.stream.write(&pos_vec).await?;

        LOGGER.log("The scouts are on positions. Reading the proof from server...");
        // Recieve proof from P2
        let receipt_bytes = chunk_read(&mut self.stream).await?;
        let receipt: Receipt = serde_cbor::from_slice(&receipt_bytes).expect("Unable to deserialize receipt");

        let hits = check_scout_proof(receipt, &pos).expect("Invalid scout proof from server");
        let positions = [Position::new(pos.x - 1, pos.y),
                         Position::new(pos.x, pos.y - 1),
                         Position::new(pos.x + 1, pos.y),
                         Position::new(pos.x, pos.y + 1)];

        for (pos, hit) in positions.iter().zip(hits.iter()) {
            self.hits.insert(pos.clone(), hit.clone());
        }
        Ok(())
    }

    async fn do_cluster_charge(&mut self) -> Result<()> {
        self.stream.write_u8(3).await?;
        self.clusters -= 1;

        let mut ul: Position;
        let mut dr: Position;

        loop {
            LOGGER.log("You should select a rectangle with area from 4 to 14");
            LOGGER.log("Enter coordinates of the upper left corner");
            ul = get_coords_from_stdin(None);

            LOGGER.log("Enter coordinates of the down right corner");
            dr = get_coords_from_stdin(None);

            let area = (dr.x - ul.x + 1) * (dr.y - ul.y + 1);
            if area > 14 || area < 4 || dr.x - ul.x == 0 || dr.y - ul.y == 0 {
                LOGGER.log("ERROR: incorrect area");
            } else {
                break;
            }
        }

        let seed = rand::random::<u8>();

        let msg = ClusterMessage { ul: ul.clone(), dr: dr.clone(), seed };
        let msg_vec = serde_cbor::to_vec(&msg).expect("Failed serialization");
        self.stream.write(&msg_vec).await?;

        LOGGER.log("Cluster charge is fired. Waiting...");
        // Recieve proof from P2
        let receipt_bytes = chunk_read(&mut self.stream).await?;
        let receipt: Receipt = serde_cbor::from_slice(&receipt_bytes).expect("Unable to deserialize receipt");

        // Check proof from P2
        let (shots, hits, new_enemy_state) = check_cluster_proof(receipt, ul, dr, seed, self.enemy_state).expect("Invalid proof from server");
        
        for (shot, hit) in shots.iter().zip(hits.iter()) {
            self.hits.insert(shot.clone(), *hit);
        }
        self.enemy_state = new_enemy_state;

        Ok(())
    }

    async fn do_player_turn(&mut self) -> Result<()> {
        LOGGER.log("");
        LOGGER.log("It's your turn! What would you like to do?");
        LOGGER.log("==========================================");
        LOGGER.log("1. BLIND FIRE - shoot 1 cell on the board");

        if self.scouts > 0 {
            LOGGER.log(format!("2. SEND SCOUTS - reveal spaceships in an area ({} left)", self.scouts).as_str());
        } else {
            LOGGER.log("2. SEND SCOUTS (0 left)");
        }

        if self.clusters > 0 {
            LOGGER.log(format!("3. CLUSTER CHARGE - randomly bombard the area ({} left)", self.clusters).as_str());
        } else {
            LOGGER.log("3. CLUSTER CHARGE (0 left)");
        }

        loop {
            let mut input = String::new();
            print!(">: ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");

            let option = input.trim().parse::<u8>();
            
            match option {
                Ok(n) => match n {
                    1 => {
                        self.do_simple_shot().await?;
                        break;
                    },
                    2 => {
                        if self.scouts > 0 {
                            self.do_scout().await?;
                            break;
                        } else {
                            LOGGER.log("You are out of charges!");
                            continue;
                        }
                    },
                    3 => {
                        if self.clusters > 0 {
                            self.do_cluster_charge().await?;
                            break;
                        } else {
                            LOGGER.log("You are out of charges!");
                            continue;
                        }
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
        self.stream.write("ok".as_bytes()).await?;

        Ok(())
    }

    async fn handle_simple_shot(&mut self) -> Result<Receipt> {
        // P1 recieves hit
        let pos = get_coords_from_server(&mut self.stream).await?;
        assert!(pos.check());

        // P1 update state
        let params = ShotParams::new(self.state.clone(), pos.x, pos.y);
        let result = params.process();

        if let HitType::Hit = result.hit {
            LOGGER.log("Our spaceship has been hit!");
        }
        if let HitType::Destroyed(_) = result.hit {
            LOGGER.log("One of your ships was destroyed!");
        }

        self.state = result.state;

        // P1 generate turn proof
        LOGGER.log("Generating turn proof...");
        let proof = create_turn_proof(&params).expect("Error while creating the turn proof!");
        Ok(proof)
    }

    async fn handle_scout(&mut self) -> Result<Receipt> {
        self.enemy_scouts -= 1;
        LOGGER.log("⚠⚠⚠ The enemy is using scan! ⚠⚠⚠");

        let pos = get_coords_from_server(&mut self.stream).await?;
        assert!(pos.check());
        
        // P1 update state
        let params = ScoutParams{ state: self.state.clone(), shot: pos };
        LOGGER.log("Generating scout proof...");
        let receipt = create_scout_proof(&params).expect("Failed scout proof");
        Ok(receipt)
    }

    async fn handle_cluster(&mut self) -> Result<Receipt> {
        self.enemy_cluster -= 1;
        LOGGER.log("⚠⚠⚠ CLUSTER CHARGE INBOUND ⚠⚠⚠");

        // P1 recieves hit
        let msg = get_cluster_message(&mut self.stream).await?;
        msg.check();

        // P1 update state
        let params = ClusterBombParams::new(self.state.clone(), msg.ul.clone(), msg.dr.clone(), msg.seed);
        LOGGER.log("Generating Cluster proof...");
        let (receipt, shots) = create_cluster_proof(&params).expect("Failed cluster proof");
        let result = params.process(&shots);

        self.state = result.state;
        Ok(receipt)
    }

    async fn do_enemy_turn(&mut self) -> Result<()> {
        let action = self.stream.read_u8().await?;

        let receipt: Receipt;
        match action {
            1 => {
                receipt = self.handle_simple_shot().await?;
            },
            2 => {
                if self.enemy_scouts > 0 {
                    receipt = self.handle_scout().await?;
                } else {
                    println!("Player is cheating. Disconnecting...");
                    std::process::exit(0);
                }
            },
            3 => {
                if self.enemy_cluster > 0 {
                    receipt = self.handle_cluster().await?;
                } else {
                    println!("Player is cheating. Disconnecting...");
                    std::process::exit(0);
                    
                }
            }
            _ => {
                return Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Error"))
            }
        }

        let proof_bytes = serde_cbor::to_vec(&receipt).expect("Failed serialization for turn proof");
        // P1 send proof
        chunk_write(&mut self.stream, &proof_bytes).await?;

        let mut msg_bytes = [0; 2];
        self.stream.read(&mut msg_bytes).await?;
        LOGGER.log(&format!("Got response: {}", String::from_utf8(msg_bytes.to_vec()).unwrap()));
        Ok(())
    }
}
