use super::require_login;
use crate::command;
use crate::{
    api::CodeSubmit,
    entities::{Language, SubmissionInfo},
};
use crate::{CsesApi, Filesystem, Resources, Storage, RP};
use anyhow::{anyhow, Result};

pub fn update_submit_parameters(
    res: &mut Resources<impl RP>,
    parameters: &command::Submit,
) -> Result<()> {
    let storage = res.storage.get_mut();
    if let Some(ref course_id) = parameters.course_id {
        storage.set_course(course_id.clone());
    }
    if let Some(task_id) = parameters.task_id {
        storage.set_task(task_id);
    }
    if let Some(ref language_name) = parameters.language_name {
        storage.set_language(language_name.clone());
    }
    if let Some(ref language_option) = parameters.language_option {
        storage.set_option(language_option.clone());
    }
    Ok(())
}

pub fn submit(res: &mut Resources<impl RP>, filename: String) -> Result<u64> {
    require_login(res)?;
    let storage = res.storage.get();
    let course_id = storage
        .get_course()
        .ok_or_else(|| anyhow!("Course not provided"))?
        .to_owned();
    let task_id = storage
        .get_task()
        .ok_or_else(|| anyhow!("Task not provided"))?
        .to_owned();
    let language_name = storage
        .get_language()
        .ok_or_else(|| anyhow!("Language not provided"))?
        .to_owned();
    let language_option = storage.get_option().map(|t| t.to_owned());

    let content = res.filesystem.get_file(&filename)?;
    let content = res.filesystem.encode_base64(&content);
    let submission = CodeSubmit {
        language: Language {
            name: language_name,
            option: language_option,
        },
        filename,
        content,
    };
    let submission_id =
        res.api
            .submit_task(require_login(res)?, &course_id, task_id, &submission)?;
    Ok(submission_id)
}

pub fn submission_info(
    res: &mut Resources<impl RP>,
    submission_id: u64,
    poll: bool,
) -> Result<SubmissionInfo> {
    let storage = res.storage.get();
    let course_id = storage.get_course().unwrap();
    let task_id = storage.get_task().unwrap();
    let submission =
        res.api
            .get_submit(require_login(res)?, course_id, task_id, submission_id, poll);
    Ok(submission?)
}
