use std::{collections::HashMap, fmt::format, io, num::ParseIntError};

use crate::{commands::{config_get::config_get, echo::echo, get::get, ping::ping, set::set}, decode::{decode, RespElement}, encode::EncodeError, storage::RedisStorage};

#[derive(Debug)]
pub struct CommandError {
    message: String
}

impl CommandError {
    pub fn new(message: &str) -> Self {
        CommandError {
            message: String::from(message)
        }
    }
}

impl From<ParseIntError> for CommandError {
    fn from(err: ParseIntError) -> Self {
        CommandError { message: format!("ParseIntError: {:#?}", err) }
    }
}

impl From<EncodeError> for CommandError {
    fn from(err: EncodeError) -> Self {
        CommandError { message: format!("EncodeError while running a command: {:#?}", err) }
    }
}

// At the moment we just return the response string. TODO is add proper error handling.
pub fn dispatch(input: &mut [u8], length: usize, storage: &mut RedisStorage) -> Result<Vec<u8>, CommandError> {
    let input = String::from_utf8_lossy(&input[..length]);
    // let input = String::from_utf8(input.to_vec()).unwrap(); // This one doesn't ignore the escape chars
    return match decode(&input.to_string()) {
        RespElement::Array(a) => dispatch_array(&a, storage),
        RespElement::BulkString(s) => todo!("Bulk string"),
        RespElement::EmptyBulkString => todo!("Empty bulk string"),
        RespElement::SimpleString(a) => todo!("Simple String"),
        RespElement::Integer(a) => todo!("Integer"),
    }
}

fn dispatch_array(array: &[RespElement], storage: &mut RedisStorage) -> Result<Vec<u8>, CommandError> {
    println!("RESP array: {array:#?}");
    // AFAIK so far, the first element will always be the command.
    return match &array[0] {
        RespElement::Array(a) => todo!("Array in array"),
        RespElement::BulkString(s) => match s.to_ascii_lowercase().as_str() {
            "ping" => ping(), 
            "echo" => echo(array), 
            "set" => set(array, storage),
            "get" => get(array, storage),
            "config" => match &array[1] {
                RespElement::BulkString(s2) => match s2.to_ascii_lowercase().as_str() {
                    "get" => config_get(array, storage),
                    _ => todo!("Rest of config BulkString arguments")
                },
                _ => todo!("Rest of config non-BulkString arguments")
            },
            _ => todo!("Rest of commands"),
        },
        RespElement::EmptyBulkString => todo!("dispatch Empty bulk string"),
        RespElement::SimpleString(s) => todo!("Simple string in array"),
        RespElement::Integer(a) => todo!("Integer in array"),
    };

}