use std::collections::HashMap;

use crate::{decode::{OptionalRespElement, RespElement}, dispatch::CommandError, encode::encode, util::validate::is_correct_command};

pub fn set(command: &[RespElement], storage: &mut HashMap<String, String>) -> Result<Vec<u8>, CommandError> {
    is_correct_command(command, "set")?;
    if command.len() < 3 {
        return Err(CommandError::new(String::from("SET Command length is less than 3")));
    }

    let key: &str;
    let value: &str;
    if let RespElement::BulkString(k) = &command[1] {
        key = k;
    } else {
        return Err(CommandError::new(String::from("SET key is not string")));
    }

    if let RespElement::BulkString(v) = &command[2] {
        value = v;
    } else {
        return Err(CommandError::new(String::from("SET value is not string")));

    }

    println!("key: {key}");
    println!("value: {value}");

    storage.insert(key.to_string(), value.to_string());

    Ok(encode(&OptionalRespElement::SimpleString(Some("OK"))))
}
