mod submit;
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::api::{ApiResult, CodeSubmit, LoginResponse, MockCsesApi};
use crate::entities::{
    CourseList, Scope, ScopeContent, SubmissionInfo, SubmissionList, TaskStatement,
    TemplateResponse, TestCaseList, UserOutline,
};
use crate::storage::StorageData;
use crate::{CsesApi, Filesystem, Resources, Storage};

struct FakeCsesApi {}

impl CsesApi for FakeCsesApi {
    fn login(&self) -> ApiResult<LoginResponse> {
        todo!()
    }
    fn login_status(&self, _token: &str) -> ApiResult<UserOutline> {
        todo!()
    }
    fn logout(&self, _token: &str) -> ApiResult<()> {
        todo!()
    }

    fn submit_task<'a>(
        &self,
        _token: &str,
        _scope: &Scope,
        _task_id: Option<&'a str>,
        _submission: &CodeSubmit,
    ) -> ApiResult<SubmissionInfo> {
        todo!()
    }

    fn get_submit(
        &self,
        _token: &str,
        _scope: &Scope,
        _submission_id: u64,
        _poll: bool,
    ) -> ApiResult<SubmissionInfo> {
        todo!()
    }

    fn get_submit_list(
        &self,
        _token: &str,
        _scope: &Scope,
        _task_id: &str,
    ) -> ApiResult<SubmissionList> {
        todo!()
    }

    fn get_courses(&self, _token: Option<&str>) -> ApiResult<CourseList> {
        todo!()
    }

    fn get_template<'a>(
        &self,
        _token: Option<&'a str>,
        _scope: &Scope,
        _task_id: Option<&'a str>,
        _language: Option<&'a str>,
        _file: Option<&'a str>,
    ) -> ApiResult<TemplateResponse> {
        todo!()
    }

    fn get_content<'a>(&self, _token: Option<&'a str>, _scope: &Scope) -> ApiResult<ScopeContent> {
        todo!()
    }

    fn get_task_statement<'a>(
        &self,
        _token: Option<&'a str>,
        _scope: &Scope,
        _task_id: &str,
    ) -> ApiResult<TaskStatement> {
        todo!()
    }

    fn get_samples<'a>(
        &self,
        _token: Option<&'a str>,
        _scope: &Scope,
        _task_id: &str,
    ) -> ApiResult<TestCaseList> {
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
    fn get_file(&self, _path: &Path) -> anyhow::Result<Vec<u8>> {
        Ok(b"test".to_vec())
    }

    fn get_filename(&self, _path: &Path) -> Result<String> {
        Ok("extracted_filename".to_owned())
    }

    fn encode_base64(&self, _filecontent: &[u8]) -> String {
        "testing".to_string()
    }

    fn decode_base64(&self, _filecontent: &str) -> anyhow::Result<Vec<u8>> {
        todo!()
    }

    fn file_exists(&self, _path: &Path) -> bool {
        todo!();
    }

    fn create_dir_all(&self, _path: &Path) -> Result<()> {
        todo!();
    }

    fn write_file(&self, _filecontent: &[u8], _path: &Path) -> Result<()> {
        todo!();
    }
}

#[allow(unused)]
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
