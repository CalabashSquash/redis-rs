use nom::{bytes::complete::{tag, take_while_m_n}, character::{complete::{self, alpha0, alphanumeric0, one_of}, is_alphanumeric}, complete::take, multi::{count, many0, many_m_n, many_till, separated_list0}, sequence::terminated, Err, IResult};

#[derive(Debug, PartialEq, Eq)]
pub enum RespElement<'a> {
    SimpleString(&'a str),
    BulkString(&'a str),
    EmptyBulkString,
    Array(Vec<RespElement<'a>>),
    Integer(u64),
}

// // We need OptionalRespElement because there's a possibility of an empty string, which is BulkString None
// #[derive(Debug, PartialEq, Eq)]
// pub enum OptionalRespElement<'a> {
//     SimpleString(Option<&'a str>),
//     BulkString(Option<&'a str>),
//     Array(Option<Vec<OptionalRespElement<'a>>>),
//     Integer(Option<u64>)
// }


pub fn decode<'a>(message: &'a String) -> RespElement<'a> {
    // let (remaining, first) = parse_element("+OK\r\n").unwrap();
    // let (remaining, first) = parse_element("$13\r\nHello, World!\r\n").unwrap();
    // println!("message: {:#?}", message);
    // let (remaining, first) = parse_element("*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$4\r\nabcd\r\n").unwrap();
    // println!("Remaining, first {remaining:?}, \"{first:?}\"");
    // let (remaining, second) = parse_element(remaining).unwrap();
    // println!("Remaining, second {remaining:?}, \"{second:?}\"");
    // println!("done");
    //let message = message.clone();
    let (remaining, decoded) = parse_element(message.as_str()).unwrap();
    if remaining.len() > 0 {
        todo!("Have to handle if more than one RESP object given");
    }

    decoded
    // match message.as_str() {
    //     "*1\r\n$4\r\nPING\r\n" => String::from("PING"),
    //     "*1\r\n$4\r\nDING\r\n" => String::from("DING"),
    //     _ => String::from("ERR_NOT_FOUND")
    // }
}

fn parse(input: &str) -> IResult<&str, &str> {


    Ok((input, ""))
}

fn parse_element(input: &str) -> IResult<&str, RespElement> {
    let (remaining, parsed) = one_of("*+$:")(input)?;
    return match parsed {
        '+' => parse_simple_string(remaining),
        '$' => parse_bulk_string(remaining),
        ':' => parse_integer(remaining),
        '*' => parse_array(remaining),
        _ => panic!("Should never happen because one_of should return error above")
    };
}

/**
 * Simple strings start with +, followed by the string, followed by \r\n. They can not have \r\n in the string because that would mark the end of the string.
 * e.g. +Hello, World!\r\n would be "Hello, World!"
 */
fn parse_simple_string(input: &str) -> IResult<&str, RespElement> {
    println!("Input: {input:?}");
    let (remaining, parsed) = many_till(alphanumeric0, tag("\r\n"))(input)?;

    Ok((remaining, RespElement::SimpleString(parsed.0[0])))
}

/**
 * Bulk strings start with $, followed by length, followed by \r\n, followed by the string, followed by \r\n.
 * e.g. $13\r\nHello, World!\r\n would be "Hello, World!"
 */
fn parse_bulk_string(input: &str) -> IResult<&str, RespElement> {
    let (remaining, length) = complete::u32(input)?;
    let (remaining, _) = tag("\r\n")(remaining)?;
    // We know the exact length, so can split there.
    let (bulk_string, remaining) = remaining.split_at(length as usize);
    let (remaining, _) = tag("\r\n")(remaining)?;

    Ok((remaining, RespElement::BulkString(bulk_string)))
}

fn parse_integer(input: &str) -> IResult<&str, RespElement> {
    let (remaining, number) = complete::u64(input)?;
    let (remaining, _) = tag("\r\n")(remaining)?;

    Ok((remaining, RespElement::Integer(number)))
}

fn parse_array(input: &str) -> IResult<&str, RespElement> {
    let (remaining, length) = complete::u32(input)?;
    let (remaining, _) = tag("\r\n")(remaining)?;
    let (remaining, parsed) = many_m_n(length as usize, length as usize, parse_element)(remaining)?;
    println!("remaining: {remaining:?}, parsed: {parsed:?}");
    Ok((remaining, RespElement::Array(parsed)))
}

// fn parse_array(input: &str) -> IResult<&str, char> {
//     one_of("*+$:")(input)
// }