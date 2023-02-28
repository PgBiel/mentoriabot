use super::events;

#[derive(Debug, Clone, Default)]
pub struct Data {
    event_handler: events::EventHandler
} // User data, which is stored and accessible in all command invocations

impl Data {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_event_handler(&self) -> &events::EventHandler {
        &self.event_handler
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
