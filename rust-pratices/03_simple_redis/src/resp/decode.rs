use std::{fmt::format, str::FromStr};

use super::{
    Double, NullArray, NullBulkString, RespDecode, RespError, RespFrame, RespNull, SimpleError,
    SimpleString,
};
use bytes::{Buf, BufMut, BytesMut};

impl RespDecode for RespFrame {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(b'+') => {
                todo!()
            }
            _ => todo!(),
        }
    }
}

fn lookup_pos_before_end(buf: &BytesMut) -> usize {
    let mut end = 0;
    for i in (1..buf.len()).rev() {
        if buf[i] == b'\n' && buf[i - 1] == b'\r' {
            end = i - 1;
            break;
        }
    }
    return end;
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
        let end = lookup_pos_before_end(&buf);
        if end == 0 {
            return Err(RespError::NotComplete);
        }

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
        let end = lookup_pos_before_end(&buf);
        if end == 0 {
            return Err(RespError::NotComplete);
        }

        let s = String::from_utf8(buf[1..end].to_vec());
        match s {
            Err(e) => Err(RespError::InvalidFrame(e.to_string())),
            Ok(s) => Ok(SimpleError(s)),
        }
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

impl RespDecode for Double {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        validate_len_of_buf(&buf)?;
        validate_starts_with(&buf, b",", "expect: Double(,)")?;

        let end = lookup_pos_before_end(&buf);
        if end == 0 {
            return Err(RespError::NotComplete);
        }

        let s: &str = &String::from_utf8_lossy(&buf[1..end]);
        match FromStr::from_str(s) {
            Err(_) => Ok(Double(f64::NAN)),
            Ok(value) => Ok(Double(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use crate::resp::{RespDecode, SimpleString};

    #[test]
    fn test_string_decoding() {
        let bytes = BytesMut::from("+ok\r\n");
        let simple_string = SimpleString::decode(bytes).unwrap();
        assert_eq!(simple_string, SimpleString::from("ok"));
    }
}
