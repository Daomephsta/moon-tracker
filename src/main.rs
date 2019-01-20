extern crate notify;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use notify::{Watcher, RecursiveMode};

use std::io::{stdin, Read};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;
use std::thread;

type ConfigWatcher = mpsc::Receiver<notify::DebouncedEvent>;

mod config;
mod cli;

fn main() 
{
    const CONFIG_PATH: &str = "config.json";

    let old_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| 
    {
        old_hook(info);
        pause_on_exit();
    }));

    let state = State
    {
        config: config::load_config(Path::new(CONFIG_PATH))
    };

    let config_watcher = create_config_watcher(CONFIG_PATH);
    cli::start(&state, config_watcher);
}

fn pause_on_exit() 
{
    println!("Press Enter to exit...");
    stdin().read(&mut [0]).unwrap();
}

fn create_config_watcher(config_path: &'static str) -> ConfigWatcher
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move ||
    {
        //Notify docs say NOTHING about why either of these might error...
        let mut watcher = notify::watcher(tx, Duration::from_secs(5)).unwrap();
        watcher.watch(config_path, RecursiveMode::NonRecursive).unwrap();     
    });

    return rx;
}

#[derive(Debug)]
pub struct State
{
    config: self::config::Configuration
}