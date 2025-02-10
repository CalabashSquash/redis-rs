use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};

use crate::{decode::RespElement, dispatch::CommandError, encode::encode, redis::RedisState, storage::{RedisStorage, RedisValue}, util::validate::is_correct_command};

pub fn get(command: &[RespElement], storage: &RedisStorage) -> Result<Vec<u8>, CommandError> {
    is_correct_command(command, "get")?;
    if command.len() < 2 {
        return Err(CommandError::new("GET Command length is less than 2"));
    }

    let key: &str;
    let value: Option<&RedisValue>;
    if let RespElement::BulkString(k) = &command[1] {
        key = k;
    } else {
        return Err(CommandError::new("GET key is not string"));
    }

    value = storage.db.get(key);
    if let Some(v) = value {
        println!("value: {v:#?}");
        // TODO check expiry
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap() // Will only fail if time goes backwards
            .as_millis();
        let expiration_time = v.expiry;
        if now >= expiration_time {
            return Ok(encode(&RespElement::EmptyBulkString)?)
        }
        return Ok(encode(&RespElement::BulkString(&v.value))?);
    }

    return Ok(encode(&RespElement::EmptyBulkString)?);
}
