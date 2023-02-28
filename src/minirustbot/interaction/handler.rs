use poise::serenity_prelude as serenity;
use tokio::sync::watch;

#[derive(Debug)]
struct PendingInteraction {
    kind: serenity::InteractionType,
    sender: watch::Sender<Option<serenity::Interaction>>,
    custom_id: String
}

#[derive(Debug, Default)]
pub struct InteractionHandler {
    pending: Vec<PendingInteraction>
}

impl InteractionHandler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn wait_for_interaction(&mut self, custom_id: String, kind: serenity::InteractionType)
            -> watch::Receiver<Option<serenity::Interaction>> {
        let (tx, rx) = watch::channel::<Option<serenity::Interaction>>(None);
        self.pending.push(PendingInteraction { kind, sender: tx, custom_id });
        rx
    }
}
