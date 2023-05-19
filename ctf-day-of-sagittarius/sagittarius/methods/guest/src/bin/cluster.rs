#![no_main]
#![no_std]
extern crate alloc;

use risc0_zkvm::guest::{env, sha::Impl, zkvm_getrandom};
use risc0_zkvm::sha::Sha256;
use risc0_zkvm::serde;

use sagittarius_game::abilities::{ClusterBombParams, ClusterCommit};
use sagittarius_game::types::Position;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let params: ClusterBombParams = env::read();
    let width = (params.down_right_coordinates.x - params.upper_left_coordinates.x) as u8;
    let height = (params.down_right_coordinates.y - params.upper_left_coordinates.y) as u8;

    // Generate some random shots
    let number_of_shots = params.check();
    let mut rand = alloc::vec![0; number_of_shots];
    zkvm_getrandom(rand.as_mut_slice()).unwrap();

    let mut area = alloc::vec::Vec::with_capacity(width as usize * height as usize);
    for x in params.upper_left_coordinates.x..=params.down_right_coordinates.x {
        for y in params.upper_left_coordinates.y..=params.down_right_coordinates.y {
            area.push(Position::new(x, y));
        }
    }

    let mut shots = alloc::vec::Vec::new();
    for i in 0..number_of_shots {
        let index = rand[i] as usize % area.len();
        shots.push(area.remove(index));
    }

    let result = params.process(&shots);

    let mut old_state_digest = [0u32; 8];
    let mut new_state_digest = [0u32; 8];
    old_state_digest.as_mut_slice().copy_from_slice(Impl::hash_words(serde::to_vec(&params.state).unwrap().as_slice()).as_words());
    new_state_digest.as_mut_slice().copy_from_slice(Impl::hash_words(serde::to_vec(&result.state).unwrap().as_slice()).as_words());
    env::commit(&ClusterCommit {
        config: params,
        old_state_digest,
        new_state_digest,
        shots,
        hits: result.hits,
    });
}