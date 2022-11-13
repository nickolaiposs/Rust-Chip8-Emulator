use std::fs::File;
use std::io::prelude::*;

pub fn game_loader(filepath: &String) -> Vec<u8> {
    let mut game_file = File::open(filepath).expect("File cannot be loaded");
    let mut buffer: Vec<u8> = Vec::new();

    game_file.read_to_end(&mut buffer).expect("There was an error with the game loading");

    buffer
}