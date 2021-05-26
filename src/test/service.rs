use crate::{api::ApiResult, service::Login};
use crate::{CsesApi, Filesystem, Resources, Storage};
use anyhow::Result;

struct FakeCsesApi {}

impl CsesApi for FakeCsesApi {
    fn login(&self, _login: &Login) -> ApiResult<String> {
        todo!()
    }
}

struct FakeStorage {}

impl Storage for FakeStorage {
    fn get_username(&self) -> Option<&str> {
        todo!()
    }
    fn get_password(&self) -> Option<&str> {
        todo!()
    }
    fn get_token(&self) -> Option<&str> {
        todo!()
    }
    fn get_course(&self) -> Option<&str> {
        todo!()
    }
    fn get_task(&self) -> Option<&str> {
        todo!()
    }
    fn get_language(&self) -> Option<&str> {
        todo!()
    }
    fn get_option(&self) -> Option<&str> {
        todo!()
    }
    fn get_file(&self) -> Option<&str> {
        todo!()
    }
    fn set_username(&mut self, _val: String) {
        todo!()
    }
    fn set_password(&mut self, _val: String) {
        todo!()
    }
    fn set_token(&mut self, _val: String) {
        todo!()
    }
    fn set_course(&mut self, _val: String) {
        todo!()
    }
    fn set_task(&mut self, _val: String) {
        todo!()
    }
    fn set_language(&mut self, _val: String) {
        todo!()
    }
    fn set_option(&mut self, _val: String) {
        todo!()
    }
    fn set_file(&mut self, _val: String) {
        todo!()
    }
    fn save(&mut self) -> Result<()> {
        todo!()
    }
    fn delete(&mut self) -> Result<()> {
        todo!()
    }
}

struct FakeFilesystem {}

impl Filesystem for FakeFilesystem {}

fn fake_resources() -> Resources<(FakeCsesApi, FakeStorage, FakeFilesystem)> {
    Resources {
        api: FakeCsesApi {},
        storage: FakeStorage {},
        filesystem: FakeFilesystem {},
    }
}

#[test]
fn ping_works() {
    let mut _resources = fake_resources();
    // assert!(service.ping());
}
