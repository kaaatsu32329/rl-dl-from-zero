use std::env;

use common::gridworld;
use rand::Rng;

fn main() {
    let environment = gridworld::GridWorld::new();
    let mut v = vec![];
    for _ in environment.states() {
        v.push(rand::thread_rng().gen::<i32>() as i32);
    }
}
