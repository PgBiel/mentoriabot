use std::collections::HashMap;

use poise::serenity_prelude as serenity;

use crate::interaction::{self, CustomId};

/// Holds data regarding a particular Select Menu option.
pub struct SelectMenuOptionSpec<Data = ()> {
    /// The option's literal label (please specify this, or a `label_function`)
    pub label: Option<String>,

    /// A function (accepting &Data)
    /// that returns the option's label as a Into<String> (specify this or `label`).
    pub label_function: Option<Box<dyn Fn(&Data) -> String>>,

    /// The option's description (small explanation text), optional.
    pub description: Option<String>,

    /// An optional single emoji to display near the label
    pub emoji: Option<serenity::ReactionType>,

    /// Function that returns the emoji to display near the button label
    /// (takes &Data, returns Into<ReactionType>)
    pub emoji_function: Option<Box<dyn Fn(&Data) -> serenity::ReactionType>>,

    /// If this is the default selection option.
    pub is_default: bool,
}

/// Holds all data necessary to display a Discord select menu.
#[derive(Default)]
pub struct SelectMenuSpec<Data = ()> {
    /// The button's fixed custom ID; if unspecified,
    /// it is auto-generated.
    pub custom_id: Option<CustomId>,

    /// Associates a symbolic value with a display message and other info for the user.
    pub values_map: Option<HashMap<String, SelectMenuOptionSpec<Data>>>,

    /// Minimum amount of options the user must select.
    pub min_values: Option<u64>,

    /// Maximum amount of options the user can select.
    pub max_values: Option<u64>,

    /// If this menu is disabled and cannot be clicked
    pub disabled: bool,

    /// Function that determines if this menu is disabled
    /// (takes &Data, returns bool)
    pub disabled_function: Option<Box<dyn Fn(&Data) -> bool>>,
}

impl<D> SelectMenuSpec<D> {
    fn on_build<'a>(
        &self,
        mut builder: &'a mut serenity::CreateSelectMenu,
        data: &D,
    ) -> (&'a mut serenity::CreateSelectMenu, interaction::CustomId) {
        let custom_id = self.custom_id.clone().unwrap_or_else(CustomId::generate);
        builder = builder.custom_id(custom_id.clone());

        if let Some(min_values) = self.min_values {
            builder = builder.min_values(min_values);
        }

        if let Some(max_values) = self.max_values {
            builder = builder.max_values(max_values);
        }

        if let Some(disabled_function) = self.disabled_function.as_ref() {
            builder = builder.disabled(disabled_function(data));
        } else {
            builder = builder.disabled(self.disabled);
        }

        if let Some(values_map) = self.values_map.as_ref() {
            builder = builder.options(|mut f| {
                for (value, option) in values_map {
                    f = f.create_option(|mut f| {
                        f = f.value(value).default_selection(option.is_default);

                        if let Some(emoji_function) = option.emoji_function.as_ref() {
                            f = f.emoji(emoji_function(data));
                        } else if let Some(emoji) = option.emoji.as_ref() {
                            f = f.emoji(emoji.clone());
                        }

                        if let Some(description) = option.description.as_ref() {
                            f = f.description(description);
                        }

                        if let Some(label_function) = option.label_function.as_ref() {
                            f = f.label(label_function(data));
                        } else {
                            f = f.label(option.label.clone().unwrap_or(String::from("")));
                        }

                        f
                    });
                }

                f
            });
        }

        (builder, custom_id)
    }
}
