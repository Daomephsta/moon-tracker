use crate::errors::InternalError;
use simple_error::SimpleError;
use std::fs::File;
use std::path::Path;

pub fn load_config(path: &Path) -> Result<Configuration, InternalError>
{
    if path.exists()
    {
        let file = File::open(&path)
            .expect("Unable to open config file");
        return match serde_json::from_reader(file)
        {
            Ok(config) => 
            {
                println!("Config loaded.");
                Ok(config)
            },
            Err(e) => bail!("Unable to parse config\nCause: {}", &e)
        }
    }
    else 
    { 
        let config = create_example_config(&path);
        println!("No config file found. Created & loaded example config file.");
        return config;
    }
}

pub fn create_example_config(path: &Path) -> Result<Configuration, InternalError>
{
    let example_cfg = Configuration 
    {
        moons: vec!
        [
            Moon
            {
                name: "Luna".to_string(),
                cycle_length: 31.0
            }
        ]
    };

    let file = File::create(&path)
        .map_err(|e| SimpleError::new(format!("Unable to create example config\nCause: {}", &e)))?;
    serde_json::to_writer_pretty(&file, &example_cfg)
        .map_err(|e| SimpleError::new(format!("Unable to write example config\nCause: {}", &e)))?;
    return Ok(example_cfg);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration
{
    moons: Vec<Moon>
}

impl Configuration
{
    pub fn moons(&self) -> &Vec<Moon>
    {
        &self.moons
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Moon 
{
    name: String,
    cycle_length: f64
}

impl Moon
{
    pub fn name(&self) -> &str 
    {
        &self.name
    }

    pub fn cycle_length(&self) -> f64
    {
        self.cycle_length
    }
}