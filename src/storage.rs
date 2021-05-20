pub struct FileStorage {}

impl FileStorage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for FileStorage {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Storage {}

impl Storage for FileStorage {}
