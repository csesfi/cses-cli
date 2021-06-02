use crate::command::Submit;
use crate::service::submit::update_submit_parameters;
use super::fake_resources;
use anyhow::{Result};

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
    update_submit_parameters(&mut fake_resources, &submit)?;

    assert_eq!(fake_resources.storage.course, Some("4".to_string()));
    assert_eq!(fake_resources.storage.option, Some("PyPy".to_string()));
    Ok(())
}

// Remove
#[test]
fn it_adds_two() {
    assert_eq!(4, 4);
}
