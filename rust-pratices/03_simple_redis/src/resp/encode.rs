use bytes::{BufMut, BytesMut};

use super::{NullArray, NullBulkString, RespEncode, SimpleError, SimpleString, RespFrame};


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
        let sign = if self > 0 {"+"} else {"-"};
        format!(":{}{}", sign, self).into()
    }
}

// Bulk strings: $<length>\r\n<data>\r\n
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

// Arrays: *<number-of-elements>\r\n<element-1>...<element-n>
// impl RespEncode for Vec<RespFrame> {
//     fn encode(self) -> Vec<u8> {
//         let mut array = BytesMut::new();
//         array.put_slice(b"*");
//         array.put_slice(self.len().to_string().as_bytes());
//         array.put_slice(b"\r\n");
//         array.put_slice(self.iter().map(|item| {
//             match item {
//                 RespFrame::SimpleString(data) => data.encode().as_ref(),
//                 RespFrame::Error(data) => data.encode().as_ref(),
//                 RespFrame::Integer(data) => data.encode().as_ref(),
//             }
//         }));
//
//         array.into()
//     }
// }

// Null arrays: *-1\r\n
impl RespEncode for NullArray {
    fn encode(self) -> Vec<u8> {
        "*-1\r\n".into()
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::RespEncode;

    #[test]
    fn test_vec_u8_encoding() {
        let bulk_string: Vec<u8> = "hello".into();
        let stream = bulk_string.encode();
        println!("encode result: {:?}", String::from_utf8_lossy(&stream));

        assert_eq!("$5\r\nhello\r\n".to_string().as_bytes(), stream);
    }

}
