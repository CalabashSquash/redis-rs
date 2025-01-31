// Handle incoming redis commands.
// Dispatching them, responding, and checking for multiple commands.

use std::collections::HashMap;

use tokio::{io::{self, AsyncReadExt}, net::TcpStream, time::error::Error};

use crate::{dispatch::dispatch, redis::RedisState};

pub async fn multi_commands(stream: &mut TcpStream, storage: &mut HashMap<String, String>) {
    loop {
        let mut buff = [0u8; 128];
        let read_result = stream.read(&mut buff).await;
        match read_result {
            Ok(size) => {
                if size == 0 {
                    println!("Size 0 read from stream");
                    return;
                }
                let response = dispatch(&mut buff, size, storage).unwrap();
                stream.try_write(&response).unwrap();
            },
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                    println!("Would block!");
                    continue;
                }
                println!("Error: {e:?}");
                break;
            }
        }
    }
}