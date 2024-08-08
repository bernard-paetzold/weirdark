use std::sync::Mutex;
use rltk::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}

pub fn reseed(seed: u64) {
    *RNG.lock().unwrap() = RandomNumberGenerator::seeded(seed);
}

pub fn range(min: i32, max:i32) -> i32 {
    RNG.lock().unwrap().range(min, max)
}

pub fn random_int() -> i32 {
    RNG.lock().unwrap().rand()
}