use miniserde::{json, Deserialize, Serialize};
use std::fs;
use std::path::Path;

const FILENAME: &str = "filestorage.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct FileStorage {
    username: String,
    password: String,
    token: String,
    course: String,
    task: String,
    language: String,
    option: String,
    file: String,
}

impl FileStorage {
    pub fn new() -> Self {
        let empty = FileStorage {
            username: String::from(""),
            password: String::from(""),
            token: String::from(""),
            course: String::from(""),
            task: String::from(""),
            language: String::from(""),
            option: String::from(""),
            file: String::from(""),
        };
        if !Path::new(FILENAME).exists() {
            return empty;
        };
        let data = fs::read_to_string(FILENAME).unwrap();
        let res = json::from_str(&data);
        match res {
            Ok(fs) => fs,
            Err(_) => empty,
        }
    }
}

impl Default for FileStorage {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Storage {
    fn get_username(&self) -> String;
    fn get_password(&self) -> String;
    fn get_token(&self) -> String;
    fn get_course(&self) -> String;
    fn get_task(&self) -> String;
    fn get_language(&self) -> String;
    fn get_option(&self) -> String;
    fn get_file(&self) -> String;
    fn set_username(&mut self, val: String);
    fn set_password(&mut self, val: String);
    fn set_token(&mut self, val: String);
    fn set_course(&mut self, val: String);
    fn set_task(&mut self, val: String);
    fn set_language(&mut self, val: String);
    fn set_option(&mut self, val: String);
    fn set_file(&mut self, val: String);
    fn save(&self);
    fn delete(&self);
}

impl Storage for FileStorage {
    fn get_username(&self) -> String {
        self.username.to_string()
    }
    fn get_password(&self) -> String {
        self.password.to_string()
    }
    fn get_token(&self) -> String {
        self.token.to_string()
    }
    fn get_course(&self) -> String {
        self.course.to_string()
    }
    fn get_task(&self) -> String {
        self.task.to_string()
    }
    fn get_language(&self) -> String {
        self.language.to_string()
    }
    fn get_option(&self) -> String {
        self.option.to_string()
    }
    fn get_file(&self) -> String {
        self.file.to_string()
    }
    fn set_username(&mut self, val: String) {
        self.username = val;
    }
    fn set_password(&mut self, val: String) {
        self.password = val;
    }
    fn set_token(&mut self, val: String) {
        self.token = val;
    }
    fn set_course(&mut self, val: String) {
        self.course = val;
    }
    fn set_task(&mut self, val: String) {
        self.task = val;
    }
    fn set_language(&mut self, val: String) {
        self.language = val;
    }
    fn set_option(&mut self, val: String) {
        self.option = val;
    }
    fn set_file(&mut self, val: String) {
        self.file = val;
    }
    fn save(&self) {
        fs::write(FILENAME, json::to_string(&self)).unwrap();
    }
    fn delete(&self) {
        fs::remove_file(FILENAME).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setters_and_getters_work() {
        let mut storage = FileStorage {
            username: String::from(""),
            password: String::from(""),
            token: String::from(""),
            course: String::from(""),
            task: String::from(""),
            language: String::from(""),
            option: String::from(""),
            file: String::from(""),
        };
        storage.set_username(String::from("username"));
        storage.set_password(String::from("password"));
        storage.set_token(String::from("token"));
        storage.set_course(String::from("course"));
        storage.set_task(String::from("task"));
        storage.set_language(String::from("language"));
        storage.set_option(String::from("option"));
        storage.set_file(String::from("file"));
        assert_eq!(String::from("username"), storage.get_username());
        assert_eq!(String::from("password"), storage.get_password());
        assert_eq!(String::from("token"), storage.get_token());
        assert_eq!(String::from("course"), storage.get_course());
        assert_eq!(String::from("task"), storage.get_task());
        assert_eq!(String::from("language"), storage.get_language());
        assert_eq!(String::from("option"), storage.get_option());
        assert_eq!(String::from("file"), storage.get_file());
    }
}
