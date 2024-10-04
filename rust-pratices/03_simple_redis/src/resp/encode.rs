use core::f64;
use std::{
    collections::{HashMap, HashSet},
    usize,
};

use bytes::{BufMut, BytesMut};

use super::{
    Map, NullArray, NullBulkString, RespEncode, RespFrame, RespNull, Set, SimpleError, SimpleString,
};

impl RespEncode for RespFrame {
    fn encode(self) -> Vec<u8> {
        match self {
            RespFrame::SimpleString(data) => data.encode(),
            RespFrame::Error(data) => data.encode(),
            RespFrame::Integer(data) => data.encode(),
            RespFrame::BulkString(data) => data.encode(),
            RespFrame::NullBulkString(data) => data.encode(),
            RespFrame::Array(data) => data.encode(),
            RespFrame::NullArray(data) => data.encode(),
            RespFrame::Null(data) => data.encode(),
            RespFrame::Boolean(data) => data.encode(),
            RespFrame::Double(data) => data.encode(),
            RespFrame::Map(data) => data.encode(),
            RespFrame::Set(data) => data.encode(),
        }
    }
}

// Simple String: +OK\r\n
impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

// Simple Error: -Error message\r\n
impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}

// Integers: :[<+|->]<value>\r\n
impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self > 0 { "+" } else { "-" };
        format!(":{}{}\r\n", sign, self).into()
    }
}

// Bulk strings: $<length>\r\n<data>\r\n
// note: A bulk string represents a single binary string. 
impl RespEncode for Vec<u8> {
    fn encode(self) -> Vec<u8> {
        let mut bulk_string = BytesMut::new();
        bulk_string.put_slice(b"$");
        bulk_string.put_slice(self.len().to_string().as_bytes());
        bulk_string.put_slice(b"\r\n");
        bulk_string.put_slice(&self);
        bulk_string.put_slice(b"\r\n");
        bulk_string.to_vec()
    }
}

// Null bulk strings: $-1\r\n
impl RespEncode for NullBulkString {
    fn encode(self) -> Vec<u8> {
        "$-1\r\n".into()
    }
}

const BUF_LEN: usize = 4096;

// Arrays: *<number-of-elements>\r\n<element-1>...<element-n>
impl RespEncode for Vec<RespFrame> {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_LEN);
        buf.extend_from_slice(b"*");
        buf.extend_from_slice(self.len().to_string().as_bytes());
        for frame in self {
            buf.extend_from_slice(frame.encode().as_ref());
        }
        buf
    }
}

// Null arrays: *-1\r\n
impl RespEncode for NullArray {
    fn encode(self) -> Vec<u8> {
        "*-1\r\n".into()
    }
}

// Null: _\r\n
impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        "_\r\n".into()
    }
}

// Boolean: #<t|f>\r\n
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        format!("#{}\r\n", if self { "t" } else { "f" }).into()
    }
}

// Double: ,[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n
impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        let value = match self {
            x if x > f64::NEG_INFINITY && x < f64::INFINITY => format!("{}", self),
            x if x >= f64::INFINITY => "inf".to_string(),
            x if x <= f64::NEG_INFINITY => "-inf".to_string(),
            _ => "nan".into(),
        };
        format!(",{}\r\n", value).into()
    }
}

// Map: %<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>
impl RespEncode for Map {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_LEN);
        buf.extend_from_slice(b"%");
        buf.extend_from_slice(self.len().to_string().as_bytes());
        buf.extend_from_slice(b"\r\n");
        for (k, v) in self.0 {
            buf.extend_from_slice(SimpleString::new(k).encode().as_ref());
            buf.extend_from_slice(v.encode().as_ref());
        }

        buf
    }
}

// Set: ~<number-of-elements>\r\n<element-1>...<element-n>
impl RespEncode for Set {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_LEN);
        buf.extend_from_slice(b"~");
        buf.extend_from_slice(self.len().to_string().as_bytes());
        buf.extend_from_slice(b"\r\n");
        for frame in self.0 {
            buf.extend_from_slice(frame.encode().as_ref());
        }

        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::{Map, RespEncode, RespFrame, Set};

    #[test]
    fn test_vec_u8_encoding() {
        let bulk_string: Vec<u8> = "hello".into();
        let stream = bulk_string.encode();
        println!("encode result: {:?}", String::from_utf8_lossy(&stream));

        assert_eq!("$5\r\nhello\r\n".to_string().as_bytes(), stream);
    }

    #[test]
    fn test_double_encoding() {
        let v: f64 = std::f64::NAN;
        let stream = v.encode();
        println!("encode result: {:?}", String::from_utf8_lossy(&stream));

        assert_eq!(",nan\r\n".to_string().as_bytes(), stream);
        assert_eq!(",1.23\r\n".to_string().as_bytes(), 1.23.encode());
        assert_eq!(",inf\r\n".to_string().as_bytes(), f64::INFINITY.encode());
        assert_eq!(
            ",-inf\r\n".to_string().as_bytes(),
            f64::NEG_INFINITY.encode()
        );
    }

    #[test]
    fn test_map_encoding() {
        let mut m = Map::new();
        m.insert("one_key".into(), RespFrame::SimpleString("Hello".into()));
        m.insert("one_value".into(), RespFrame::SimpleString("World".into()));

        println!("{:?}", String::from_utf8_lossy(&m.encode()));
    }

    #[test]
    fn test_set_encoding() {
        let mut m = Set::new();
        m.insert(RespFrame::SimpleString("See".into()));
        m.insert(RespFrame::SimpleString("You".into()));

        println!("{:?}", String::from_utf8_lossy(&m.encode()));
    }
}
