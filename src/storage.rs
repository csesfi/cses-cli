use anyhow::Result;
use miniserde::{json, Deserialize, Serialize};
use std::fs;
use std::path::Path;

const FILENAME: &str = "filestorage.json";

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct StorageData {
    token: Option<String>,
    course: Option<String>,
    task: Option<u64>,
    language: Option<String>,
    option: Option<String>,
    file: Option<String>,
}

impl StorageData {
    pub fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }
    pub fn get_course(&self) -> Option<&str> {
        self.course.as_deref()
    }
    pub fn get_task(&self) -> Option<u64> {
        self.task
    }
    pub fn get_language(&self) -> Option<&str> {
        self.language.as_deref()
    }
    pub fn get_option(&self) -> Option<&str> {
        self.option.as_deref()
    }
    pub fn get_file(&self) -> Option<&str> {
        self.file.as_deref()
    }
    pub fn set_token(&mut self, val: String) {
        self.token = Some(val);
    }
    pub fn set_course(&mut self, val: String) {
        self.course = Some(val);
    }
    pub fn set_task(&mut self, val: u64) {
        self.task = Some(val);
    }
    pub fn set_language(&mut self, val: String) {
        self.language = Some(val);
    }
    pub fn set_option(&mut self, val: String) {
        self.option = Some(val);
    }
    pub fn set_file(&mut self, val: String) {
        self.file = Some(val);
    }
}

#[derive(Default, Debug)]
pub struct FileStorage {
    data: StorageData,
}

impl FileStorage {
    pub fn new() -> Self {
        if !Path::new(FILENAME).exists() {
            return Default::default();
        };
        let data = fs::read_to_string(FILENAME).unwrap();
        let res = json::from_str(&data);
        match res {
            Ok(data) => FileStorage { data },
            Err(_) => Default::default(),
        }
    }
}

pub trait Storage {
    fn get(&self) -> &StorageData;
    fn get_mut(&mut self) -> &mut StorageData;
    fn save(&self) -> Result<()>;
    fn delete(&self) -> Result<()>;
}

impl Storage for FileStorage {
    fn get(&self) -> &StorageData {
        &self.data
    }
    fn get_mut(&mut self) -> &mut StorageData {
        &mut self.data
    }
    fn save(&self) -> Result<()> {
        Ok(fs::write(FILENAME, json::to_string(&self.data))?)
    }
    fn delete(&self) -> Result<()> {
        Ok(fs::remove_file(FILENAME)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setters_and_getters_work() {
        let mut storage_data: StorageData = Default::default();
        storage_data.set_token(String::from("token"));
        storage_data.set_course(String::from("course"));
        storage_data.set_task(42);
        storage_data.set_language(String::from("language"));
        storage_data.set_option(String::from("option"));
        storage_data.set_file(String::from("file"));
        assert_eq!(String::from("token"), storage_data.get_token().unwrap());
        assert_eq!(String::from("course"), storage_data.get_course().unwrap());
        assert_eq!(42, storage_data.get_task().unwrap());
        assert_eq!(
            String::from("language"),
            storage_data.get_language().unwrap()
        );
        assert_eq!(String::from("option"), storage_data.get_option().unwrap());
        assert_eq!(String::from("file"), storage_data.get_file().unwrap());
    }
}
