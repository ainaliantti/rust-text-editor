/*
   File IO 
   */

use std::fs;
use std::path::Path;
use std::io::ErrorKind;

pub fn read_file(filename: &String) -> String {
    let path = Path::new(filename); 
    match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => String::new(),
            other_error => {
                panic!("Problem opening file: {:?}", other_error)
            }
        },
    }
}

