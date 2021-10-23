use std::convert::TryInto;
use std::io::Write;
use std::path::{Path, MAIN_SEPARATOR};

use anyhow::{anyhow, Result};

use super::util::prompt_yes_no;
use super::Ui;
use crate::api::ApiError;
use crate::command::{Template, TemplateQuery};
use crate::entities::{Scope, ScopeItem, ScopeItemRaw};
use crate::{service, RP};

pub fn get_template(ui: &mut Ui<impl RP>, scope: &Scope, command: Template) -> Result<()> {
    let language = command.language.as_deref();
    match command.query {
        TemplateQuery::Single { task_id, filename } => {
            if let Some(saved) =
                get_one(ui, scope, task_id.as_deref(), language, filename.as_deref())?
            {
                writeln!(
                    ui.term,
                    "Template file was successfully saved to .{}{}",
                    MAIN_SEPARATOR, &saved
                )?
            }
        }
        TemplateQuery::Section(index) => {
            let content = service::scope_content(&mut ui.res, scope)?;
            let section = index
                .checked_sub(1)
                .and_then(|i| i.try_into().ok())
                .and_then(|i: usize| content.sections.get(i))
                .ok_or_else(|| anyhow!("No such section"))?;
            writeln!(ui.term, "Downloading all templates from {}", section.header)?;
            get_many(ui, scope, language, section.list.iter())?;
        }
        TemplateQuery::All => {
            let content = service::scope_content(&mut ui.res, scope)?;
            let items = content
                .sections
                .iter()
                .map(|section| section.list.iter())
                .flatten();
            if items.clone().count() > 200 {
                anyhow::bail!(
                    "{} is too large to download all templates",
                    match scope {
                        Scope::Course(_) => "Course",
                        Scope::Contest(_) => "Contest",
                    }
                );
            }
            get_many(ui, scope, language, items)?;
        }
    }
    Ok(())
}

fn get_one(
    ui: &mut Ui<impl RP>,
    scope: &Scope,
    task_id: Option<&str>,
    language: Option<&str>,
    filename: Option<&str>,
) -> Result<Option<String>> {
    let template_response = service::get_template(&mut ui.res, scope, task_id, language, filename)?;
    if service::file_exists(&ui.res, Path::new(&template_response.filename)) {
        let overwrite_message = format!(
            "A file .{}{} already exists.\n\
            Do you want to overwrite it with the new template? (yes/No) ",
            MAIN_SEPARATOR, &template_response.filename
        );
        if !prompt_yes_no(ui, &overwrite_message)? {
            return Ok(None);
        }
    }
    service::save_response(&mut ui.res, &template_response)?;
    Ok(Some(template_response.filename))
}

fn get_many<'a>(
    ui: &mut Ui<impl RP>,
    scope: &Scope,
    language: Option<&str>,
    items: impl Iterator<Item = &'a ScopeItemRaw>,
) -> Result<()> {
    let mut found = false;
    for item in items {
        match item.as_enum()? {
            ScopeItem::Task { name, id, .. } => {
                match get_one(ui, scope, Some(id), language, None) {
                    Ok(decision) => {
                        found = true;
                        if let Some(saved) = decision {
                            writeln!(
                                ui.term,
                                "Template for task {} saved to .{}{}",
                                name, MAIN_SEPARATOR, &saved
                            )?
                        }
                    }
                    Err(error) => match error.downcast_ref::<ApiError>() {
                        Some(ApiError::ClientError(_)) => {}
                        _ => return Err(error),
                    },
                }
            }
            _ => {}
        }
    }
    if !found {
        writeln!(ui.term, "No templates found")?;
    }
    Ok(())
}
