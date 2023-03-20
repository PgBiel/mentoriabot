use std::ops::Add;
use poise::Modal;
use poise::serenity_prelude as serenity;
use crate::{common::{ApplicationContext, Context}, error::Result, util};
use crate::commands::modals::lectures::LectureCreateModals;
use crate::error::Error;
use crate::model::{Lecture, NewLecture, NewUser, User};
use crate::repository::Repository;
use crate::util::{brazil_timezone, HumanParseableDateTime};

/// Manages lectures.
#[poise::command(
    slash_command,
    ephemeral,
    name_localized("pt-BR", "aulas"),
    description_localized("pt-BR", "Gerencia aulas."),
    subcommands("create", "get", "remove", "all")
)]
pub async fn lectures(ctx: Context<'_>) -> Result<()> {
    ctx.send(|reply| reply.content("Please specify a subcommand").ephemeral(true))
        .await?;
    Ok(())
}

/// Creates a lecture at a certain date.
#[poise::command(
    slash_command,
    ephemeral,
    owners_only,
    name_localized("pt-BR", "criar"),
    description_localized("pt-BR", "Cria uma aula.")
)]
async fn create(
    ctx: ApplicationContext<'_>,

    // #[name_localized("pt-BR", "nome")]
    // #[description_localized("pt-BR", "O tópico desta aula")]
    // #[description = "This lecture's topic"]
    // name: String,
    //
    // #[name_localized("pt-BR", "descrição")]
    // #[description_localized("pt-BR", "Uma curta descrição para esta aula")]
    // #[description = "A brief summary of this lecture"]
    // description: String,

    #[name_localized("pt-BR", "vagas")]
    #[description_localized("pt-BR", "Número máximo de alunos na aula")]
    #[description = "Max amount of students which can take the class"]
    #[min = 1]
    #[max = 256]
    student_limit: i32,

    // #[name_localized("pt-BR", "começa_em")]
    // #[description_localized(
    //     "pt-BR",
    //     "Quando esta aula irá começar, no formato: [DD/MM/YYYY] HH:MM[:SS]"
    // )]
    // #[description = "When this lecture is planned to start"]
    // starts_at: HumanParseableDateTime,

    #[name_localized("pt-BR", "horas")]
    #[description_localized("pt-BR", "A duração desta aula, em horas")]
    #[description = "The duration of this lecture, in hours"]
    #[min = 0]
    #[max = 12]
    hours: i64,
) -> Result<()> {
    let modal = LectureCreateModals::execute_based_on_locale(ctx).await?;

    let Some(modal) = modal else {
        ctx.send(|b| b
            .content(
                "Failed to receive your response to the modal form; \
                please try running this command again."
            )
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
        ctx.send(
            |b| b
                .content("Please provide a future timestamp for when the lecture will start."))
            .await?;
        return Ok(());
    }

    let name = modal.name().clone();
    let description = modal.description().clone();
    let end_at = start_at.add(chrono::Duration::hours(hours));

    let author = ctx.author();
    let author_id = author.id.into();

    ctx.defer_ephemeral().await?;

    ctx.data().db.user_repository().insert_if_not_exists(&NewUser {
        discord_id: author_id,
        name: author.name.clone(),
        bio: None
    }).await?;

    let res = ctx.data().db.lecture_repository().insert(&NewLecture {
        name,
        description,
        teacher_id: author_id,
        student_limit,
        notified: false,
        start_at,
        end_at,
    }).await?;

    let Lecture {
        id: inserted_id,
        name: inserted_name,
        ..
    } = res;

    ctx.send(|b| b.content(
        if ctx.locale() == Some("pt-BR") {
            format!(
                "Aula '{}' criada com sucesso. \
                (Veja mais informações usando '/aulas obter {}'.) 👍",
                inserted_name,
                inserted_id,
            )
        } else {
            format!(
                "Lecture '{}' created successfully. \
                (View more info with '/lectures get {}'.) 👍",
                inserted_name,
                inserted_id,
            )
        }
    )).await?;
    Ok(())
}

/// Gets a lecture from the database
#[poise::command(
    slash_command,
    ephemeral,
    owners_only,
    name_localized("pt-BR", "obter"),
    description_localized("pt-BR", "Obtém informações sobre uma aula do banco de dados.")
)]
pub async fn get(
    ctx: ApplicationContext<'_>,

    #[description = "ID of the Lecture to get"]
    #[description_localized("pt-BR", "Identificador da aula a obter.")]
    id: i64,
) -> Result<()> {
    let lecture_and_teacher = ctx.data.db.lecture_repository().get_with_teacher(id).await?;
    if let Some((lecture, teacher)) = lecture_and_teacher {
        let Lecture {
            name,
            description,
            teacher_id,
            student_limit,
            start_at,
            end_at,
            ..
        } = lecture;

        let User { name: teacher_name, .. } = teacher;
        let teacher_mention = teacher_id.as_user_mention();

        let timezone = brazil_timezone()
            .ok_or_else(|| Error::Other("Failed to fetch the Brazilian timezone"))?;

        let start_at = start_at.with_timezone(&timezone);
        let duration = end_at.signed_duration_since(start_at);

        ctx.send(|f| {
            f.ephemeral(true).embed(|f| {
                if ctx.locale() == Some("pt-BR") {
                    let duration = util::convert_chrono_duration_to_brazilian_string(duration);
                    f.title(format!("Aula '{}'", name))
                        .field("Professor", format!("{teacher_name} ({teacher_mention})"), true)
                        .field("Começa em", format!("{start_at}"), false)
                        .field("Vagas", format!("{student_limit}"), true)
                        .field("Duração", format!("{duration}"), true)
                        .description(format!("\"{}\"", description))
                        .color(serenity::Colour::BLITZ_BLUE)
                } else {
                    let duration = util::convert_chrono_duration_to_string(duration);
                    f.title(format!("Lecture '{}'", name))
                        .field("Teacher", format!("{teacher_name} ({teacher_mention})"), true)
                        .field("Starts at", format!("{start_at}"), false)
                        .field("Max Students", format!("{student_limit}"), true)
                        .field("Duration", format!("{duration}"), true)
                        .description(format!("\"{}\"", description))
                        .color(serenity::Colour::BLITZ_BLUE)
                }
            })
        })
            .await?;
    } else {
        ctx.send(|b| b.content("Lecture not found.").ephemeral(true))
            .await?;
    }

    Ok(())
}

/// Removes a lecture from the database
#[poise::command(
    slash_command,
    ephemeral,
    owners_only,
    name_localized("pt-BR", "remover"),
    description_localized("pt-BR", "Remove uma aula permanentemente do banco de dados.")
)]
pub async fn remove(
    ctx: ApplicationContext<'_>,

    #[description = "ID of the lecture to remove"]
    #[description_localized("pt-BR", "Identificador da aula a ser removida.")]
    id: i64,
) -> Result<()> {
    let lecture = ctx.data().db.lecture_repository().get(id).await?;
    if let Some(lecture) = lecture {
        ctx.data().db.lecture_repository().remove(&lecture).await?;

        let removed_msg = format!(
            "Successfully removed lecture '{}' from the database.",
            lecture.name
        );
        ctx.send(|b| b.content(removed_msg).ephemeral(true)).await?;
    } else {
        ctx.send(|b| {
            b.content("Unknown lecture (maybe it was already deleted).")
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
    description_localized("pt-BR", "Lista todas as aulas no banco de dados.")
)]
pub async fn all(ctx: ApplicationContext<'_>) -> Result<()> {
    let lectures = ctx.data().db.lecture_repository().find_all().await?;
    let text = if lectures.is_empty() {
        "No lectures registered.".to_string()
    } else {
        let mut text = String::from("Lectures:");
        for lecture in lectures {
            text.push_str(&format!("\n- {:?}", lecture));
        }
        text
    };
    ctx.send(|b| b.content(text).ephemeral(true)).await?;

    Ok(())
}
