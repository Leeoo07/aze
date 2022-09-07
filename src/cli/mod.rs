use chrono::NaiveDateTime;
use chrono::ParseError;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;

use crate::config::load_config;
use crate::service::project::has_project;
use crate::service::tags::has_tag;

pub fn parse_to_datetime(s: &str) -> Result<NaiveDateTime, ParseError> {
    let result = NaiveDateTime::parse_from_str(s, &load_config().datetime_format);

    if result.is_err() {
        return NaiveDateTime::parse_from_str(
            format!("{} 00:00", s).as_str(),
            &load_config().datetime_format,
        );
    }

    return result;
}
pub fn convert_tags(v: &str) -> Result<String, String> {
    if !v.starts_with('+') {
        return Err("Fail".to_string());
    }
    return Ok(v.strip_prefix('+').unwrap().to_string());
}

pub fn process_tags(tags: Vec<String>, confirm: bool) -> bool {
    for tag in tags {
        if !process_tag(tag, confirm) {
            return false;
        }
    }
    true
}

pub fn process_tag(tag: String, confirm: bool) -> bool {
    if confirm
        && !has_tag(tag.to_string())
        && !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Tag '{}' does not exist yet. Create it?", tag))
            .default(false)
            .interact()
            .unwrap()
    {
        return false;
    }
    true
}

pub fn process_project(project: String, confirm: bool) -> bool {
    if confirm
        && !has_project(project.to_string())
        && !Confirm::with_theme(&ColorfulTheme::default())
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
    true
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};

    use super::parse_to_datetime;

    #[test]
    fn parse_with_date() {
        let result = parse_to_datetime("2000-02-03").unwrap();

        assert_eq!(2000, result.year());
        assert_eq!(2, result.month());
        assert_eq!(3, result.day());
        assert_eq!(0, result.hour());
        assert_eq!(0, result.minute());
    }

    #[test]
    fn parse_with_datetime() {
        let result = parse_to_datetime("2000-02-03 04:05").unwrap();

        assert_eq!(2000, result.year());
        assert_eq!(2, result.month());
        assert_eq!(3, result.day());
        assert_eq!(4, result.hour());
        assert_eq!(5, result.minute());
    }
}
