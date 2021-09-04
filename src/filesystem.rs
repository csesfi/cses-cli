use anyhow::{anyhow, bail, Context, Result};
use base64::{decode, encode};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

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
    fn get_file(&self, path: &Path) -> Result<Vec<u8>>;
    fn file_exists(&self, path: &Path) -> bool;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    fn write_file(&self, filecontent: &[u8], path: &Path) -> Result<()>;
    fn get_filename(&self, path: &Path) -> Result<String>;
    fn encode_base64(&self, filecontent: &[u8]) -> String;
    fn decode_base64(&self, filecontent: &str) -> Result<Vec<u8>>;
}

impl Filesystem for ConcreteFilesystem {
    fn get_file(&self, path: &Path) -> Result<Vec<u8>> {
        (|| {
            let mut file = File::open(path)?;
            let length = file.metadata()?.len() as usize;
            if length > FILE_SIZE_LIMIT {
                bail!("File is too large (limit {} kB)", FILE_SIZE_LIMIT / 1024);
            }
            let mut buffer = Vec::with_capacity(length);
            file.read_to_end(&mut buffer)?;

            Ok(buffer)
        })()
        .context(format!("Failed reading file {}", path.display()))
    }

    fn file_exists(&self, path: &Path) -> bool {
        Path::new(path).exists()
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path).context(format!("Failed creating directory {}", path.display()))
    }

    fn write_file(&self, filecontent: &[u8], path: &Path) -> Result<()> {
        fs::write(path, filecontent).context(format!("Failed saving file to {}", path.display()))
    }

    fn get_filename(&self, path: &Path) -> Result<String> {
        Path::new(path)
            .file_name()
            .and_then(|f| f.to_str())
            .map(|f| f.to_owned())
            .ok_or_else(|| anyhow!("The path \"{}\" is not a valid file", path.display()))
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
    use std::fs::{remove_dir, remove_file};
    use std::io::Write;

    #[test]
    fn can_read_file() {
        let mut path = temp_dir();
        path.push("test_file1"); // TODO. Possibly use tempfile crate
        let mut test_file = File::create(&path).unwrap();

        test_file.write_all(b"test content").unwrap();
        test_file.sync_all().unwrap();

        let filesystem = ConcreteFilesystem::default();
        let read_file = filesystem.get_file(&path).unwrap();
        assert_eq!(read_file, b"test content");

        remove_file(&path).unwrap();
    }

    #[test]
    fn cannot_read_oversized_file() {
        let mut path = temp_dir();
        path.push("test_file2");
        let mut test_file = File::create(&path).unwrap();

        for _kb in 0..=FILE_SIZE_LIMIT / 1024 {
            test_file.write_all(&[0; 1024]).unwrap();
        }
        test_file.sync_all().unwrap();

        let filesystem = ConcreteFilesystem::default();
        let result = filesystem.get_file(&path);
        assert!(result.is_err());

        remove_file(&path).unwrap();
    }

    #[test]
    fn can_write_file() {
        let mut path = temp_dir();
        path.push("RgnDfAjXcbpeIvdSCkxm");
        let filesystem = ConcreteFilesystem::default();
        let content = vec![b'a', b'b', b'c'];

        filesystem.write_file(&content, &path).unwrap();
        assert_eq!(filesystem.get_file(&path).unwrap(), content);
        remove_file(&path).unwrap();
    }

    #[test]
    fn can_create_directory() {
        let mut path = temp_dir();
        path.push("SCkxm");
        path.push("RgnDf");
        let filesystem = ConcreteFilesystem::default();

        let result = filesystem.create_dir_all(&path);
        let result_2 = filesystem.create_dir_all(&path);
        let is_dir = path.is_dir();
        remove_dir(&path).unwrap();
        remove_dir(&path.parent().unwrap()).unwrap();
        assert!(matches!(result, Ok(_)));
        assert!(matches!(result_2, Ok(_)));
        assert!(is_dir);
    }

    #[test]
    fn cannot_create_directory_on_file() {
        let mut path = temp_dir();
        path.push("SCkwaefaxm");
        let filesystem = ConcreteFilesystem::default();

        let content = b"abc";
        filesystem.write_file(content, &path).unwrap();

        let result = filesystem.create_dir_all(&path);
        remove_file(&path).unwrap();
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn base64_encoding_works_correctly() {
        let filesystem = ConcreteFilesystem::default();
        let to_encode = b"tEstVAlu3";
        assert_eq!(filesystem.encode_base64(to_encode), "dEVzdFZBbHUz");
    }

    #[test]
    fn base64_decoding_works_correctly() {
        let filesystem = ConcreteFilesystem::default();
        let to_decode = "aGVsbG8gd29ybGQ=";
        let decoded = filesystem.decode_base64(to_decode).unwrap();
        let corrrect_result: Vec<u8> = vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];
        assert_eq!(decoded, corrrect_result);
    }
}
