use anyhow::Result;
use miniserde::{json, Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct StorageData {
    token: Option<String>,
    course: Option<String>,
}

#[cfg(unix)]
fn create_path() -> Result<PathBuf> {
    let mut path = PathBuf::from(std::env::var("HOME")?);
    path.push(".config/cses-cli/filestorage.json");
    Ok(path)
}

#[cfg(windows)]
fn create_path() -> Result<PathBuf> {
    let mut path = PathBuf::from(std::env::var("APPDATA")?);
    path.push("cses-cli\\filestorage.json");
    Ok(path)
}

impl StorageData {
    pub fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }
    pub fn get_course(&self) -> Option<&str> {
        self.course.as_deref()
    }
    pub fn set_token(&mut self, val: String) {
        self.token = Some(val);
    }
    pub fn set_course(&mut self, val: String) {
        self.course = Some(val);
    }
}

#[derive(Default, Debug)]
pub struct FileStorage {
    data: StorageData,
    path: PathBuf,
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
            return Ok(FileStorage {
                data: Default::default(),
                path: filename,
            });
        }
        let data = fs::read_to_string(&filename)?;
        let res: StorageData = json::from_str(&data)?;
        Ok(FileStorage {
            data: res,
            path: filename,
        })
    }
}

pub trait Storage {
    fn get(&self) -> &StorageData;
    fn get_mut(&mut self) -> &mut StorageData;
    fn save(&mut self) -> Result<()>;
    fn delete(&mut self) -> Result<()>;
    fn get_path(&self) -> &Path;
}

impl Storage for FileStorage {
    fn get(&self) -> &StorageData {
        &self.data
    }
    fn get_mut(&mut self) -> &mut StorageData {
        &mut self.data
    }
    fn save(&mut self) -> Result<()> {
        Ok(fs::write(&self.path, json::to_string(&self.data))?)
    }
    fn delete(&mut self) -> Result<()> {
        Ok(fs::remove_file(&self.path)?)
    }

    fn get_path(&self) -> &Path {
        &self.path
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
        assert_eq!(String::from("token"), storage_data.get_token().unwrap());
        assert_eq!(String::from("course"), storage_data.get_course().unwrap());
    }
}
