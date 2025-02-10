use crate::{decode::RespElement, dispatch::CommandError, encode::encode, util::validate::is_correct_command};

pub fn ping() -> Result<Vec<u8>, CommandError> {
    Ok(encode(&RespElement::BulkString("PONG"))?)
}