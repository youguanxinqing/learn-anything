use std::{collections::HashMap, io::BufRead, str::FromStr};

use super::{
    Double, Map, NullArray, NullBulkString, RespDecode, RespError, RespFrame, RespNull, Set,
    SimpleError, SimpleString,
};
use bytes::{Buf, BytesMut};

impl RespDecode for RespFrame {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(b'+') => {
                SimpleString::decode(buf[1..].into()).map(|val| RespFrame::SimpleString(val))
            }
            Some(b'-') => SimpleError::decode(buf[1..].into()).map(|val| RespFrame::Error(val)),
            _ => todo!(),
        }
    }
}

fn lookup_pos_before_end(buf: &BytesMut) -> Result<usize, RespError> {
    let mut end = 0;
    for i in (1..buf.len()).rev() {
        if buf[i] == b'\n' && buf[i - 1] == b'\r' {
            end = i - 1;
            break;
        }
    }

    if end == 0 {
        return Err(RespError::NotComplete);
    }
    return Ok(end);
}

fn lookup_pos_at_first(buf: &BytesMut) -> Result<usize, RespError> {
    let mut end = 0;
    for i in 1..buf.len()-1 {
        if buf[i] == b'\r' && buf[i + 1] == b'\n' {
            end = i;
            break;
        }
    }

    if end == 0 {
        return Err(RespError::NotComplete);
    }
    return Ok(end)
}

fn validate_len_of_buf(buf: &BytesMut) -> Result<(), RespError> {
    if buf.len() < 3 {
        return Err(RespError::NotComplete);
    }
    Ok(())
}

fn validate_starts_with(buf: &BytesMut, prefix: &[u8], error_msg: &str) -> Result<(), RespError> {
    if !buf.starts_with(prefix) {
        return Err(RespError::InvalidFrameType(format!(
            "{}, but got: {:?}",
            error_msg, buf
        )));
    }
    Ok(())
}

impl RespDecode for SimpleString {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        validate_len_of_buf(&buf)?;
        validate_starts_with(&buf, b"+", "expect: SimpleString(+)")?;

        // search position before \r\n
        let end = lookup_pos_before_end(&buf)?;
        let s = String::from_utf8(buf[1..end].to_vec());
        match s {
            Err(e) => Err(RespError::InvalidFrame(e.to_string())),
            Ok(s) => Ok(SimpleString(s)),
        }
    }
}

impl RespDecode for SimpleError {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        validate_len_of_buf(&buf)?;
        validate_starts_with(&buf, b"-", "expect: SimpleError(-)")?;

        // search position before \r\n
        let end = lookup_pos_before_end(&buf)?;
        let s = String::from_utf8(buf[1..end].to_vec());
        match s {
            Err(e) => Err(RespError::InvalidFrame(e.to_string())),
            Ok(s) => Ok(SimpleError(s)),
        }
    }
}

// Integers: :[<+|->]<value>\r\n
impl RespDecode for i64 {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        validate_len_of_buf(&buf)?;
        validate_starts_with(&buf, b":", "expect: i64(:)")?;

        let end = lookup_pos_before_end(&buf)?;
        let s = String::from_utf8(buf[1..end].to_vec());
        match s {
            Err(e) => Err(RespError::InvalidFrame(e.to_string())),
            Ok(s) => Ok(s.parse::<i64>().map_err(|e| RespError::InvalidFrame(e.to_string()))?),
        }
    }
}

// Bulk strings: $<length>\r\n<data>\r\n
// note: A bulk string represents a single binary string. 
impl RespDecode for Vec<u8> {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        validate_len_of_buf(&buf)?;
        validate_starts_with(&buf, b"$", "expect: Vec<u8>($)")?;

        // the left element of first '\r\n' is total of elements
        let num_splitter_index = lookup_pos_at_first(&buf)
            .map_err(|e| RespError::InvalidFrame(format!("lack of the first splitter: {}", e.to_string())))?;
        let length = String::from_utf8(buf[1..num_splitter_index].to_vec())
            .map(|val| val.parse::<usize>())
            .map_err(|e| RespError::InvalidFrame(format!("parse length err: {}", e)))?;
        let length = length.map_err(|e| RespError::InvalidFrame(format!("parse length err: {}", e)))?;

        let start = num_splitter_index+2;
        return Ok(buf[start..start+length].to_vec());
    }
}

impl RespDecode for NullBulkString {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        validate_len_of_buf(&buf)?;

        if buf != "$-1\r\n" {
            return Err(RespError::InvalidFrameType(format!(
                "expect: NullBulkString($-1\r\n), got: {:?}",
                buf
            )));
        }

        Ok(NullBulkString)
    }
}

impl RespDecode for Vec<RespFrame> {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        todo!()
    }
}

impl RespDecode for NullArray {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        validate_len_of_buf(&buf)?;

        if buf != "*-1\r\n" {
            return Err(RespError::InvalidFrameType(format!(
                "expect: NullArray(*-1\r\n), got: {:?}",
                buf
            )));
        }

        Ok(NullArray)
    }
}

impl RespDecode for RespNull {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        validate_len_of_buf(&buf)?;

        if buf != "_\r\n" {
            return Err(RespError::InvalidFrameType(format!(
                "expect: RespNull(_\r\n), got: {:?}",
                buf
            )));
        }

        Ok(RespNull)
    }
}

impl RespDecode for bool {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        validate_len_of_buf(&buf)?;
        validate_starts_with(&buf, b"#", "expect: bool(,)")?;

        let end = lookup_pos_before_end(&buf)?;
        let s: &str = &String::from_utf8_lossy(&buf[1..end]).to_string();
        match s {
            "t" => Ok(true),
            "f" => Ok(false),
            _ => Err(RespError::InvalidFrame(format!("expect: bool(t|f)"))),
        }
    }
}

impl RespDecode for Double {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        validate_len_of_buf(&buf)?;
        validate_starts_with(&buf, b",", "expect: Double(,)")?;

        let end = lookup_pos_before_end(&buf)?;
        let s: &str = &String::from_utf8_lossy(&buf[1..end]);
        match FromStr::from_str(s) {
            Err(_) => Ok(Double(f64::NAN)),
            Ok(value) => Ok(Double(value)),
        }
    }
}


impl RespDecode for Map {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        validate_len_of_buf(&buf)?;
        validate_starts_with(&buf, b"%", "expect: Map(%)")?;

        // find number of k-v pairs
        let mut eof_of_num = 0;
        let length_of_buf = buf.to_vec().len();
        for i in 1..length_of_buf {
            if i + 1 >= length_of_buf {
                break;
            }
            if buf[i] == b'\r' && buf[i + 1] == b'\n' {
                eof_of_num = i;
                break;
            }
        }
        if eof_of_num == 0 {
            return Err(RespError::InvalidFrame(format!(
                "expect: Map(%<num>\r\n...)"
            )));
        }

        let num_of_pair = String::from_utf8_lossy(&buf[1..eof_of_num])
            .parse::<usize>()
            .map_err(|_| {
                RespError::InvalidFrame(format!(
                    "expect: valid unsigned number, but got {}",
                    String::from_utf8_lossy(&buf[1..eof_of_num]),
                ))
            })?;

        let entries_chunk = String::from_utf8_lossy(&buf[eof_of_num + 2..]).to_string();

        let mut map: HashMap<String, RespFrame> = HashMap::with_capacity(num_of_pair);
        entries_chunk.split("\r\n").for_each(|chunk| todo!());

        Ok(Map(map))
    }
}

impl RespDecode for Set {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        validate_len_of_buf(&buf)?;
        validate_starts_with(&buf, b"~", "expect: Set(~)")?;

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use crate::resp::{RespDecode, SimpleString};

    #[test]
    fn test_string_decode() {
        let bytes = BytesMut::from("+ok\r\n");
        let simple_string = SimpleString::decode(bytes).unwrap();
        assert_eq!(simple_string, SimpleString::from("ok"));
    }

    #[test]
    fn test_i64_decode() {
        let bytes = BytesMut::from(":+12\r\n");
        let num = i64::decode(bytes).unwrap();
        assert_eq!(num, 12);

        let bytes = BytesMut::from(":-121\r\n");
        let num = i64::decode(bytes).unwrap();
        assert_eq!(num, -121);
    }

    #[test]
    fn test_bulk_string_decode() {
        let bytes = BytesMut::from("$12\r\nhello world!\r\n");
        let bulk_string = Vec::<u8>::decode(bytes).unwrap();
        assert_eq!(bulk_string, Vec::from("hello world!"));
    }
}
