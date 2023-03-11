use poise::{serenity_prelude as serenity, ApplicationContext};

use crate::interaction::{self, CustomId};

/// Holds all data necessary to display a Discord button.
#[derive(Default)]
pub struct ButtonSpec<ContextData, ContextError, Data = ()> {
    /// The button's literal label (please specify this, or a `label_function`)
    pub label: Option<String>,

    /// A function (accepting context and &Data)
    /// that returns the button's label as a Into<String> (specify this or `label`).
    pub label_function:
        Option<Box<dyn Fn(ApplicationContext<'_, ContextData, ContextError>, &Data) -> String>>,

    /// The button's fixed custom ID; if unspecified,
    /// it is auto-generated.
    pub custom_id: Option<CustomId>,

    pub style: Option<serenity::ButtonStyle>,

    /// Makes the button lead to the given link
    /// NOTE: Such a button cannot be awaited for.
    pub link: Option<String>,

    /// Function takes context and &Data, and returns the link the button leads to (Into<String>).
    pub link_function:
        Option<Box<dyn Fn(ApplicationContext<'_, ContextData, ContextError>, &Data) -> String>>,

    /// An optional single emoji to display near the label
    pub emoji: Option<serenity::ReactionType>,

    /// Function that returns the emoji to display near the button label
    /// (takes context and &Data, returns Into<ReactionType>)
    pub emoji_function: Option<
        Box<
            dyn Fn(
                ApplicationContext<'_, ContextData, ContextError>,
                &Data,
            ) -> serenity::ReactionType,
        >,
    >,

    /// If this button is disabled and cannot be clicked
    pub disabled: bool,

    /// Function that determines if this button is disabled
    /// (takes context and &Data, returns bool)
    pub disabled_function:
        Option<Box<dyn Fn(ApplicationContext<'_, ContextData, ContextError>, &Data) -> bool>>,
}

impl<CD, CE, D> ButtonSpec<CD, CE, D> {
    pub fn on_build<'a>(
        &self,
        mut builder: &'a mut serenity::CreateButton,
        context: ApplicationContext<'_, CD, CE>,
        data: &D,
    ) -> (
        &'a mut serenity::CreateButton,
        Option<interaction::CustomId>,
    ) {
        builder = builder.label(
            self.label_function
                .as_ref()
                .map_or_else(|| self.label.clone(), |f| Some(f(context, data)))
                .unwrap_or(String::from("")),
        );

        let mut custom_id = None;

        if let Some(link_function) = &self.link_function {
            builder = builder.url(link_function(context, data));
        } else if let Some(link) = &self.link {
            builder = builder.url(link);
        } else {
            // if not a link button => can have custom ID and custom style
            let custom_id_value = self.custom_id.clone().unwrap_or_else(CustomId::generate);

            builder = builder.custom_id(&custom_id_value);

            custom_id = Some(custom_id_value);

            if let Some(style) = self.style {
                builder = builder.style(style);
            }
        }

        if let Some(disabled_function) = &self.disabled_function {
            builder = builder.disabled(disabled_function(context, data));
        } else {
            builder = builder.disabled(self.disabled);
        }

        if let Some(emoji_function) = &self.emoji_function {
            builder = builder.emoji(emoji_function(context, data));
        } else if let Some(emoji) = self.emoji.as_ref() {
            builder = builder.emoji(emoji.clone());
        }

        (builder, custom_id)
    }
}
