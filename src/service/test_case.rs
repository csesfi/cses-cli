use crate::entities::{Scope, TestCase};
use crate::{CsesApi, Filesystem, Resources, Storage, RP};
use anyhow::{Context, Result};

pub fn get_test_cases(res: &mut Resources<impl RP>, scope: &Scope, task_id: &str) -> Result<()> {
    let test_cases = fetch_test_cases(res, scope, task_id)?;
    save_test_cases(res, test_cases)?;
    Ok(())
}

fn fetch_test_cases(
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

fn save_test_cases(res: &mut Resources<impl RP>, test_cases: Vec<TestCase>) -> Result<()> {
    let mut case_num = 0;
    for case in test_cases.iter() {
        case_num += 1;
        res.filesystem.write_file(
            &res.filesystem.decode_base64(&case.input)?,
            &format!("input{}", case_num),
        )?;
        res.filesystem.write_file(
            &res.filesystem.decode_base64(&case.output)?,
            &format!("output{}", case_num),
        )?;
    }
    Ok(())
}
