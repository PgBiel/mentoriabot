use std::collections::HashMap;

use poise::serenity_prelude as serenity;

use super::SelectComponent;
use crate::interaction::{self, CustomId};

pub struct SelectMenuOption<Data = ()> {
    /// The option's literal label (please specify this, or a `label_function`)
    pub label: Option<String>,

    /// A function (accepting &Data)
    /// that returns the option's label as a Into<String> (specify this or `label`).
    pub label_function: Option<Box<dyn FnOnce(&Data) -> String>>,

    /// The option's description (small explanation text), optional.
    pub description: Option<String>,

    /// An optional single emoji to display near the label
    pub emoji: Option<serenity::ReactionType>,

    /// Function that returns the emoji to display near the button label
    /// (takes &Data, returns Into<ReactionType>)
    pub emoji_function: Option<Box<dyn FnOnce(&Data) -> serenity::ReactionType>>,

    /// If this is the default selection option.
    pub default: bool,
}

/// Holds all data necessary to display a Discord select menu.
#[derive(Debug, Default, Clone)]
pub struct SelectMenu<Data = ()> {
    pub interaction: Option<serenity::MessageComponentInteraction>,

    /// Associates a symbolic value with a display message and other info for the user.
    pub values_map: Option<HashMap<String, SelectMenuOption<Data>>>,

    /// If this menu is disabled and cannot be clicked
    pub disabled: bool,

    /// Function that determines if this menu is disabled
    /// (takes &Data, returns bool)
    pub disabled_function: Option<Box<dyn FnOnce(&Data) -> bool>>,
}

impl<D> SelectComponent<D> for SelectMenu<D> {
    fn on_build<'a>(
        &self,
        builder: &'a mut serenity::CreateSelectMenu,
        data: &D,
    ) -> (&'a mut serenity::CreateSelectMenu, interaction::CustomId) {
        let custom_id = CustomId::generate();
        builder = builder.custom_id(custom_id.to_string());

        if let Some(link_function) = self.link_function {
            builder = builder.url(link_function(data));
        } else if let Some(link) = self.link {
            builder = builder.url(link);
        } else if let Some(style) = self.style {
            builder = builder.style(style);
        }

        if let Some(disabled_function) = self.disabled_function {
            builder = builder.disabled(disabled_function(data));
        } else {
            builder = builder.disabled(self.disabled);
        }

        builder = builder.options(|f| {
            for (value, option) in self.values_map {
                f = f.create_option(|f| {
                    f = f.default_selection(option.default);
                    // TODO...
                });
            }
        });

        (builder, custom_id)
    }

    /// Function to run when an interaction response is received.
    fn set_interaction(&mut self, interaction: serenity::MessageComponentInteraction) {
        self.interaction = Some(interaction);
    }
}
