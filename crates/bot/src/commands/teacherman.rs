use crate::{
    commands::{embeds, modals::teacher::TeacherModal},
    common::{ApplicationContext, Context},
    lib::{db::Repository, error::Result, model::Teacher, util::tr},
};

/// Manages mentors.
#[poise::command(
    slash_command,
    ephemeral,
    description_localized("pt-BR", "Gerencia mentores."),
    subcommands("create", "get", "all")
)]
pub async fn teacherman(ctx: Context<'_>) -> Result<()> {
    ctx.send(|reply| {
        reply
            .content(tr!("commands.general.specify_subcommand", ctx = ctx))
            .ephemeral(true)
    })
    .await?;
    Ok(())
}

/// Creates a mentor.
#[poise::command(
    slash_command,
    ephemeral,
    owners_only,
    name_localized("pt-BR", "criar"),
    description_localized("pt-BR", "Cria um mentor.")
)]
async fn create(ctx: ApplicationContext<'_>) -> Result<()> {
    let Some(modal) = TeacherModal::ask(ctx).await? else {
        return Ok(());
    };

    let teacher = modal.generate_new_teacher();

    if ctx
        .data()
        .db
        .teacher_repository()
        .find_by_email(&teacher.email)
        .await?
        .is_some()
    {
        ctx.say(tr!(
            "commands.teacherman.email_already_exists",
            ctx = ctx,
            email = teacher.email
        ))
        .await?;
        return Ok(());
    }

    let teacher = ctx.data().db.teacher_repository().insert(&teacher).await?;

    let Teacher { name, email, .. } = teacher;

    ctx.say(tr!(
        "commands.teacherman.success",
        ctx = ctx,
        name = name,
        email = email
    ))
    .await?;
    Ok(())
}

/// Gets info about a mentor from the database
#[poise::command(
    slash_command,
    ephemeral,
    owners_only,
    name_localized("pt-BR", "obter"),
    description_localized("pt-BR", "Obtém informações sobre um mentor do banco de dados.")
)]
pub async fn get(
    ctx: ApplicationContext<'_>,

    #[description = "Email of the mentor to get"]
    #[description_localized("pt-BR", "E-mail do mentor a obter.")]
    email: String,
) -> Result<()> {
    let db = &ctx.data.db;
    let teacher = db.teacher_repository().find_by_email(&email).await?;

    if let Some(teacher) = teacher {
        ctx.send(|f| {
            f.ephemeral(true)
                .embed(|f| embeds::teacher_embed(f, &teacher, ctx.locale()))
        })
        .await?;
    } else {
        ctx.send(|b| b.content("Mentor not found.").ephemeral(true))
            .await?;
    }

    Ok(())
}

// /// Removes a mentor from the database
// #[poise::command(
//     slash_command,
//     ephemeral,
//     owners_only,
//     name_localized("pt-BR", "remover"),
//     description_localized("pt-BR", "Remove um mentor permanentemente do banco de dados.")
// )]
// pub async fn remove(
//     ctx: ApplicationContext<'_>,

//     #[description = "ID of the session to remove"]
//     #[description_localized("pt-BR", "Identificador da sessão a ser removida.")]
//     id: i64,
// ) -> Result<()> {
//     let session = ctx.data().db.session_repository().get(id).await?;
//     if let Some(session) = session {
//         ctx.data().db.session_repository().remove(&session).await?;

//         let removed_msg = format!(
//             "Successfully removed session '{}' from the database.",
//             session.summary.unwrap_or_else(|| "Unnamed".to_string())
//         );
//         ctx.send(|b| b.content(removed_msg).ephemeral(true)).await?;
//     } else {
//         ctx.send(|b| {
//             b.content("Unknown session (maybe it was already deleted).")
//                 .ephemeral(true)
//         })
//         .await?;
//     }

//     Ok(())
// }

/// List all mentors in the database.
#[poise::command(
    slash_command,
    ephemeral,
    owners_only,
    name_localized("pt-BR", "listar"),
    description_localized("pt-BR", "Lista todos os mentores no banco de dados.")
)]
pub async fn all(ctx: ApplicationContext<'_>) -> Result<()> {
    let teachers = ctx.data().db.teacher_repository().find_all().await?;
    let text = if teachers.is_empty() {
        "No mentors registered.".to_string()
    } else {
        let mut text = String::from("Mentors:");
        for teacher in teachers {
            text.push_str(&format!("\n- #{}: {}", teacher.id, teacher.email));
        }
        text
    };
    ctx.send(|b| b.content(text).ephemeral(true)).await?;

    Ok(())
}
