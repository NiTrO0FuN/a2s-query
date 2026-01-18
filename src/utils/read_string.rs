use std::io::Cursor;

use crate::errors::Error;
use byteorder::ReadBytesExt;

pub trait ReadString {
    // Read a null terminated string
    fn read_string(&mut self) -> Result<String, Error>;
}

impl ReadString for Cursor<Vec<u8>> {
    fn read_string(&mut self) -> Result<String, Error> {
        let mut res: Vec<u8> = Vec::new();
        let size = self.get_ref().len();
        while (self.position() as usize) < size {
            let byte = self.read_u8()?;
            if byte == 0 {
                break;
            }
            res.push(byte);
        }
        Ok(String::from_utf8_lossy(&res).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_string_simple() {
        let data = b"hello\x00world\x00";
        let mut cursor = Cursor::new(data.to_vec());
        assert_eq!(cursor.read_string().unwrap(), "hello");
        assert_eq!(cursor.read_string().unwrap(), "world");
    }

    #[test]
    fn test_read_string_empty() {
        let data = b"\x00";
        let mut cursor = Cursor::new(data.to_vec());
        assert_eq!(cursor.read_string().unwrap(), "");
    }

    #[test]
    fn test_read_string_no_null_at_end() {
        let data = b"hello";
        let mut cursor = Cursor::new(data.to_vec());
        assert_eq!(cursor.read_string().unwrap(), "hello");
    }

    #[test]
    fn test_read_string_with_invalid_utf8() {
        let data = b"\xff\xfe\x00";
        let mut cursor = Cursor::new(data.to_vec());
        assert_eq!(cursor.read_string().unwrap(), "��");
    }

    #[test]
    fn test_read_string_sequential() {
        let data = b"first\x00second\x00third";
        let mut cursor = Cursor::new(data.to_vec());
        assert_eq!(cursor.read_string().unwrap(), "first");
        assert_eq!(cursor.read_string().unwrap(), "second");
        assert_eq!(cursor.read_string().unwrap(), "third");
    }

    #[test]
    fn test_read_string_at_eof() {
        let data = b"end";
        let mut cursor = Cursor::new(data.to_vec());
        cursor.set_position(3);
        assert_eq!(cursor.read_string().unwrap(), "");
    }
}
