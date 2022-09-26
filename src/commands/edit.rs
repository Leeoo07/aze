use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use anyhow::anyhow;
use anyhow::Result;
use chrono::NaiveDateTime;
use colored::Colorize;
use mycroft::database::establish_connection;
use mycroft::database::MyJsonType;
use mycroft::display::frame::JsonFrame;
use mycroft::models::Frame;
use mycroft::schema::frames;
use mycroft::service::frame::find_frame;
use mycroft::service::frame::frame_collides;
use mycroft::service::frame::frame_start_collides;
use mycroft::service::frame::last_created_frame;
use mycroft::service::frame::last_started_frame;

use super::MyCommand;

#[derive(clap::Args, Debug)]
#[clap(about = "Edit a frame.")]
pub struct EditSubcommand {
    #[clap(
        help = "Frame ID which should be edited. If not specified, will edit last frame recorded."
    )]
    pub frame_id: Option<String>,

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

impl MyCommand for EditSubcommand {
    fn run(&self, output: super::Output) -> Result<()> {
        let frame: Frame;

        if self.frame_id.is_some() {
            let frame_id = self.frame_id.as_ref().unwrap().to_string();
            let frame_by_id = find_frame(&frame_id);
            if frame_by_id.is_err() {
                return Err(anyhow!("No frame found with id {}", frame_id));
            }
            frame = frame_by_id.unwrap();
        } else {
            let last_frame = last_created_frame();
            if last_frame.is_none() {
                return Err(anyhow!(
                    "No frames recorded yet. It's time to create your first one!"
                ));
            }
            frame = last_frame.unwrap();
        }

        let json_frame = JsonFrame::new(&frame);

        let content = serde_json::ser::to_string(&json_frame).expect("Could not serialize frame");
        let edited = edit::edit(content)?;

        let result_frame: serde_json::Result<JsonFrame> = serde_json::from_str(edited.as_str());

        if result_frame.is_err() {
            return Err(anyhow!(
                "Error while parsing inputted values: {}",
                result_frame.unwrap_err().to_string()
            ));
        }

        let new_frame = result_frame.unwrap();

        if new_frame.end.is_none() {
            let current_frame = last_started_frame();
            if current_frame.is_some() && frame.end.is_some() {
                return Err(anyhow!("Frame already started"));
            }

            if frame_start_collides(&new_frame.start) {
                return Err(anyhow!("Frame start collides"));
            }
        }


        let update_satement = diesel::update(&frame).set((
            frames::start.eq(new_frame.start),
            frames::end.eq(new_frame.end),
            frames::project.eq(new_frame.project),
            frames::tags.eq(MyJsonType(serde_json::json!(new_frame.tags))),
        ));

        let mut conn = establish_connection();
        let result = update_satement.execute(&mut conn);

        if result.is_err() {
            return Err(anyhow!("Could not save frame with id {}", frame.id));
        }
        Ok(())
    }
}
