#![allow(unused_imports)]
use std::{collections::HashMap, io::{Read, Write}};
use tokio::net::{TcpListener, TcpStream};

mod commands;
mod util;
mod dispatch;
mod encode;
mod decode;
mod handler;
mod redis;

use commands::{ping};
use dispatch::dispatch;
use handler::multi_commands;
use redis::RedisState;

async fn print_hi() {
    println!("hi");
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let redis = RedisState::new();
    
    loop {
        let connection = listener.accept().await;

        match connection {
            Ok((mut _stream, _)) => {
                println!("accepted new connection");
                tokio::spawn(async move {
                    new_connection(&mut _stream).await;
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }


    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(mut _stream) => {
    //             println!("accepted new connection");
    //             multi_commands(&mut _stream);
    //         }
    //         Err(e) => {
    //             println!("error: {}", e);
    //         }
    //     }
    // }
}

async fn new_connection(stream: &mut TcpStream) {
    // stream.readable().await.unwrap();
    let mut storage = HashMap::new();
    multi_commands(stream, &mut storage).await;
}
