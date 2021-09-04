use std::path::{Path, PathBuf};

use crate::entities::{Scope, TestCase};
use crate::{CsesApi, Filesystem, Resources, Storage, RP};
use anyhow::{Context, Result};

pub fn fetch_samples(
    res: &Resources<impl RP>,
    scope: &Scope,
    task_id: &str,
) -> Result<Vec<TestCase>> {
    (|| -> Result<_> {
        let token = res.storage.get().get_token();
        let response = res.api.get_samples(token, scope, task_id)?;
        Ok(response.test_cases)
    })()
    .context("Failed querying sample test cases from the server.")
}

pub fn save_test_cases(
    res: &Resources<impl RP>,
    test_cases: Vec<TestCase>,
    dir_name: Option<&Path>,
) -> Result<()> {
    (|| -> Result<_> {
        let path = make_path(dir_name);
        let mut case_num = 0;
        for case in test_cases.iter() {
            case_num += 1;
            res.filesystem.write_file(
                &res.filesystem.decode_base64(&case.input)?,
                &format_path(path, case_num, "in"),
            )?;
            res.filesystem.write_file(
                &res.filesystem.decode_base64(&case.output)?,
                &format_path(path, case_num, "out"),
            )?;
        }
        Ok(())
    })()
    .context("Failed saving test cases.")
}

pub fn test_cases_exist(
    res: &Resources<impl RP>,
    case_count: u64,
    dir_name: Option<&Path>,
) -> Vec<String> {
    let path = make_path(dir_name);
    let mut files_found = Vec::<String>::new();
    for case_num in 1..=case_count {
        if res
            .filesystem
            .file_exists(&format_path(path, case_num, "in"))
        {
            files_found.push(format!("{}.in", case_num));
        }
        if res
            .filesystem
            .file_exists(&format_path(path, case_num, "out"))
        {
            files_found.push(format!("{}.out", case_num));
        }
    }

    files_found
}

pub fn create_dir_all(res: &Resources<impl RP>, dir_name: Option<&Path>) -> Result<()> {
    res.filesystem
        .create_dir_all(make_path(dir_name))
        .context("Failed creating directory for the test cases")
}

fn make_path(dir_name: Option<&Path>) -> &Path {
    match dir_name {
        Some(path) => path,
        None => Path::new("."),
    }
}
fn format_path(path: &Path, case_num: u64, end: &str) -> PathBuf {
    let mut path = path.to_owned();
    path.push(&format!("{}.{}", case_num, end));
    path
}
