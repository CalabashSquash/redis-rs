use crate::{decode::{OptionalRespElement, RespElement}, dispatch::CommandError, encode::encode, util::validate::is_correct_command};

pub fn echo(command: &[RespElement]) -> Result<Vec<u8>, CommandError> {
    // Sanity check echo is only called when it's actually ECHO
    is_correct_command(command, "echo")?;
    if command.len() != 2 {
        return Err(CommandError::new(String::from("ECHO Command is not len 2")));
    }

    return match &command[1] {
        RespElement::BulkString(s) => Ok(encode(&OptionalRespElement::BulkString(Some(s)))),
        _ => Err(CommandError::new(String::from("ECHO Command is not len 2"))),
    };
}