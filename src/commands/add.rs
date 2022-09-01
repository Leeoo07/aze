use anyhow::anyhow;
use anyhow::Result;
use chrono::NaiveDateTime;
use colored::Colorize;
use mycroft::cli::convert_tags;
use mycroft::cli::parse_to_datetime;
use mycroft::cli::process_project;
use mycroft::cli::process_tags;
use mycroft::service::frame::create_frame;
use mycroft::service::frame::frame_collides;

use super::MyCommand;

#[derive(clap::Args, Debug)]
#[clap(
    about = "Add time to a project with tag(s) that was not tracked live.",
    after_help = "Example:\n\n$ mycroft add --from \"2018-03-20 12:00:00\" --to \"2018-03-20 13:00:00\" \\ \n programming +addfeature"
)]
pub struct AddSubcommand {
    #[clap(help = "Name of the project which should be used to add time.")]
    pub project: String,

    #[clap(help = "Tag(s) which should be added to the activity. Each tag has to be prepended with a plus sign.", value_parser = convert_tags)]
    pub tags: Vec<String>,

    #[clap(help = "Date and time of start of tracked activity", display_order = 1, short = 'f', long = "from", value_parser = parse_to_datetime, required = true)]
    pub from: NaiveDateTime,

    #[clap(help = "Date and time of end of tracked activity", display_order = 2, short = 't', long = "to", value_parser = parse_to_datetime, required = true)]
    pub to: NaiveDateTime,

    #[clap(
        help = "Confirm addition of new project",
        display_order = 3,
        short = 'c',
        long = "confirm-new-project"
    )]
    pub confirm_project: bool,

    #[clap(
        help = "Confirm addition of new tag",
        display_order = 4,
        short = 'b',
        long = "confirm-new-tags"
    )]
    pub confirm_tags: bool,
}

impl MyCommand for AddSubcommand {
    fn run(&self, output: super::Output) -> Result<()> {
        process_project(self.project.to_string(), self.confirm_project);
        process_tags(self.tags.to_owned(), self.confirm_tags);

        if frame_collides(&self.from, &self.to) {
            return Err(anyhow!(
                "Frame already exist which overlaps with start and end"
            ));
        }

        writeln!(
            output.out,
            "starting project {}{} from {} to {}",
            self.project.purple(),
            if self.tags.len() > 0 {
                format!(" [{}]", self.tags.join(",").cyan())
            } else {
                "".to_string()
            },
            self.from
                .format(&self.config().datetime_format)
                .to_string()
                .green(),
            self.to
                .format(&self.config().datetime_format)
                .to_string()
                .green()
        )?;

        create_frame(&self.from, &self.to, &self.project, self.tags.to_owned());

        return Ok(());
    }
}
