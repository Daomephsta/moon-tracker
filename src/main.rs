extern crate notify;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use notify::{DebouncedEvent, RecursiveMode, Watcher};

use std::io::{stdin, Read};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;
use std::thread;

type ConfigWatcher = mpsc::Receiver<DebouncedEvent>;

mod config;
mod cli;

const CONFIG_PATH: &str = "config.json";

fn main() 
{
    let old_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| 
    {
        old_hook(info);
        pause_on_exit();
    }));

    let mut state = State
    {
        config: config::load_config(Path::new(CONFIG_PATH))
    };

    
    let (watcher_tx, watcher_rx) = mpsc::channel();
    //Notify docs say NOTHING about why either of these might error...
    let mut watcher = notify::watcher(watcher_tx, Duration::from_secs(1)).unwrap();
    watcher.watch(Path::new(CONFIG_PATH), RecursiveMode::NonRecursive).unwrap(); 

    let (config_tx, config_rx) = mpsc::channel();
    thread::Builder::new()
        .name("config-watcher".to_string())
        .spawn(move ||
        {
            loop
            {
                match watcher_rx.recv()
                {
                    Ok(event) => config_tx.send(event).unwrap(),
                    Err(e) => eprintln!("Error in config watcher loop {}", e)
                }
            }
        })
        .unwrap();

    cli::start(&mut state, config_rx);
}

fn pause_on_exit() 
{
    println!("Press Enter to exit...");
    stdin().read(&mut [0]).unwrap();
}

/*fn create_config_watcher(config_path: &'static str) -> mpsc::Receiver<DebouncedEvent>
{
        
    return watcher_rx;
}*/

/*fn spawn_watcher_thread(watcher_rx: mpsc::Receiver<DebouncedEvent>, config_tx: mpsc::Sender<DebouncedEvent>) -> thread::JoinHandle<()>
{
    
}*/

#[derive(Debug)]
pub struct State
{
    config: self::config::Configuration
}

impl State
{
    pub fn set_config(&mut self, new_config: self::config::Configuration)
    {
        self.config = new_config;
    }
}