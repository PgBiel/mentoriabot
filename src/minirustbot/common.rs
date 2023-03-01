pub struct Data {

} // User data, which is stored and accessible in all command invocations

impl Data {
    pub fn new() -> Self {
        Data {}
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;
