use std::{collections::HashMap, io::BufRead, str::FromStr};

use crate::invalid_frame;

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
        return Err(RespError::InvalidFrameLength(buf.len() as isize));
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

        Ok(NullBulkString::default())
    }
}

// *<number-of-elements>\r\n<element-1>...<element-n>
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

        Ok(NullArray::default())
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

        Ok(RespNull::default())
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
        let splitter_start_pos = lookup_pos_at_first(&buf)
            .map_err(|_| invalid_frame!("lack of the first splitter"))?;
        let num = String::from_utf8(buf[1..splitter_start_pos].to_vec())
            .map_err(|e| invalid_frame!("extract number of entries err: {}", e))?;
        let num = num.parse::<usize>().map_err(|e| invalid_frame!("parse to usize err: {}", e))?;

        // parse entries
        let map = HashMap::with_capacity(num);
        

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

    use crate::resp::{Double, NullArray, NullBulkString, RespDecode, RespEncode, RespNull, SimpleString};

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

    #[test]
    fn test_null_bulk_string_decode() {
        let bytes = BytesMut::from("$-1\r\n");
        let null_bulk_string = NullBulkString::decode(bytes).unwrap();
        assert_eq!(null_bulk_string, NullBulkString::default());
    }

    #[test]
    fn test_null_array_decode() {
        let bytes = BytesMut::from("*-1\r\n");
        let null_array = NullArray::decode(bytes).unwrap();
        assert_eq!(null_array, NullArray::default());
    }

    #[test]
    fn test_resp_null_decode() {
        let bytes = BytesMut::from("_\r\n");
        let resp_null = RespNull::decode(bytes).unwrap();
        assert_eq!(resp_null, RespNull::default());
    }

    #[test]
    fn test_bool_decode() {
        let bytes = BytesMut::from("#t\r\n");
        let true_value = bool::decode(bytes).unwrap();
        assert_eq!(true_value, true);
        
        let bytes = BytesMut::from("#f\r\n");
        let false_value = bool::decode(bytes).unwrap();
        assert_eq!(false_value, false);
    }

    #[test]
    fn test_double_decode() {
        let bytes = BytesMut::from(",+12.1\r\n");
        let value = Double::decode(bytes).unwrap();
        assert_eq!(value, Double(12.1_f64));

        let bytes = BytesMut::from(",12.1\r\n");
        let value = Double::decode(bytes).unwrap();
        assert_eq!(value, Double(12.1_f64));

        let bytes = BytesMut::from(",-31.415\r\n");
        let value = Double::decode(bytes).unwrap();
        assert_eq!(value, Double(-31.415_f64));

        let bytes = BytesMut::from(",abcder\r\n");
        let value = Double::decode(bytes).unwrap();
        assert!(f64::is_nan(value.0));

        let bytes = BytesMut::from(",+inf\r\n");
        let value = Double::decode(bytes).unwrap();
        assert!(f64::is_sign_positive(value.0) && f64::is_infinite(value.0));

        let bytes = BytesMut::from(",-inf\r\n");
        let value = Double::decode(bytes).unwrap();
        assert!(f64::is_sign_negative(value.0) && f64::is_infinite(value.0));
    }
}
