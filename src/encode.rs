use crate::decode::{OptionalRespElement};


pub fn encode(message: &OptionalRespElement) -> Vec<u8> {
    return match message {
        OptionalRespElement::Array(Some(a)) => todo!("Encode array"),
        OptionalRespElement::BulkString(Some(s)) => {
            let encoded = format!("${}\r\n{}\r\n", s.len(), s);
            println!("Encoded: {encoded:?}");
            encoded.into_bytes()
        },
        OptionalRespElement::BulkString(None) => {
            let encoded = format!("$-1\r\n");
            println!("Encoded: {encoded:?}");
            encoded.into_bytes()
        },
        OptionalRespElement::SimpleString(Some(s)) => {
            let encoded = format!("+{}\r\n", s);
            println!("Encoded: {encoded:?}");
            encoded.into_bytes()
        }
        OptionalRespElement::Integer(i) => todo!("encode Integer"),
        _ => todo!("Encoding"),
    }
}