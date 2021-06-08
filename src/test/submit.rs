use super::fake_resources;
use super::fake_resources_with_mock_api;
use crate::command::Submit;
use crate::entities::Language;
use crate::service;
use crate::storage::{Storage, StorageData};
use anyhow::Result;

#[test]
fn submit_parameters_are_updated() -> Result<()> {
    let mut fake_resources = fake_resources();
    let submit = Submit {
        course_id: Some("4".to_string()),
        task_id: Some(17),
        language: Language {
            name: Some("Python2".to_string()),
            option: None,
        },
        file_name: "submission.py".to_string(),
    };
    fake_resources
        .storage
        .get_mut()
        .set_option("PyPy".to_string());
    service::update_submit_parameters(&mut fake_resources, &submit)?;

    assert_eq!(fake_resources.storage.get().get_course(), Some("4"));
    assert_eq!(fake_resources.storage.get().get_option(), Some("PyPy"));
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
                && course_id == "17"
                && *task_id == 3
                && submission.language.name == Some("Python".to_owned())
                && submission.filename == "extracted_filename"
                && submission.content == "testing"
        })
        .returning(|_, _, _, _| Ok(17));
    let mut storage_data: StorageData = Default::default();
    storage_data.set_token("gnewwoiJ".to_string());
    storage_data.set_language("Python".to_string());
    storage_data.set_course("17".to_string());
    storage_data.set_task(3);
    fake_resources.storage.data = storage_data;
    let submission_id = service::submit(&mut fake_resources, "test".to_string())?;
    assert_eq!(submission_id, 17);
    Ok(())
}
