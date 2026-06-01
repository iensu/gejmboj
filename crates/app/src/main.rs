use std::env;
use std::fs;

use log::debug;

fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let Some(file_path) = env::args().nth(1) else {
        panic!("You must provide a path to a Game Boy rom file!");
    };
    let rom_bytes = fs::read(&file_path)?;

    debug!("Read {} bytes from {file_path}", rom_bytes.len());

    Ok(())
}
