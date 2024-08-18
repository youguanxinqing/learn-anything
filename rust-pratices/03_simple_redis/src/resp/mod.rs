pub mod decode;
pub mod encode;


use std::{collections::{HashMap, HashSet}, ops::{Deref, DerefMut}};
use bytes::{BytesMut};

trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

trait RespDecode {
    fn decode(buf: Self) -> Result<RespFrame, String>;
}

impl RespDecode for BytesMut {
    fn decode(buf: Self) -> Result<RespFrame, String> {
        todo!()
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

#[derive(Eq, Hash, PartialEq)]
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

#[derive(Eq, Hash, PartialEq)]
pub struct RespNull;

#[derive(Eq, Hash, PartialEq)]
pub struct NullArray;

#[derive(Eq, Hash, PartialEq)]
pub struct NullBulkString;

pub struct Double(f64);

impl Deref for Double {
    type Target = f64;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::cmp::PartialEq for Double {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl std::cmp::Eq for Double {
    fn assert_receiver_is_total_eq(&self) {
        todo!()
    }
}

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
