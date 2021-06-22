use super::fake_resources;
use super::fake_resources_with_mock_api;
use crate::command::Submit;
use crate::entities::{Language, Scope, SubmissionInfo, SubmitParameters, TaskId};
use crate::service;
use crate::storage::StorageData;
use anyhow::Result;

#[test]
fn submit_parameters_passed_through() -> Result<()> {
    let mut fake_resources = fake_resources();
    let submit = Submit {
        task: Some(TaskId::Number(17)),
        language: Language {
            name: Some("Python2".to_string()),
            option: None,
        },
        file_name: "submission.py".to_string(),
    };
    let submit_params = service::create_submit_parameters(
        &mut fake_resources,
        &Scope::Course("crs".to_string()),
        submit,
    )?;
    assert_eq!(submit_params.file, "submission.py");

    Ok(())
}

#[test]
fn submit_mock() -> Result<()> {
    let mut fake_resources = fake_resources_with_mock_api();
    fake_resources
        .api
        .expect_submit_task()
        .withf(|token, course_id, task_id, submission| {
            token == "gnewwoiJ"
                && course_id == "crs"
                && *task_id == Some(3)
                && submission.language.name == Some("Python".to_string())
                && submission.filename == "extracted_filename"
                && submission.content == "testing"
        })
        .returning(|_, _, _, _| {
            Ok(SubmissionInfo {
                id: 17,
                ..Default::default()
            })
        });
    let mut storage_data: StorageData = Default::default();
    storage_data.set_token("gnewwoiJ".to_string());
    storage_data.set_scope(Scope::Course("crs".to_string()));
    fake_resources.storage.data = storage_data;
    let submit_params = SubmitParameters {
        course: "crs".to_owned(),
        file: "extracted_filename".to_owned(),
        task: Some(TaskId::Number(3)),
        language: Language {
            name: Some("Python".to_owned()),
            option: None,
        },
    };
    let submission_response = service::submit(&mut fake_resources, submit_params)?;
    assert_eq!(submission_response.id, 17);
    Ok(())
}
