use crate::{
    commands::embeds,
    common::Context,
    lib::{
        error::Result,
        util::{self, tr, BRAZIL_TIMEZONE},
    },
};

const SESSIONS_PER_PAGE: usize = 10;

/// Manages sessions with mentors.
#[poise::command(
    slash_command,
    ephemeral,
    name_localized("pt-BR", "mentorias"),
    description_localized("pt-BR", "Lista suas sessões de mentoria."),
    subcommands("list", "info")
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

/// Lists all your mentorship sessions.
#[poise::command(
    slash_command,
    ephemeral,
    name_localized("pt-BR", "lista"),
    description_localized("pt-BR", "Lista todas as suas mentorias.")
)]
pub async fn list(ctx: Context<'_>) -> Result<()> {
    let sessions = {
        let mut sessions = ctx
            .data()
            .db
            .session_repository()
            .find_by_student(ctx.author().id.into())
            .await?;
        sessions.reverse(); // show older sessions first
        sessions
    };

    if sessions.is_empty() {
        ctx.say(tr!("commands.sessions.no_sessions", ctx = ctx))
            .await?;
    } else {
        let pages = sessions
            .chunks(SESSIONS_PER_PAGE)  // 10 sessions per page
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|session| tr!(
                        "commands.session.session_list_entry",
                        ctx = ctx,
                        "id" => session.id,
                        "date" => util::time::day_month_year_display(&session.start_at.with_timezone(&*BRAZIL_TIMEZONE).date_naive()),
                    ))
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .collect::<Vec<_>>();

        let page_count = pages.len();
        let titles = (1..=page_count)
            .map(|page| tr!("commands.session.session_list_title", ctx = ctx, "page" => page + 1, "pages" => page_count))
            .collect::<Vec<_>>();
        let footers = std::iter::repeat(tr!("commands.session.session_list_footer", ctx = ctx))
            .take(page_count)
            .collect::<Vec<_>>();

        // convert to str references
        let pages = pages.iter().map(|s| &**s).collect::<Vec<_>>();
        let titles = titles.iter().map(|s| &**s).collect::<Vec<_>>();
        let footers = footers.iter().map(|s| &**s).collect::<Vec<_>>();

        crate::commands::forms::paginate(ctx, Some(&titles), Some(&footers), &pages).await?;
    }

    Ok(())
}

/// Shows information about a mentorship session.
#[poise::command(
    slash_command,
    ephemeral,
    name_localized("pt-BR", "info"),
    description_localized("pt-BR", "Mostra informações sobre uma mentoria.")
)]
pub async fn info(
    ctx: Context<'_>,

    #[description = "The session's number."]
    #[description_localized("pt-BR", "O número da mentoria.")]
    number: u32,
) -> Result<()> {
    if let Some((session, teacher)) = ctx
        .data()
        .db
        .session_repository()
        .get_with_teacher(number as i64)
        .await?
    {
        if session.student_id != ctx.author().id.into() {
            ctx.say(tr!("commands.sessions.info.not_your_session", ctx = ctx, "id" => number))
                .await?;
        } else {
            ctx.send(|b| {
                b.ephemeral(true)
                    .embed(|b| embeds::session_embed(b, &session, &teacher, ctx.locale(), true))
            })
            .await?;
        }
    } else {
        ctx.say(tr!("commands.sessions.info.no_such_session", ctx = ctx, "id" => number))
            .await?;
    }

    Ok(())
}
