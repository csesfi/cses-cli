use std::fs::{self, File};
use std::io::Read;
use anyhow::Result;

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
}

impl Filesystem for ConcreteFilesystem {
    fn get_file(&self, filename: &str) -> Result<Vec<u8>> {
        let mut file = File::open(&filename)?;
        let metadata = fs::metadata(&filename)?;
        let mut buffer = vec![0; metadata.len() as usize];
        file.read(&mut buffer)?;

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;
    use super::*;
    use std::io::Write;

    #[test]
    fn can_read_file() {
        let mut path = temp_dir();
        path.push("test_file");
        let mut test_file = File::create(&path).unwrap();

        test_file.write_all(b"test content").unwrap();

        let filesystem = ConcreteFilesystem::default();
        let read_file = filesystem.get_file(path.to_str().unwrap()).unwrap();
        assert_eq!(read_file, b"test content");
    }
}