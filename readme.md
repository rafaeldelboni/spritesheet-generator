# spritesheet-generator
[![Build Status](https://travis-ci.org/rafaeldelboni/spritesheet-generator.svg?branch=master)](https://travis-ci.org/rafaeldelboni/spritesheet-generator) [![Crate Version](http://meritbadge.herokuapp.com/spritesheet-generator)](https://crates.io/crates/spritesheet-generator)

A spritesheet generator library using the piston's `texture_packer`,
this lib provides the packed image and a json with all information following
the codeandweb's `Texture Packer` basic format.

## Usage
Add the crate named `spritesheet-generator` to your dependencies in `Cargo.toml`:
```ignore
[dependencies]
spritesheet-generator = "0.4"
```

## Code Sample
To export the spritesheet:
```rust
extern crate spritesheet_generator;
use spritesheet_generator::spritesheet_generator::generate;
use spritesheet_generator::spritesheet_generator_config::SpritesheetGeneratorConfig;

fn main() {
    let config = SpritesheetGeneratorConfig {
        max_width: 500,
        max_height: 500,
        border_padding: 4,
        input_folder: "examples/assets/".to_string(),
        output_folder: "examples/resources/".to_string(),
        output_file_name: "example".to_string(),
    };
    generate(config);
}
```

## Example
To test the sample code, run the code below from the project directory.
```bash
# Be sure you're running this code at the root of the project directory!
cargo run --example generate-test
```

## Credits
- [Piston/Texture Packer](https://github.com/PistonDevelopers/texture_packer)
- [Texture Packer](https://www.codeandweb.com/texturepacker)
- [Example Assets](https://opengameart.org/content/5-more-rpgfantasy-weapons)
