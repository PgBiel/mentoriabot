use poise::serenity_prelude::Attachment;

use crate::{common::ApplicationContext, lib::error::Error};

/// Load mentors from a CSV file
#[poise::command(slash_command, ephemeral, owners_only)]
pub async fn loadmentors(ctx: ApplicationContext<'_>, csv_file: Attachment) -> Result<(), Error> {
    let csv_contents = match csv_file.download().await {
        Ok(file_bytes) => match String::from_utf8(file_bytes) {
            Ok(contents) => contents,
            Err(_) => {
                ctx.say("It seems your attached file is not properly encoded in UTF-8, so I cannot read it!")
                    .await?;

                return Ok(());
            }
        },
        Err(err) => {
            tracing::warn!(
                "Could not download 'loadmentors' file ({}): {}",
                csv_file.filename,
                err
            );
            ctx.say("Could not download the attachment! Maybe it was too large?")
                .await?;

            return Ok(());
        }
    };

    match crate::loadmentors::load_teachers_to_db(&csv_contents, &ctx.data.db).await {
        Ok(res) => match res {
            Ok(lines) => {
                let teacher_count = lines.len();
                let avail_count: usize = lines
                    .into_iter()
                    .map(|(_, availabilities)| availabilities.len())
                    .sum();

                ctx.say(format!("Successfully added {teacher_count} teachers from CSV, with a total of {avail_count} availabilities."))
                    .await?;
            }
            Err(errs) => {
                let lines = errs
                    .iter()
                    .map(|(line, _)| line.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                let line_s = if lines.len() == 1 { "" } else { "s" };
                tracing::warn!(
                    "Inserting teachers read from CSV file failed; per-line errors: {errs:?}"
                );
                ctx.say(
                    format!(
                        "Failed to add the teachers read to the database; one or more errors occurred in line{line_s} {lines}!
Perhaps the teacher data is invalid?")
                )
                    .await?;

                return Ok(());
            }
        },
        Err(err) => {
            tracing::warn!("Reading teachers from CSV file failed: {err}");
            ctx.say("Could not read teachers from CSV file - maybe it wasn't valid CSV, or it had unknown columns? (There must be a valid header row!)")
                .await?;

            return Ok(());
        }
    };

    Ok(())
}
