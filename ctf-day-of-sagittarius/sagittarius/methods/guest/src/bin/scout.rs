#![no_main]
#![no_std]

use risc0_zkvm::guest::env;
use sagittarius_game::abilities::ScoutParams;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let params: ScoutParams = env::read();
    let result = params.process();
    env::commit(&result);
}