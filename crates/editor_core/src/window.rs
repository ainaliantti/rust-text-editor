/*
   Window is what the user sees of the buffer(s).
   */

use std::vec::Vec;
use crate::buffer::Buffer;

pub struct Window {
    pub buffers: Vec<Buffer> 
}

impl Window {
    pub fn new(buffers: Vec<Buffer>) -> Window {
        Window { buffers }
    }
} 

