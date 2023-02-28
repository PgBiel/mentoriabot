use poise::serenity_prelude as serenity;
use poise::Event;
use super::common;

#[derive(Debug, Clone, Default)]
pub struct EventHandler {}

impl EventHandler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn handle<'a>(&self,
                      ctx: &'a serenity::Context,
                      event: &'a Event<'a>,
                      framework_context: poise::FrameworkContext<'a, common::Data, common::Error>,
                      data: &'a common::Data)
            -> poise::BoxFuture<'a, Result<(), common::Error>> {
        Box::pin(async move {
            if let Event::InteractionCreate { interaction } = event {
                match interaction {
                    serenity::Interaction::ApplicationCommand(command) => {
                        println!("Got command: {:?}", command);
                    },
                    serenity::Interaction::MessageComponent(component) => {
                        println!("Got component: {:?}", component);
                        println!("Kind: {:?}", component.data.component_type);
                        component.create_interaction_response(ctx.http.clone(), |response|
                            response
                                .kind(serenity::InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|response_data|
                                    response_data.content("Bruh").ephemeral(true))).await?;
                    },
                    serenity::Interaction::ModalSubmit(modal_submit) => {
                        println!("Got modal submit: {:?}", modal_submit);
                    },
                    _ => ()
                };
            }
            Ok(())
        })
    }
}
