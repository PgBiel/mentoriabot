use std::ops::Add;

use poise::serenity_prelude as serenity;

use crate::{
    commands::modals::sessions::SessionCreateModals,
    common::{ApplicationContext, Context},
    error::Result,
    model::{NewSession, NewUser, Session, User},
    repository::Repository,
    util,
    util::{tr, BRAZIL_TIMEZONE},
};

/// Manages sessions with mentors.
#[poise::command(
    slash_command,
    ephemeral,
    name_localized("pt-BR", "sessões"),
    description_localized("pt-BR", "Gerencia sessões de mentoria."),
    subcommands("create", "get", "remove", "all")
)]
pub async fn sessions(ctx: Context<'_>) -> Result<()> {
    ctx.send(|reply| {
        reply
            .content(tr!("commands.general.specify_subcommand", ctx = ctx))
            .ephemeral(true)
    })
    .await?;
    Ok(())
}

/// Creates a mentorship session, at a certain date.
#[poise::command(
    slash_command,
    ephemeral,
    owners_only,
    name_localized("pt-BR", "criar"),
    description_localized("pt-BR", "Cria uma sessão de mentoria.")
)]
async fn create(
    ctx: ApplicationContext<'_>,

    #[name_localized("pt-BR", "horas")]
    #[description_localized("pt-BR", "A duração estimada desta sessão, em horas")]
    #[description = "The estimated duration of this session, in hours"]
    #[min = 0]
    #[max = 12]
    hours: i64,
) -> Result<()> {
    let modal = SessionCreateModals::execute_based_on_locale(ctx).await?;

    let Some(modal) = modal else {
        ctx.send(|b| b
            .content(tr!("commands.general.no_modal_response", ctx = ctx))
            .ephemeral(true)
        ).await?;
        return Ok(());
    };

    let Some(start_at) = modal.parsed_starts_at() else {
        ctx.send(|b| {
            b
                .content(format!(
                    "Sorry, I could not parse the date '{}'. Please use the format \
                        `DD/MM/YYYY HH:MM`.",
                    modal.starts_at()
                ))
                .ephemeral(true)
        }).await?;
        return Ok(());
    };

    if start_at <= chrono::Utc::now().add(chrono::Duration::seconds(1)) {
        ctx.send(|b| {
            b.content("Please provide a future timestamp for when the session will start.")
        })
        .await?;
        return Ok(());
    }

    let name = modal.name().clone();
    let description = modal.description().clone();
    let end_at = start_at.add(chrono::Duration::hours(hours));

    let author = ctx.author();
    let author_id = author.id.into();

    ctx.defer_ephemeral().await?;

    ctx.data()
        .db
        .user_repository()
        .insert_if_not_exists(&NewUser {
            discord_id: author_id,
            name: author.name.clone(),
            bio: None,
        })
        .await?;

    let res = ctx
        .data()
        .db
        .session_repository()
        .insert(&NewSession {
            name,
            description,
            availability_id: None, // TODO
            teacher_id: author_id,
            student_id: author_id,
            notified: false,
            start_at,
            end_at,
        })
        .await?;

    let Session {
        id: inserted_id,
        name: inserted_name,
        ..
    } = res;

    ctx.send(|b| {
        b.content(if ctx.locale() == Some("pt-BR") {
            format!(
                "Aula '{}' criada com sucesso. \
                (Veja mais informações usando '/aulas obter {}'.) 👍",
                inserted_name, inserted_id,
            )
        } else {
            format!(
                "Session '{}' created successfully. \
                (View more info with '/sessions get {}'.) 👍",
                inserted_name, inserted_id,
            )
        })
    })
    .await?;
    Ok(())
}

/// Gets a session from the database
#[poise::command(
    slash_command,
    ephemeral,
    owners_only,
    name_localized("pt-BR", "obter"),
    description_localized("pt-BR", "Obtém informações sobre uma aula do banco de dados.")
)]
pub async fn get(
    ctx: ApplicationContext<'_>,

    #[description = "ID of the Session to get"]
    #[description_localized("pt-BR", "Identificador da aula a obter.")]
    id: i64,
) -> Result<()> {
    let db = &ctx.data.db;
    let session_and_teacher = db.session_repository().get_with_teacher(id).await?;
    if let Some((session, teacher)) = session_and_teacher {
        let Session {
            name,
            description,
            teacher_id,
            start_at,
            end_at,
            ..
        } = session;

        let User {
            name: teacher_name, ..
        } = db.user_repository().find_by_teacher(&teacher).await?;
        let teacher_mention = teacher_id.as_user_mention();

        let start_at = start_at.with_timezone(&*BRAZIL_TIMEZONE);
        let duration = end_at.signed_duration_since(start_at);

        ctx.send(|f| {
            f.ephemeral(true).embed(|f| {
                if ctx.locale() == Some("pt-BR") {
                    let duration =
                        util::locale::convert_chrono_duration_to_brazilian_string(duration);
                    f.title(format!("Sessão '{}'", name))
                        .field(
                            "Mentor",
                            format!("{teacher_name} ({teacher_mention})"),
                            true,
                        )
                        .field("Começa em", start_at.to_string(), false)
                        .field("Duração", duration, true)
                        .description(format!("\"{}\"", description))
                        .color(serenity::Colour::BLITZ_BLUE)
                } else {
                    let duration = util::locale::convert_chrono_duration_to_string(duration);
                    f.title(format!("Session '{}'", name))
                        .field(
                            "Mentor",
                            format!("{teacher_name} ({teacher_mention})"),
                            true,
                        )
                        .field("Starts at", start_at.to_string(), false)
                        .field("Duration", duration, true)
                        .description(format!("\"{}\"", description))
                        .color(serenity::Colour::BLITZ_BLUE)
                }
            })
        })
        .await?;
    } else {
        ctx.send(|b| b.content("Session not found.").ephemeral(true))
            .await?;
    }

    Ok(())
}

/// Removes a session from the database
#[poise::command(
    slash_command,
    ephemeral,
    owners_only,
    name_localized("pt-BR", "remover"),
    description_localized("pt-BR", "Remove uma sessão permanentemente do banco de dados.")
)]
pub async fn remove(
    ctx: ApplicationContext<'_>,

    #[description = "ID of the session to remove"]
    #[description_localized("pt-BR", "Identificador da sessão a ser removida.")]
    id: i64,
) -> Result<()> {
    let session = ctx.data().db.session_repository().get(id).await?;
    if let Some(session) = session {
        ctx.data().db.session_repository().remove(&session).await?;

        let removed_msg = format!(
            "Successfully removed session '{}' from the database.",
            session.name
        );
        ctx.send(|b| b.content(removed_msg).ephemeral(true)).await?;
    } else {
        ctx.send(|b| {
            b.content("Unknown session (maybe it was already deleted).")
                .ephemeral(true)
        })
        .await?;
    }

    Ok(())
}

#[poise::command(
    slash_command,
    ephemeral,
    owners_only,
    name_localized("pt-BR", "listar"),
    description_localized("pt-BR", "Lista todas as sessões no banco de dados.")
)]
pub async fn all(ctx: ApplicationContext<'_>) -> Result<()> {
    let sessions = ctx.data().db.session_repository().find_all().await?;
    let text = if sessions.is_empty() {
        "No sessions registered.".to_string()
    } else {
        let mut text = String::from("Sessions:");
        for session in sessions {
            text.push_str(&format!("\n- {:?}", session));
        }
        text
    };
    ctx.send(|b| b.content(text).ephemeral(true)).await?;

    Ok(())
}
