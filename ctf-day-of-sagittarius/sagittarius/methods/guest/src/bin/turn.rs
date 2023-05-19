#![no_main]
#![no_std]

use risc0_zkvm::guest::env;
use risc0_zkvm::guest::sha::Impl;
use risc0_zkvm::sha::Sha256;
use risc0_zkvm::serde;

use sagittarius_game::abilities::{ShotCommit, ShotParams};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let params: ShotParams = env::read();
    let result = params.process();

    let mut old_state_digest = [0u32; 8];
    let mut new_state_digest = [0u32; 8];
    old_state_digest.as_mut_slice().copy_from_slice(Impl::hash_words(serde::to_vec(&params.state).unwrap().as_slice()).as_words());
    new_state_digest.as_mut_slice().copy_from_slice(Impl::hash_words(serde::to_vec(&result.state).unwrap().as_slice()).as_words());
    env::commit(&ShotCommit {
        old_state_digest,
        new_state_digest,
        shot: params.shot,
        hit: result.hit,
    });
}