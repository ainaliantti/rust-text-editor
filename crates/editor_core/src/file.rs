/*
   File IO 
   */

use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::vec::Vec;

pub fn read_file(filename: &String) -> Vec<String>{
    let path = Path::new(filename); 
    let file = match File::open(path) {
        Ok(f) => f,
        Err(err) => panic!("Something went wrong: {}", err),
    };
    let reader = BufReader::new(file);
    let contents: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    contents
}

