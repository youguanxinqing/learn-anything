pub mod decode;
pub mod encode;

use bytes::BytesMut;
use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};
use thiserror::Error;

pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode: Sized {
    fn decode(buf: BytesMut) -> Result<Self, RespError>;
}

#[derive(Error, Debug, PartialEq)]
pub enum RespError {
    #[error("Invliad frame: {0}")]
    InvalidFrame(String),
    #[error("Invliad frame type: {0}")]
    InvalidFrameType(String),
    #[error("Invliad frame length: {0}")]
    InvalidFrameLength(isize),
    #[error("Frame is not complete")]
    NotComplete,
}

#[macro_export]
macro_rules! invalid_frame {
    ($($arg:tt)*) => {
        {
            let err = crate::resp::RespError::InvalidFrame(format!($($arg)*));
            err
        }
    };
}

#[macro_export]
macro_rules! invalid_frame_type {
    ($($arg:tt)*) => {
        {
            let err = crate::resp::RespError::InvalidFrameType(format!($($arg)*));
            err
        }
    };
}

#[macro_export]
macro_rules! invalid_frame_length {
    ($arg:tt) => {
        {
            let err = crate::resp::RespError::InvalidFrameLength($arg);
            err
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::resp::RespError;

    #[test]
    fn test_invalid_frame_macro() {
        let err = invalid_frame!("there is a err: {}", "yes!");
        assert_eq!(err, RespError::InvalidFrame(format!("there is a err: {}", "yes!")));
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(Vec<u8>),
    NullBulkString(NullBulkString),
    Array(Vec<RespFrame>),
    NullArray(NullArray),
    Null(RespNull),
    Boolean(bool),
    Double(Double),
    Map(Map),
    Set(Set),
}

impl RespFrame {
    fn from_bytes(buf: BytesMut) -> Result<Self, RespError> {
        if buf.len() < 1 {
            return Err(invalid_frame!("expect: need valid buf"));
        }

        let first_symbol = buf[0];
        match first_symbol {
            b'+' => SimpleString::decode(buf).map(|val| Self::SimpleString(val)),
            b'-' => SimpleError::decode(buf).map(|val| Self::Error(val)),
            b':' => Double::decode(buf).map(|val| Self::Double(val)),
            b'$' => {
                if buf == "$-1\r\n" {
                    Ok(Self::NullBulkString(NullBulkString::default()))
                } else {
                    Vec::<u8>::decode(buf).map(|val| Self::BulkString(val))
                }
            },
            b'*' => {
                if buf == "*-1\r\n" {
                    Ok(Self::NullArray(NullArray::default()))
                } else {
                    Vec::<RespFrame>::decode(buf).map(|val| Self::Array(val))
                }
            },
            b'_' => RespNull::decode(buf).map(|val| Self::Null(val)),
            b'#' => bool::decode(buf).map(|val| Self::Boolean(val)),
            b',' => Double::decode(buf).map(|val| Self::Double(val)),
            b'%' => Map::decode(buf).map(|val| Self::Map(val)),
            b'~' => Set::decode(buf).map(|val| Self::Set(val)),
            _ => Err(invalid_frame!("not support type: {}", first_symbol))
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct SimpleString(String);

impl SimpleString {
    pub fn new(v: impl Into<String>) -> Self {
        Self(v.into())
    }
}

impl From<&str> for SimpleString {
    fn from(value: &str) -> Self {
        SimpleString::new(value)
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct SimpleError(String);

#[derive(Eq, Hash, PartialEq, Default, Debug)]
pub struct RespNull;

#[derive(Eq, Hash, PartialEq, Default, Debug)]
pub struct NullArray;

#[derive(Eq, Hash, PartialEq, Default, Debug)]
pub struct NullBulkString;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Double(f64);

impl Deref for Double {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::cmp::Eq for Double {}

impl std::hash::Hash for Double {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}

#[derive(Eq, PartialEq)]
pub struct Map(HashMap<String, RespFrame>);

impl Map {
    pub fn new() -> Map {
        Self(HashMap::new())
    }
}

impl Deref for Map {
    type Target = HashMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::hash::Hash for Map {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}

#[derive(Eq, PartialEq)]
pub struct Set(HashSet<RespFrame>);

impl Set {
    fn new() -> Set {
        Set(HashSet::new())
    }
}

impl Deref for Set {
    type Target = HashSet<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Set {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::hash::Hash for Set {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}
