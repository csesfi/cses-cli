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

pub trait Filesystem {}

impl Filesystem for ConcreteFilesystem {}
