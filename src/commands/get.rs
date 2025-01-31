use std::collections::HashMap;

use crate::{decode::{OptionalRespElement, RespElement}, dispatch::CommandError, encode::encode, util::validate::is_correct_command};

pub fn get(command: &[RespElement], storage: &HashMap<String, String>) -> Result<Vec<u8>, CommandError> {
    is_correct_command(command, "get")?;
    if command.len() < 2 {
        return Err(CommandError::new(String::from("GET Command length is less than 3")));
    }

    let key: &str;
    let value: Option<&String>;
    if let RespElement::BulkString(k) = &command[1] {
        key = k;
    } else {
        return Err(CommandError::new(String::from("GET key is not string")));
    }

    println!("key: {key}");

    value = storage.get(key);
    if let Some(v) = value {
        println!("value: {v}");
        return Ok(encode(&OptionalRespElement::BulkString(Some(v))));
    }

    return Ok(encode(&OptionalRespElement::BulkString(None)));
}
