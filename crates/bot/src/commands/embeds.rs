use poise::serenity_prelude as serenity;

use crate::lib::{util::{self, BRAZIL_TIMEZONE}, model::{Session, Teacher}};

/// Generates an embed displaying info for a session.
/// Ensure the teacher passed was obtained together with the session,
/// using e.g. [get_session_with_teacher].
///
/// [get_session_with_teacher]: crate::lib::db::repository::SessionRepository::get_session_with_teacher
pub fn session_embed<'embed>(
    embed: &'embed mut serenity::CreateEmbed, session: &Session, teacher: &Teacher, locale: Option<&str>, show_meet_link: bool
) -> &'embed mut serenity::CreateEmbed {
    let Session {
        id,
        summary,
        meet_id,
        start_at,
        end_at,
        ..
    } = session;

    let Teacher {
        name: teacher_name,
        ..
    } = teacher;

    let summary = summary
        .as_ref()
        .map(|s| format!("\"{}\"", s))
        .unwrap_or("".to_string());
    let start_at = start_at.with_timezone(&*BRAZIL_TIMEZONE);
    let duration = end_at.signed_duration_since(start_at);

    if locale == Some("pt-BR") {
        let duration =
            util::locale::convert_chrono_duration_to_brazilian_string(duration);
        let starts_at_label = if start_at < util::time::brazil_now() { "Começou em" } else { "Começa em" };
        let embed = embed.title(format!("Sessão #{}", id))
            .field("Mentor", teacher_name.to_string(), true)
            .field(starts_at_label, start_at.to_string(), false)
            .field("Duração", duration, true)
            .description(summary)
            .color(serenity::Colour::BLITZ_BLUE);

        if show_meet_link {
            embed
                .field(
                    "Link do Meet",
                    meet_id
                        .as_ref()
                        .map(|meet_id| format!("https://meet.google.com/{meet_id}"))
                        .unwrap_or_else(|| "(nenhum)".to_owned()),
                    true)
        } else {
            embed
        }
    } else {
        let duration = util::locale::convert_chrono_duration_to_string(duration);
        let starts_at_label = if start_at < util::time::brazil_now() { "Started at" } else { "Starts at" };
        let embed = embed.title(format!("Session #{}", id))
            .field("Mentor", teacher_name.to_string(), true)
            .field(starts_at_label, start_at.to_string(), false)
            .field("Duration", duration, true)
            .description(summary)
            .color(serenity::Colour::BLITZ_BLUE);

        if show_meet_link {
            embed
                .field(
                    "Meet Link",
                    meet_id
                        .as_ref()
                        .map(|meet_id| format!("https://meet.google.com/{meet_id}"))
                        .unwrap_or_else(|| "(none)".to_owned()),
                    true)
        } else {
            embed
        }
    }

}
