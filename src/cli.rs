use std::io::stdin;
use notify::DebouncedEvent;
use crate::ConfigWatcher;

const PHASES: [&str; 8] = ["New moon", "Waxing crescent", "First quarter", "Waxing gibbous", "Full moon", "Waning gibbous", "Third quarter", "Waning crescent"];

pub fn start(state: &mut crate::State, config_watcher: ConfigWatcher)
{
    'cli: loop
    {
        loop 
        {
            let mut should_run = String::new();
            println!("Run Moon Tracker? (Y/N)");
            stdin().read_line(&mut should_run)
                .expect("Unable to read from standard input");
            should_run = should_run.trim().to_lowercase();
            if should_run == "y"
            {
                run(&state);
                break;
            }
            else if should_run == "n"
            {
                break 'cli;
            }
            else
            {
                println!("Invalid answer. Valid answers are Y, y, N, and n.");
            }
        }
        
        if let Ok(event) = config_watcher.try_recv()
        {
            match event
            {
                DebouncedEvent::NoticeWrite(path) | DebouncedEvent::Write(path) => state.set_config(crate::config::load_config(&path)), 
                DebouncedEvent::Error(e, _) => eprintln!("An error occurred in the config watcher {}", e),
                _ => {}
            }
        }
    }
}

fn run(state: &crate::State)
{
    loop
    {
        // Get day
        let mut day = String::new();
        println!("Input the day of the lunar cycle: ");
        stdin().read_line(&mut day)
            .expect("Unable to read from standard input");
        day = day.trim().to_string();
        let day = match day.parse::<i32>()
        {
            Ok(num) => num,
            Err(_) => 
            {
                println!("Could not parse {} as an integer", day);
                continue;
            }
        };
        
        println!("On day {} of the lunar cycle", day);
        for moon in state.config.moons()
        {
            let numerator = day as f64 % moon.cycle_length();
            //Integer divide the numerator by the moon's phase length (1/8 of cycle length), then modulo the result by 8
            let f: usize = ((numerator / (moon.cycle_length() / 8.0)) % 8.0) as usize;
            let phase = PHASES[f];
            println!("{} will be {}/{} through its cycle ({})", moon.name(), numerator, moon.cycle_length(), phase)
        }
        //Newline
        println!("");
        break;
    }
}