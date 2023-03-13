use poise::serenity_prelude as serenity;

use crate::{
    component::Buildable,
    interaction::{CustomId, SelectValue},
};

/// Holds data regarding a particular Select Menu option.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SelectMenuOptionSpec {
    /// The option's literal label
    pub label: String,

    /// Value key that maps to this option when an interaction response is received.
    pub value_key: SelectValue,

    /// The option's description (small explanation text), optional.
    pub description: Option<String>,

    /// An optional single emoji to display near the label
    pub emoji: Option<serenity::ReactionType>,

    /// If this is the default selection option.
    pub is_default: bool,
}

/// Holds all data necessary to display a Discord select menu.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SelectMenuSpec {
    /// The button's fixed custom ID; if unspecified,
    /// it is auto-generated.
    pub custom_id: CustomId,

    /// Associates a symbolic value with a display message and other info for the user.
    pub options: Vec<SelectMenuOptionSpec>,

    /// Minimum amount of options the user must select.
    pub min_values: Option<u64>,

    /// Maximum amount of options the user can select.
    pub max_values: Option<u64>,

    /// If this menu is disabled and cannot be clicked
    pub disabled: bool,
}

impl Buildable<serenity::CreateSelectMenu> for SelectMenuSpec {
    fn on_build<'a>(
        &self,
        mut builder: &'a mut serenity::CreateSelectMenu,
    ) -> &'a mut serenity::CreateSelectMenu {
        builder = builder.custom_id(self.custom_id.clone());

        if let Some(min_values) = self.min_values {
            builder = builder.min_values(min_values);
        }

        if let Some(max_values) = self.max_values {
            builder = builder.max_values(max_values);
        }

        builder = builder.disabled(self.disabled);

        if !self.options.is_empty() {
            builder = builder.options(|mut f| {
                for option in &self.options {
                    let value = &option.value_key;
                    f = f.create_option(|mut f| {
                        f = f.value(value).default_selection(option.is_default);

                        if let Some(emoji) = option.emoji.as_ref() {
                            f = f.emoji(emoji.clone());
                        }

                        if let Some(description) = option.description.as_ref() {
                            f = f.description(description);
                        }

                        f = f.label(&option.label);

                        f
                    });
                }

                f
            });
        }

        builder
    }
}
