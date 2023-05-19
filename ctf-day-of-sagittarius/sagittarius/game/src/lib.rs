#![cfg_attr(not(test), no_std)]
pub mod types;
pub mod state;
pub mod abilities;

pub const NUM_SHIPS: usize = 4;
pub const BOARD_SIZE: usize = 8;

pub const SHIP_SPANS: [usize; NUM_SHIPS] = [
    4, // Mothership
    3, // Carrier
    3, // Tempest
    2, // Void ray
];

pub const SUNK_MASKS: [u8; NUM_SHIPS] = [0x0f, 0x07, 0x07, 0x03];