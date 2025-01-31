use crate::{decode::OptionalRespElement, dispatch::CommandError, encode::encode, util::validate::is_correct_command};

pub fn ping() -> Result<Vec<u8>, CommandError> {
    Ok(encode(&OptionalRespElement::BulkString(Some("PONG"))))
}