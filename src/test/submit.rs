use crate::command::Submit;
use crate::service;
use super::FakeStorage;
use super::fake_resources;
use super::fake_resources_with_mock_api;
use anyhow::Result;

#[test]
fn submit_parameters_are_updated() -> Result<()> {
    let mut fake_resources = fake_resources();
    let submit = Submit {
        course_id: Some("4".to_string()),
        task_id: Some(17),
        language_name: Some("Python2".to_string()),
        language_option: None,
        file_name: "submission.py".to_string(),
    };
    fake_resources.storage.option = Some("PyPy".to_string());
    service::update_submit_parameters(&mut fake_resources, &submit)?;

    assert_eq!(fake_resources.storage.course, Some("4".to_string()));
    assert_eq!(fake_resources.storage.option, Some("PyPy".to_string()));
    Ok(())
}

#[test]
fn submit_mock() -> Result<()> {
    let mut fake_resources = fake_resources_with_mock_api();
    fake_resources.api.expect_submit_task()
        .withf(|token, course_id, task_id, submission| {
            token == "gnewwoiJ" &&
            course_id == "17" &&
            *task_id == 3 &&
            submission.language.name == "Python" &&
            submission.filename == "test" &&
            submission.content == "testing"
        })
        .returning(|_, _, _, _| Ok(17));
    fake_resources.storage = FakeStorage {
        token: Some("gnewwoiJ".to_string()),
        language: Some("Python".to_string()),
        course: Some("17".to_string()),
        task: Some(3),
        ..Default::default()
    };

    let submission_id = service::submit(&mut fake_resources, "test".to_string())?;
    assert_eq!(submission_id, 17);
    Ok(())
}
