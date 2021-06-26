use super::{prompt_yes_no, Ui};
use crate::{command, entities::Scope, service, RP};
use anyhow::Result;
use std::io::Write;

pub fn get_examples(ui: &mut Ui<impl RP>, scope: &Scope, params: command::Examples) -> Result<()> {
    let test_cases = service::fetch_examples(&ui.res, scope, &params.task)?;
    if service::test_cases_exist(&ui.res, params.dir_name.as_deref()) {
        let overwrite_message = format!(
            "Test cases already present in the {}\n\
            Do you want to overwrite them? (yes/No)? ",
            format_dir_name(params.dir_name.as_deref())
        );
        if !prompt_yes_no(ui, &overwrite_message)? {
            return Ok(());
        }
    }
    service::create_dir_all(&ui.res, params.dir_name.as_deref())?;
    service::save_test_cases(&ui.res, test_cases, params.dir_name.as_deref())?;
    Ok(writeln!(
        ui.term,
        "Example test cases successfully saved to the directory {}",
        format_dir_name(params.dir_name.as_deref())
    )?)
}

fn format_dir_name(dir_name: Option<&str>) -> String {
    if let Some(dir_name) = dir_name {
        format!("directory {}", dir_name)
    } else {
        "current directory".to_owned()
    }
}
