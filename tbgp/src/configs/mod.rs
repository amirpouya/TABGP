extern crate toml;
extern crate serde;

use std::fs::File;
use std::io::Read;
use serde::{Deserialize};


#[derive(Debug,Deserialize)]
pub struct Config{
    pub input_dir : String,
    pub nfa_dir: String,
    pub debug: usize,
    pub pattern_type: String,
    pub pattern_size : usize,

}

pub fn parse(some_filename: &str) -> Config {
    let mut file = File::open(some_filename).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let my_config: Config = toml::from_str(&s).unwrap();
    my_config
}




