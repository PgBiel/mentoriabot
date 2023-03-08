use poise::serenity_prelude as serenity;

use super::ButtonComponent;
use crate::interaction::{self, CustomId};

/// Holds all data necessary to display a Discord button.
#[derive(Debug, Default, Clone)]
pub struct Button<Data = ()> {
    pub interaction: Option<serenity::MessageComponentInteraction>,

    /// The button's literal label (please specify this, or a `label_function`)
    pub label: Option<String>,

    /// A function (accepting &Data)
    /// that returns the button's label as a Into<String> (specify this or `label`).
    pub label_function: Option<Box<dyn FnOnce(&Data) -> String>>,

    pub style: serenity::ButtonStyle,

    /// Makes the button lead to the given link
    /// NOTE: Such a button cannot be awaited for.
    pub link: Option<String>,

    /// Function takes &Data, and returns the link the button leads to (Into<String>).
    pub link_function: Option<Box<dyn FnOnce(&Data) -> String>>,

    /// An optional single emoji to display near the label
    pub emoji: Option<serenity::ReactionType>,

    /// Function that returns the emoji to display near the button label
    /// (takes &Data, returns Into<ReactionType>)
    pub emoji_function: Option<Box<dyn FnOnce(&Data) -> serenity::ReactionType>>,

    /// If this button is disabled and cannot be clicked
    pub disabled: bool,

    /// Function that determines if this button is disabled
    /// (takes &Data, returns bool)
    pub disabled_function: Option<Box<dyn FnOnce(&Data) -> bool>>,
}

impl<D> ButtonComponent<D> for Button<D> {
    fn on_build<'a>(
        &self,
        builder: &'a mut serenity::CreateButton,
        data: &D,
    ) -> (
        &'a mut serenity::CreateButton,
        Option<interaction::CustomId>,
    ) {
        builder = builder.label(
            self.label_function
                .map_or(self.label, |f| Some(f(data)))
                .unwrap_or(String::from("")),
        );

        let mut custom_id = None;

        if let Some(link_function) = self.link_function {
            builder = builder.url(link_function(data));
        } else if let Some(link) = self.link {
            builder = builder.url(link);
        } else {
            // if not a link button => can have custom ID and custom style
            let custom_id_value = CustomId::generate();
            custom_id = Some(custom_id_value);

            builder = builder.custom_id(custom_id_value);

            if let Some(style) = self.style {
                builder = builder.style(style);
            }
        }

        if let Some(disabled_function) = self.disabled_function {
            builder = builder.disabled(disabled_function(data));
        } else {
            builder = builder.disabled(self.disabled);
        }

        if let Some(emoji_function) = self.emoji_function {
            builder = builder.emoji(emoji_function(data));
        } else if let Some(emoji) = self.emoji {
            builder = builder.emoji(emoji);
        }

        (builder, custom_id)
    }

    /// Function to run when an interaction response is received.
    fn set_interaction(&mut self, interaction: serenity::MessageComponentInteraction) {
        self.interaction = Some(interaction);
    }
}
