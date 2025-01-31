use std::{collections::HashMap};

use tokio::net::TcpStream;

use crate::handler::multi_commands;


pub struct RedisState {
    key_val: HashMap<String, String> 
}

impl RedisState {
    pub fn new() -> Self {
        RedisState {
            key_val: HashMap::new()
        }
    }
}
