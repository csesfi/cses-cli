use super::fake_resources_with_mock_api;
use crate::command;
use crate::entities::{Language, Scope, SubmissionInfo};
use crate::service;
use crate::storage::StorageData;
use anyhow::Result;
use std::path::PathBuf;

#[test]
fn submit_mock() -> Result<()> {
    let mut fake_resources = fake_resources_with_mock_api();
    fake_resources
        .api
        .expect_submit_task()
        .withf(|token, scope, task_id, submission| {
            token == "gnewwoiJ"
                && scope.to_string() == "crs"
                && *task_id == Some("3")
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
    let scope = Scope::Course("crs".to_string());
    fake_resources.storage.data = storage_data;
    let submit_params = command::Submit {
        task: Some("3".to_owned()),
        language: Language {
            name: Some("Python".to_owned()),
            option: None,
        },
        filename: PathBuf::from("input filename"),
    };
    let submission_response = service::submit(&mut fake_resources, &scope, submit_params)?;
    assert_eq!(submission_response.id, 17);
    Ok(())
}
