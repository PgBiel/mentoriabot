pub struct Data {} // User data, which is stored and accessible in all command invocations

impl Data {
    pub fn new() -> Self {
        Data {}
    }
}

pub type ErrorBox = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, ErrorBox>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, ErrorBox>;
