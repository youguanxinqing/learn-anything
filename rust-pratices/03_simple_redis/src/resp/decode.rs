use super::{RespDecode, RespError, RespFrame, SimpleString};
use bytes::{Buf, BytesMut};

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

impl RespDecode for SimpleString {
    fn decode(buf: BytesMut) -> Result<Self, RespError> {
        if buf.len() < 3 {
            return Err(RespError::NotComplete);
        }

        if !buf.starts_with(b"+") {
            return Err(RespError::InvalidFrameType(format!(
                "expect: SimpleString(+), got: {:?}",
                buf
            )));
        }

        // search \r\n
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

        let s = String::from_utf8(buf[1..end].to_vec());
        match s {
            Err(e) => Err(RespError::InvalidFrame(e.to_string())),
            Ok(s) => Ok(SimpleString(s)),
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
