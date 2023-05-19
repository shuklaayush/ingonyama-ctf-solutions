#![no_main]
#![no_std]

use risc0_zkvm::guest::env;
use risc0_zkvm::guest::sha::Impl;
use risc0_zkvm::sha::Sha256;
use risc0_zkvm::serde;

use sagittarius_game::state::GameState;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let state: GameState = env::read();
    if !state.check() {
        panic!("Invalid GameState");
    }
    env::commit(Impl::hash_words(serde::to_vec(&state).unwrap().as_slice()));
}