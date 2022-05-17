/*
   Buffer is an area of memory which holds text. Can be read from a file or stdin and written to a file.
   keeps an internal copy of the file to be edited until buffer is closed.
   */


use crate::file;

pub struct Buffer {
    pub filename: String,
    pub contents: Vec<String>,
}

impl Buffer {
    pub fn new(path: &String) -> Buffer {
        let filename = path.clone();
        let contents = file::read_file(&filename);
        Buffer { filename, contents }
    }
}

