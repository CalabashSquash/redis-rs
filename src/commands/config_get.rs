use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};

use crate::{decode::RespElement, dispatch::CommandError, encode::encode, redis::RedisState, storage::{RedisStorage, RedisValue}, util::validate::is_correct_command};

pub fn config_get(command: &[RespElement], storage: &RedisStorage) -> Result<Vec<u8>, CommandError> {
    is_correct_command(command, "config")?;
    if command.len() < 3 {
        return Err(CommandError::new("CONFIG GET Command length is less than 3"));
    }

    let key: &str;
    let value: &str;
    if let RespElement::BulkString(k) = &command[2] {
        key = k;
    } else {
        return Err(CommandError::new("CONFIG GET key is not string"));
    }
    
    if key == "dir" {
        value = storage.rdbArgs.dir.as_str();
    } else if key == "dbfilename" {
        value =  storage.rdbArgs.dbfilename.as_str();
    } else {
        return Err(CommandError::new("incorrect param given to CONFIG GET"));
    }

    return Ok(encode(&RespElement::Array(vec![RespElement::BulkString(key), RespElement::BulkString(value)]))?);
}
