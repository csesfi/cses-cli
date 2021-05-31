use anyhow::{Result, bail};
use base64::{decode, encode};
use std::fs::{self, File};
use std::io::Read;

const FILE_SIZE_LIMIT: usize = 128 * 1024;

pub struct ConcreteFilesystem {}

impl ConcreteFilesystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ConcreteFilesystem {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Filesystem {
    fn get_file(&self, filename: &str) -> Result<Vec<u8>>;
    fn encode_base64(&self, filecontent: &[u8]) -> String;
    fn decode_base64(&self, filecontent: &str) -> Result<Vec<u8>>;
}

impl Filesystem for ConcreteFilesystem {
    fn get_file(&self, filename: &str) -> Result<Vec<u8>> {
        let mut file = File::open(&filename)?;
        let length = fs::metadata(&filename)?.len() as usize;
        if length > FILE_SIZE_LIMIT {
            bail!("File is too large (limit {} kB)", FILE_SIZE_LIMIT / 1024);
        }
        let mut buffer = Vec::with_capacity(length);
        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    fn encode_base64(&self, filecontent: &[u8]) -> String {
        encode(filecontent)
    }

    fn decode_base64(&self, filecontent: &str) -> Result<Vec<u8>> {
        Ok(decode(filecontent)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    use std::fs::remove_file;
    use std::io::Write;

    #[test]
    fn can_read_file() {
        let mut path = temp_dir();
        path.push("test_file1"); // TODO. Possibly use tempfile crate
        let mut test_file = File::create(&path).unwrap();

        test_file.write_all(b"test content").unwrap();
        test_file.sync_all().unwrap();

        let filesystem = ConcreteFilesystem::default();
        let read_file = filesystem.get_file(path.to_str().unwrap()).unwrap();
        assert_eq!(read_file, b"test content");

        remove_file(&path).unwrap();
    }

    #[test]
    fn cannot_read_oversized_file()  {
        let mut path = temp_dir();
        path.push("test_file2");
        let mut test_file = File::create(&path).unwrap();

        for _kb in 0..=FILE_SIZE_LIMIT / 1024 {
            test_file.write_all(&[0; 1024]).unwrap();
        }
        test_file.sync_all().unwrap();

        let filesystem = ConcreteFilesystem::default();
        let result = filesystem.get_file(path.to_str().unwrap());
        assert!(result.is_err());

        remove_file(&path).unwrap();
    }

    #[test]
    fn base64_encoding_works_correctly() {
        let to_encode = b"tEstVAlu3";
        assert_eq!(encode(&to_encode), "dEVzdFZBbHUz");
    }

    #[test]
    fn base64_decoding_works_correctly() {
        let to_decode = "aGVsbG8gd29ybGQ=";
        assert_eq!(
            decode(&to_decode),
            Ok(vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100])
        );
    }
}
