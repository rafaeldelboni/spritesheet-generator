extern crate serde_json;
extern crate spritesheet_generator;

use std::fs::File;

use spritesheet_generator::spritesheet::Spritesheet;

fn main() {
    // Open the file in read-only mode.
    let file = File::open("examples/resources/example.json").unwrap();

    // Read the JSON contents of the file as an instance of `Spritesheet`.
    let sprites: Spritesheet = serde_json::from_reader(file).unwrap();

    println!("Json parsed {:?}", sprites);
}
