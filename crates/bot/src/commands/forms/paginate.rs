//! Taken from [poise::builtins::paginate]
//! The only modifications include adding titles, footers and color to the embed.
use poise::serenity_prelude as serenity;

/// # Example
///
/// ```rust,no_run
/// # async fn _test(ctx: poise::Context<'_, (), serenity::Error>) -> Result<(), serenity::Error> {
/// let pages = &[
///     "Content of first page",
///     "Content of second page",
///     "Content of third page",
///     "Content of fourth page",
/// ];
///
/// poise::samples::paginate(ctx, pages).await?;
/// # Ok(()) }
/// ```
///
/// ![Screenshot of output](https://i.imgur.com/JGFDveA.png)
pub async fn paginate<U, E>(
    ctx: poise::Context<'_, U, E>,
    titles: Option<&[&str]>,
    footers: Option<&[&str]>,
    pages: &[&str],
) -> Result<(), serenity::Error> {
    // Define some unique identifiers for the navigation buttons
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx.id());
    let next_button_id = format!("{}next", ctx.id());

    let mut current_page = 0;

    fn create_buttons<'components>(
        b: &'components mut serenity::CreateComponents,
        prev_button_id: &str,
        next_button_id: &str,
        current_page: usize,
        pages: &[&str],
    ) -> &'components mut serenity::CreateComponents {
        b.create_action_row(|b| {
            b.create_button(|b| {
                b.custom_id(prev_button_id)
                    .emoji('◀')
                    .disabled(current_page == 0)
            })
            .create_button(|b| {
                b.custom_id(next_button_id)
                    .emoji('▶')
                    .disabled(current_page == pages.len() - 1)
            })
        })
    }

    fn create_embed<'embed>(
        b: &'embed mut serenity::CreateEmbed,
        current_page: usize,
        pages: &[&str],
        titles: Option<&[&str]>,
        footers: Option<&[&str]>,
    ) -> &'embed mut serenity::CreateEmbed {
        let b = b
            .description(pages[current_page])
            .color(serenity::Color::BLITZ_BLUE);

        let b = if let Some(titles) = titles {
            b.title(titles[current_page])
        } else {
            b
        };

        let b = if let Some(footers) = footers {
            b.footer(|f| f.text(footers[current_page]))
        } else {
            b
        };

        b
    }

    // Send the embed with the first page as content
    ctx.send(|b| {
        b.embed(|b| create_embed(b, current_page, pages, titles, footers))
            .components(|b| {
                create_buttons(b, &prev_button_id, &next_button_id, current_page, pages)
            })
    })
    .await?;

    // Loop through incoming interactions with the navigation buttons
    while let Some(press) = serenity::CollectComponentInteraction::new(ctx)
        // We defined our button IDs to start with `ctx_id`. If they don't, some other command's
        // button was pressed
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        // Timeout when no navigation button has been pressed for 24 hours
        .timeout(std::time::Duration::from_secs(3600 * 24))
        .await
    {
        // Depending on which button was pressed, go to next or previous page
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else {
            // This is an unrelated button interaction
            continue;
        }

        // Update the message with the new page contents
        press
            .create_interaction_response(ctx, |b| {
                b.kind(serenity::InteractionResponseType::UpdateMessage)
                    .interaction_response_data(|b| {
                        b.embed(|b| create_embed(b, current_page, pages, titles, footers))
                            .components(|b| {
                                create_buttons(
                                    b,
                                    &prev_button_id,
                                    &next_button_id,
                                    current_page,
                                    pages,
                                )
                            })
                    })
            })
            .await?;
    }

    Ok(())
}
