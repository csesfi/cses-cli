use anyhow::Result;
use miniserde::{json, Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct StorageData {
    token: Option<String>,
    course: Option<String>,
    task: Option<u64>,
    language: Option<String>,
    option: Option<String>,
    file: Option<String>,
}

#[cfg(unix)]
fn create_path() -> Result<PathBuf> {
    let mut path = PathBuf::from(std::env::var("HOME")?);
    path.push(".config/cses-cli/filestorage.json");
    return Ok(path);
}

#[cfg(target_os = "windows")]
fn create_path() -> Result<PathBuf> {
    let mut path = PathBuf::from(std::env::var("APPDATA")?);
    path.push_str("cses-cli\\filestorage.json");
    return Ok(path);
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
    pub fn new(test: bool) -> Result<FileStorage> {
        let filename;
        if test {
            filename = PathBuf::from("filestorage.json");
        } else {
            filename = create_path()?;
            fs::create_dir_all(filename.parent().unwrap())?;
        }
        if !filename.exists() {
            return Ok(Default::default());
        };
        let data = fs::read_to_string(filename)?;
        let res: StorageData = json::from_str(&data)?;
        Ok(FileStorage {data: res})
    }
}

pub trait Storage {
    fn get(&self) -> &StorageData;
    fn get_mut(&mut self) -> &mut StorageData;
    fn save(&mut self) -> Result<()>;
    fn delete(&mut self) -> Result<()>;
}

impl Storage for FileStorage {
    fn get(&self) -> &StorageData {
        &self.data
    }
    fn get_mut(&mut self) -> &mut StorageData {
        &mut self.data
    }
    fn save(&mut self) -> Result<()> {
        Ok(fs::write(create_path()?, json::to_string(&self.data))?)
    }
    fn delete(&mut self) -> Result<()> {
        Ok(fs::remove_file(create_path()?)?)
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
