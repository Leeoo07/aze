use anyhow::anyhow;
use anyhow::Result;
use chrono::Datelike;
use chrono::Duration;
use chrono::Local;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::associations::HasTable;
use diesel::sql_types::Bool;
use diesel::sql_types::SqlType;
use diesel::BoxableExpression;
use mycroft::cli::parse_to_datetime;
use mycroft::database::establish_connection;
use mycroft::display::Display;
use mycroft::models::Frame;
use mycroft::schema;

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use super::MyCommand;

#[derive(clap::Args, Debug)]
pub struct LogSubcommand {
    #[clap(
        short = 'p',
        long = "project",
        help = "Logs activity only for the given project. You can add other projects by using this option several times.",
        multiple = true,
        display_order = 10
    )]
    pub projects: Vec<String>,

    #[clap(
        short = 'T',
        long = "tag",
        help = "Logs activity only for frames containing the given tag. You can add several tags by using this option multiple times.",
        multiple = true,
        display_order = 11
    )]
    pub tags: Vec<String>,

    #[clap(
        long = "ignore-project",
        help = "Logs activity for all projects but the given ones. You can ignore several projects by using this option several times.",
        multiple = true,
        display_order = 12
    )]
    pub ignored_projects: Vec<String>,

    #[clap(
        long = "ignore-tag",
        help = "Logs activity for all tags but the given ones. You can ignore several tags by using this option several times.",
        multiple = true,
        display_order = 13
    )]
    pub ignored_tags: Vec<String>,

    #[clap(
        short = 'c',
        long = "current",
        display_order = 1,
        help = "Include currently running frame in output."
    )]
    pub current: bool,

    #[clap(
        short = 'r',
        long = "reverse",
        display_order = 2,
        help = "Reverse the order of the days in output."
    )]
    pub reverse: bool,

    #[clap(help = "The date from when the log should start. Defaults to seven days ago.", display_order = 3, short = 'f', long = "from", value_parser = parse_to_datetime)]
    pub from: Option<NaiveDateTime>,
    #[clap(help = "The date at which the log should stop (inclusive). Defaults to tomorrow", display_order = 4, short = 't', long = "to", value_parser = parse_to_datetime)]
    pub to: Option<NaiveDateTime>,

    #[clap(
        short = 'y',
        long = "year",
        display_order = 5,
        group = "short_filter",
        help = "Reports activity for the current year."
    )]
    pub year: bool,

    #[clap(
        short = 'm',
        long = "month",
        display_order = 6,
        group = "short_filter",
        help = "Reports activity for the current month."
    )]
    pub month: bool,

    #[clap(
        short = 'w',
        long = "week",
        display_order = 7,
        group = "short_filter",
        help = "Reports activity for the current week."
    )]
    pub week: bool,

    #[clap(
        short = 'd',
        long = "day",
        display_order = 8,
        group = "short_filter",
        help = "Reports activity for the current day."
    )]
    pub day: bool,

    #[clap(
        short = 'a',
        long = "all",
        display_order = 9,
        group = "short_filter",
        help = "Reports all activities."
    )]
    pub all: bool,

    #[clap(
        short = 'j',
        long = "json",
        display_order = 9,
        group = "view",
        help = "Format output in JSON instead of plain text."
    )]
    pub json: bool,

    #[clap(
        short = 's',
        long = "csv",
        display_order = 9,
        group = "view",
        help = "Format output in CSV instead of plain text."
    )]
    pub csv: bool,

    #[clap(
        short = 'g',
        long = "pager",
        display_order = 9,
        group = "view",
        help = "View output through a pager."
    )]
    pub pager: bool,
}

impl LogSubcommand {
    fn parse_project(&self) -> Vec<&String> {
        let mut difference = vec![];
        for i in &self.projects {
            if self.ignored_projects.contains(&i) {
                difference.push(i);
            }
        }
        difference
    }

    fn parse_tags(&self) -> Vec<&String> {
        let mut difference = vec![];
        for i in &self.tags {
            if self.ignored_tags.contains(&i) {
                difference.push(i);
            }
        }
        difference
    }
}

impl MyCommand for LogSubcommand {
    fn run(&self, output: super::Output) -> Result<()> {
        use mycroft::schema::frames::dsl::*;

        let mut conn = establish_connection();

        let collisions = self.parse_project();
        if !collisions.is_empty() {
            return Err(anyhow!("given projects can't be ignored at the same time"));
        }

        let collisions = self.parse_tags();
        if !collisions.is_empty() {
            return Err(anyhow!("given tags can't be ignored at the same time"));
        }

        let last_week = (Local::now() - Duration::weeks(1)).naive_utc();

        let mut query = frames::table().into_boxed();

        query = query
            .filter(deleted.eq(false))
            .filter(start.gt(last_week))
            .filter(project.ne_all(self.ignored_projects.to_vec()))
            .filter(tags.ne_all(self.ignored_tags.to_vec()))
            .order_by(start.desc());

        if !self.projects.is_empty() {
            query = query.filter(project.eq_any(self.projects.to_vec()));
        }

        if !self.tags.is_empty() {
            query = query.filter(tags.eq_any(self.tags.to_vec()));
        }

        let results = query
            .load::<Frame>(&mut conn)
            .expect("Error loading frames");

        let mut actual_day: Option<NaiveDate> = None;
        let mut list: Vec<Display> = Vec::new();

        for frame in results {
            let cloned_start = frame.start.date();
            if actual_day.is_none() || actual_day.unwrap() != frame.start.date() {
                list.push(Display::new(cloned_start, vec![frame]));
                actual_day = Some(cloned_start);
            } else {
                list.last_mut().unwrap().add_frame(frame);
            }
        }

        for mut display in list {
            let duration = display.total_duration();

            writeln!(
                output.out,
                "{} {} {} {} ({}h {}m {}s)",
                display.date.weekday(),
                display.date.day(),
                display.date.month(),
                display.date.year(),
                duration.num_hours(),
                format!(
                    "{:02}",
                    duration.num_minutes() - (duration.num_hours() * 60)
                ),
                format!(
                    "{:02}",
                    duration.num_seconds() - (duration.num_minutes() * 60)
                )
            )?;
            for frame in display.frames {
                if frame.end.is_none() {
                    continue;
                }
                let frame_duration = frame.end.unwrap() - frame.start;
                writeln!(
                    output.out,
                    "\t{}\t{} to {}\t{}h {}m {}s\t{}",
                    &frame.id[..7],
                    frame.start.format("%H:%M"),
                    frame.end.unwrap().format("%H:%M"),
                    frame_duration.num_hours(),
                    format!(
                        "{:02}",
                        frame_duration.num_minutes() - (frame_duration.num_hours() * 60)
                    ),
                    format!(
                        "{:02}",
                        frame_duration.num_seconds() - (frame_duration.num_minutes() * 60)
                    ),
                    frame.project
                )?;
            }
        }

        Ok(())
    }
}
