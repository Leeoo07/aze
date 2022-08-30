use chrono::NaiveDateTime;
use chrono::ParseError;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;

use crate::service::project::has_project;
use crate::service::tags::has_tag;

pub fn parse_to_datetime(s: &str) -> Result<NaiveDateTime, ParseError> {
    return NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S");
}
pub fn convert_tags(v: &str) -> Result<String, String> {
    if !v.starts_with("+") {
        return Err("Fail".to_string());
    }
    return Ok(v.strip_prefix("+").unwrap().to_string());
}

pub fn process_tags(tags: Vec<String>, confirm: bool) -> bool {
    for tag in tags {
        if !process_tag(tag, confirm) {
            return false;
        }
    }
    return true;
}

pub fn process_tag(tag: String, confirm: bool) -> bool {
    if confirm && !has_tag(tag.to_string()) {
        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Tag '{}' does not exist yet. Create it?", tag))
            .default(false)
            .interact()
            .unwrap()
        {
            return false;
        }
    }
    return true;
}

pub fn process_project(project: String, confirm: bool) -> bool {
    if confirm && !has_project(project.to_string()) {
        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Project '{}' does not exist yet. Create it?",
                project
            ))
            .default(false)
            .interact()
            .unwrap()
        {
            return false;
        }
    }
    return true;
}
