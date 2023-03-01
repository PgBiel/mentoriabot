use super::interaction;

pub struct Data {
    interaction_handler: interaction::InteractionHandler,
} // User data, which is stored and accessible in all command invocations

impl Data {
    pub fn new() -> Self {
        Data { interaction_handler: interaction::InteractionHandler::new() }
    }

    pub fn interaction_handler(&self) -> &interaction::InteractionHandler {
        &self.interaction_handler
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;
