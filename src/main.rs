extern crate azul;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate simple_error;

use std::io::{stdin, Read};
use std::path::Path;


mod config;
mod gui;
mod errors;

fn main() 
{
    if let Err(e) = run()
    {
        println!("Application error: {}", e);
        pause_on_exit();
    }
}

fn run() -> Result<(), errors::InternalError>
{
    let state = State
    {
        config: config::load_config(Path::new("config.json"))?
    };

    return Ok(());
}

fn pause_on_exit() 
{
    println!("Press Enter to exit...");
    stdin().read(&mut [0]).unwrap();
}

pub struct State
{
    config: self::config::Configuration
}