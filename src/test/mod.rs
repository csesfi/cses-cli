mod submit;

use crate::api::CodeSubmit;
use crate::storage::StorageData;
use crate::{api::ApiResult, api::MockCsesApi, service::Login};
use crate::{CsesApi, Filesystem, Resources, Storage};
use anyhow::Result;
use std::path::PathBuf;

struct FakeCsesApi {}

impl CsesApi for FakeCsesApi {
    fn login(&self, _login: &Login) -> ApiResult<String> {
        todo!()
    }
    fn logout(&self, _token: &str) -> ApiResult<()> {
        todo!()
    }

    fn submit_task(
        &self,
        _token: &str,
        _course_id: &str,
        _task_id: u64,
        _submission: &CodeSubmit,
    ) -> ApiResult<u64> {
        todo!()
    }

    fn get_submit(
        &self,
        _token: &str,
        _course_id: &str,
        _task_id: u64,
        _submission_id: u64,
        _poll: bool,
    ) -> ApiResult<crate::entities::SubmissionInfo> {
        todo!()
    }
}

#[derive(Default, Debug)]
struct FakeStorage {
    data: StorageData,
    path: PathBuf,
}

impl Storage for FakeStorage {
    fn get(&self) -> &StorageData {
        &self.data
    }
    fn get_mut(&mut self) -> &mut StorageData {
        &mut self.data
    }
    fn save(&mut self) -> Result<()> {
        Ok(())
    }
    fn delete(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_path(&self) -> &std::path::Path {
        &self.path
    }
}

struct FakeFilesystem {}

impl Filesystem for FakeFilesystem {
    fn get_file(&self, _filename: &str) -> anyhow::Result<Vec<u8>> {
        Ok(b"test".to_vec())
    }

    fn encode_base64(&self, _filecontent: &[u8]) -> String {
        "testing".to_string()
    }

    fn decode_base64(&self, _filecontent: &str) -> anyhow::Result<Vec<u8>> {
        todo!()
    }
}

fn fake_resources() -> Resources<(FakeCsesApi, FakeStorage, FakeFilesystem)> {
    Resources {
        api: FakeCsesApi {},
        storage: Default::default(),
        filesystem: FakeFilesystem {},
    }
}

fn fake_resources_with_mock_api() -> Resources<(MockCsesApi, FakeStorage, FakeFilesystem)> {
    Resources {
        api: MockCsesApi::new(),
        storage: Default::default(),
        filesystem: FakeFilesystem {},
    }
}
