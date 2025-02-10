#![allow(unused_imports)]
use std::{collections::HashMap, hash::Hash, io::{Read, Write}};
use storage::RedisStorage;
use tokio::net::{TcpListener, TcpStream};
use clap::Parser;
use std::sync::Arc;

mod commands;
mod util;
mod dispatch;
mod encode;
mod decode;
mod handler;
mod redis;
mod storage;

use dispatch::dispatch;
use handler::multi_commands;
use redis::RedisState;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RdbArgs {
    // RDB file directory
    #[arg(long, default_value = "/tmp/redis-files")]
    dir: String,

    // RDB file name
    #[arg(short, long, default_value = "dump.rdb")]
    dbfilename: String,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let redis = RedisState::new();
    let args = RdbArgs::parse();
    let args = Arc::from(args);
    println!("{args:?}");
    
    loop {
        let connection = listener.accept().await;

        match connection {
            Ok((mut _stream, _)) => {
                let args_clone = Arc::clone(&args); // Cheap.
                println!("accepted new connection");
                tokio::spawn(async move {
                    new_connection(&mut _stream, &args_clone).await;
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

async fn new_connection(stream: &mut TcpStream, args: &Arc<RdbArgs>) {
    // stream.readable().await.unwrap();
    // let mut storage = HashMap::new();
    let mut storage = RedisStorage {
        db: HashMap::new(),
        rdbArgs: args
    };
    multi_commands(stream, &mut storage).await;
}
