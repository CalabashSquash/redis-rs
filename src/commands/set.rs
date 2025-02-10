use std::{collections::HashMap, time::{Duration, Instant, SystemTime, UNIX_EPOCH}, u128};

use crate::{decode::RespElement, dispatch::CommandError, encode::encode, storage::{RedisStorage, RedisValue}, util::validate::is_correct_command};

pub fn set(command: &[RespElement], storage: &mut RedisStorage) -> Result<Vec<u8>, CommandError> {
    is_correct_command(command, "set")?;
    if command.len() < 3 {
        return Err(CommandError::new("SET Command length is less than 3"));
    }

    let key: &str;
    let value: &str;
    if let RespElement::BulkString(k) = &command[1] {
        key = k;
    } else {
        return Err(CommandError::new("SET key is not string"));
    }

    if let RespElement::BulkString(v) = &command[2] {
        value = v;
    } else {
        return Err(CommandError::new("SET value is not string"));

    }

    println!("key: {key}");
    println!("value: {value}");

    let expiry: u128 = get_expiration_time(command)?;

    let value = RedisValue {
        value: value.to_string(),
        expiry: expiry
    };

    storage.db.insert(key.to_string(), value);

    Ok(encode(&RespElement::SimpleString("OK"))?)
}

fn get_expiration_time(command: &[RespElement]) -> Result<u128, CommandError> {
    if command.len() == 5 {
        if let RespElement::BulkString(flag) = &command[3] {
            if flag.to_ascii_lowercase() == "px" {
                if let RespElement::BulkString(expiry_length) = &command[4] {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap() // Will only fail if time goes backwards
                        .as_millis();
                    return Ok(now + str::parse::<u128>(expiry_length)?);
                }
            }
        }
    }
    return Ok(u128::MAX);
}