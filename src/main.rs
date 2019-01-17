extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::io::{stdin, Read};
use std::path::Path;

mod config;
mod cli;

fn main() 
{
    let old_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| 
    {
        old_hook(info);
        pause_on_exit();
    }));

    let state = State
    {
        config: config::load_config(Path::new("config.json"))
    };

    cli::start(&state);
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