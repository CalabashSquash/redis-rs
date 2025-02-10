use crate::{decode::RespElement, dispatch::CommandError};

pub fn is_correct_command(command: &[RespElement], correct_command: &str) -> Result<(), CommandError> {
    let correct_command = correct_command.to_ascii_lowercase();
    match &command[0] {
        RespElement::BulkString(s) => {
            if s.to_ascii_lowercase().as_str() != correct_command {
                return Err(CommandError::new(format!("Command is not {correct_command}").as_str()));
            }
            return Ok(())
        },
        _ => return Err(CommandError::new(format!("Command is not {correct_command}").as_str())),
    }

}