use crate::entities::{Scope, TestCase};
use crate::{CsesApi, Filesystem, Resources, Storage, RP};
use anyhow::{Context, Result};

pub fn fetch_test_cases(
    res: &mut Resources<impl RP>,
    scope: &Scope,
    task_id: &str,
) -> Result<Vec<TestCase>> {
    (|| -> Result<_> {
        let token = res.storage.get().get_token();
        let response = res.api.get_test_case_list(token, &scope, task_id)?;
        Ok(response.test_cases)
    })()
    .context("Failed querying test cases from the server.")
}

pub fn save_test_cases(
    res: &mut Resources<impl RP>,
    test_cases: Vec<TestCase>,
    dir_name: Option<&str>,
) -> Result<()> {
    (|| -> Result<_> {
        let path = make_path(dir_name);
        let mut case_num = 0;
        for case in test_cases.iter() {
            case_num += 1;
            res.filesystem.write_file(
                &res.filesystem.decode_base64(&case.input)?,
                &format_path(&path, case_num, "in"),
            )?;
            res.filesystem.write_file(
                &res.filesystem.decode_base64(&case.output)?,
                &format_path(&path, case_num, "out"),
            )?;
        }
        Ok(())
    })()
    .context("Failed saving test cases.")
}

pub fn test_cases_exist(res: &Resources<impl RP>, dir_name: Option<&str>) -> bool {
    let path = make_path(dir_name);
    let case_num = 1;
    res.filesystem
        .file_exists(&format_path(&path, case_num, "in"))
        && res
            .filesystem
            .file_exists(&format_path(&path, case_num, "out"))
}

fn make_path(dir_name: Option<&str>) -> String {
    let mut path = String::from("./");
    if let Some(d) = dir_name {
        path.push_str(d);
        path.push('/');
    }
    path
}
fn format_path(path: &str, case_num: u64, end: &str) -> String {
    format!("{}{}.{}", path, case_num, end)
}