use std::string::FromUtf8Error;

use crate::decode::RespElement;

#[derive(Debug)]
pub struct EncodeError {
    message: String
}

impl EncodeError {
    pub fn new(message: String) -> Self {
        EncodeError {
            message
        }
    }
}

impl From<FromUtf8Error> for EncodeError {
    fn from(err: FromUtf8Error) -> Self {
        EncodeError { message: format!("Failed when converting an encoded element back to a string: {:#?}", err) }
    }
}


pub fn encode(message: &RespElement) -> Result<Vec<u8>, EncodeError> {
    return match message {
        RespElement::Array(a) => {
            let mut encoded = format!("*{}\r\n", a.len());
            for element in a {
                let encoded_element = String::from_utf8(encode(element)?)?;
                println!("Appending {encoded_element} to {encoded}");
                encoded.push_str(&encoded_element);
            }
            println!("Encoded array: {encoded}");
            Ok(encoded.into_bytes())
        },
        RespElement::BulkString(s) => {
            let encoded = format!("${}\r\n{}\r\n", s.len(), s);
            println!("Encoded: {encoded:?}");
            Ok(encoded.into_bytes())
        },
        RespElement::EmptyBulkString => {
            let encoded = format!("$-1\r\n");
            println!("Encoded: {encoded:?}");
            Ok(encoded.into_bytes())
        },
        RespElement::SimpleString(s) => {
            let encoded = format!("+{}\r\n", s);
            println!("Encoded: {encoded:?}");
            Ok(encoded.into_bytes())
        }
        RespElement::Integer(i) => todo!("encode Integer"),
        _ => todo!("Encoding"),
    }
}