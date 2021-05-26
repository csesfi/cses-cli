use crate::{api::ApiResult, service::Login};
use crate::{CsesApi, Filesystem, Resources, Storage};
use anyhow::Result;

struct FakeCsesApi {}

impl CsesApi for FakeCsesApi {
    fn login(&self, _login: &Login) -> ApiResult<String> {
        todo!()
    }
    fn logout(&self, _token: &str) -> ApiResult<()> {
        todo!()
    }
}

#[derive(Default, Debug)]
pub struct FakeStorage {
    token: Option<String>,
    course: Option<String>,
    task: Option<String>,
    language: Option<String>,
    option: Option<String>,
    file: Option<String>,
}

impl Storage for FakeStorage {
    fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }
    fn get_course(&self) -> Option<&str> {
        self.course.as_deref()
    }
    fn get_task(&self) -> Option<&str> {
        self.task.as_deref()
    }
    fn get_language(&self) -> Option<&str> {
        self.language.as_deref()
    }
    fn get_option(&self) -> Option<&str> {
        self.option.as_deref()
    }
    fn get_file(&self) -> Option<&str> {
        self.file.as_deref()
    }
    fn set_token(&mut self, val: String) {
        self.token = Some(val);
    }
    fn set_course(&mut self, val: String) {
        self.course = Some(val);
    }
    fn set_task(&mut self, val: String) {
        self.task = Some(val);
    }
    fn set_language(&mut self, val: String) {
        self.language = Some(val);
    }
    fn set_option(&mut self, val: String) {
        self.option = Some(val);
    }
    fn set_file(&mut self, val: String) {
        self.file = Some(val);
    }
    fn save(&mut self) -> Result<()> {
        Ok(())
    }
    fn delete(&mut self) -> Result<()> {
        Ok(())
    }
}

struct FakeFilesystem {}

impl Filesystem for FakeFilesystem {}

fn fake_resources() -> Resources<(FakeCsesApi, FakeStorage, FakeFilesystem)> {
    Resources {
        api: FakeCsesApi {},
        storage: Default::default(),
        filesystem: FakeFilesystem {},
    }
}

#[test]
fn ping_works() {
    let _resources = fake_resources();
    // assert!(service.ping());
}
