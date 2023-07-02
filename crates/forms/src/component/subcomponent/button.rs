use poise::serenity_prelude as serenity;

use crate::{
    component::Buildable,
    interaction::{CustomId, HasCustomId},
};

/// Holds all data necessary to display a Discord button.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ButtonSpec {
    /// The button's literal label.
    pub label: String,

    /// The button's fixed custom ID,
    /// which defaults to an auto-generated one.
    pub custom_id: CustomId,

    pub style: Option<serenity::ButtonStyle>,

    /// Makes the button lead to the given link
    /// NOTE: Such a button cannot be awaited for.
    pub link: Option<String>,

    /// An optional single emoji to display near the label
    pub emoji: Option<serenity::ReactionType>,

    /// If this button is disabled and cannot be clicked
    pub disabled: bool,
}

impl ButtonSpec {
    /// Returns the link this button points to,
    /// if it is not empty. Otherwise `None`.
    fn get_link_if_valid(&self) -> Option<&String> {
        match self.link.as_ref() {
            Some(link) if !link.is_empty() => Some(link),
            _ => None,
        }
    }
}

impl Buildable<serenity::CreateButton> for ButtonSpec {
    fn on_build<'a>(
        &self,
        mut builder: &'a mut serenity::CreateButton,
    ) -> &'a mut serenity::CreateButton {
        builder = builder.label(self.label.clone());

        if let Some(link) = self.get_link_if_valid() {
            builder = builder.url(link);
        } else {
            // if not a link button => can have custom ID and custom style
            let custom_id = self.custom_id.clone();

            builder = builder.custom_id(&custom_id);

            if let Some(style) = self.style {
                if style != serenity::ButtonStyle::Link {
                    // not a link so
                    builder = builder.style(style);
                }
            }
        }

        builder = builder.disabled(self.disabled);

        if let Some(emoji) = self.emoji.as_ref() {
            builder = builder.emoji(emoji.clone());
        }

        builder
    }
}

/// Will return the absence of a Custom ID
/// if this ButtonSpec contains a link.
/// Otherwise, returns `self.custom_id`.
impl HasCustomId for ButtonSpec {
    fn get_custom_id(&self) -> Option<&CustomId> {
        if self.get_link_if_valid().is_some() {
            // link buttons can't have a Custom ID
            None
        } else {
            Some(&self.custom_id)
        }
    }
}
