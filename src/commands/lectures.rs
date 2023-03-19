use std::ops::Add;
use crate::{common::{ApplicationContext, Context}, error::Result};
use crate::model::{Lecture, NewLecture};
use crate::repository::BasicRepository;
use crate::util::HumanParseableDateTime;

/// Manages lectures.
#[poise::command(
    slash_command,
    ephemeral,
    name_localized("pt-BR", "aulas"),
    description_localized("pt-BR", "Gerencia aulas."),
    subcommands("create")
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

    #[name_localized("pt-BR", "nome")]
    #[description_localized("pt-BR", "O tópico desta aula")]
    #[description = "This lecture's topic"]
    name: String,

    #[name_localized("pt-BR", "descrição")]
    #[description_localized("pt-BR", "Uma curta descrição para esta aula")]
    #[description = "A brief summary of this lecture"]
    description: String,

    #[name_localized("pt-BR", "começa_em")]
    #[description_localized(
        "pt-BR",
        "Quando esta aula irá começar, no formato: [DD/MM/YYYY] HH:MM[:SS]"
    )]
    #[description = "When this lecture is planned to start"]
    starts_at: HumanParseableDateTime,

    #[name_localized("pt-BR", "horas")]
    #[description_localized("pt-BR", "A duração desta aula, em horas")]
    #[description = "The duration of this lecture, in hours"]
    #[min = 0]
    #[max = 12]
    hours: i64,
) -> Result<()> {
    if &*starts_at <= &chrono::Utc::now().add(chrono::Duration::seconds(1)) {
        ctx.send(
            |b| b.content("Please provide a future timestamp for 'starts_at'."))
            .await?;
        return Ok(());
    }
    ctx.defer_ephemeral().await?;
    let res = ctx.data().db.lecture_repository().insert(NewLecture {
        name: name.clone(),
        description: description.clone(),
        teacher_id: ctx.author().id.into(),
        start_at: starts_at.clone().into(),
        end_at: starts_at.add(chrono::Duration::hours(hours))
    }).await?;

    let Lecture {
        id: inserted_id,
        name: inserted_name,
        description: inserted_description,
        teacher_id: inserted_teacher_id,
        start_at: inserted_start_at,
        end_at: inserted_end_at,
        ..
    } = res;

    ctx.send(|b| b.content(
        format!(
            "Got: name={name} | desc={description} | starts_at={starts_at} | {hours} hours\n\
            Inserted: id={inserted_id} | name={inserted_name} | description={inserted_description}\
            | teacher_id={inserted_teacher_id} | start_at={inserted_start_at}\
            | end_at={inserted_end_at}"
        )
    )).await?;
    Ok(())
}
