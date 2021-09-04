use super::{util::prompt_yes_no, Ui};
use crate::{command, entities::Scope, service, RP};
use anyhow::Result;
use std::{io::Write, path::Path};

pub fn get_samples(ui: &mut Ui<impl RP>, scope: &Scope, params: command::Samples) -> Result<()> {
    let test_cases = service::fetch_samples(&ui.res, scope, &params.task)?;
    let existing_files =
        service::test_cases_exist(&ui.res, test_cases.len() as u64, params.dir_name.as_deref());
    if !existing_files.is_empty() {
        let mut overwrite_message = format!(
            "The following files are already present in the {}:\n",
            format_dir_name(params.dir_name.as_deref())
        );
        for file in existing_files {
            overwrite_message.push_str(format!("  {}\n", file).as_str());
        }
        overwrite_message.push_str("Do you want to overwrite them? (yes/No) ");
        if !prompt_yes_no(ui, &overwrite_message)? {
            return Ok(());
        }
    }
    service::create_dir_all(&ui.res, params.dir_name.as_deref())?;
    let amount = test_cases.len();
    service::save_test_cases(&ui.res, test_cases, params.dir_name.as_deref())?;
    Ok(writeln!(
        ui.term,
        "{} sample test cases successfully saved to the {}",
        amount,
        format_dir_name(params.dir_name.as_deref())
    )?)
}

fn format_dir_name(dir_name: Option<&Path>) -> String {
    if let Some(dir_name) = dir_name {
        format!("directory {}", dir_name.display())
    } else {
        "current directory".to_owned()
    }
}
