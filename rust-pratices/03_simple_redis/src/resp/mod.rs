pub mod decode;
pub mod encode;


use std::collections::{HashMap, HashSet};
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
    Double(f64),
    Map(HashMap<String, RespFrame>),
    Set(HashSet<RespFrame>),
}

pub struct SimpleString(String);
pub struct SimpleError(String);
pub struct RespNull;
pub struct NullArray;
pub struct NullBulkString;

