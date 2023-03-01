use poise::serenity_prelude as serenity;
use tokio::sync::watch;
use tokio::task;
use std::{time, sync::Arc};

#[derive(Debug)]
struct PendingInteraction {
    kind: serenity::InteractionType,
    sender: watch::Sender<Option<serenity::Interaction>>,
    custom_id: String,
}

impl PendingInteraction {
    fn new(kind: serenity::InteractionType,
           sender: watch::Sender<Option<serenity::Interaction>>,
           custom_id: String) -> Self {
        PendingInteraction { kind, sender, custom_id }
    }
}

pub struct InteractionHandler {
    pending_cache: Arc<retainer::Cache<String, PendingInteraction>>,
    cache_monitor: task::JoinHandle<()>
}

impl InteractionHandler {
    pub fn new() -> Self {
        let pending_cache = Arc::new(retainer::Cache::new());
        let cache_clone = Arc::clone(&pending_cache);  // so it can be moved into the async scope

        let cache_monitor = tokio::spawn(async move {
            cache_clone
                .monitor(/*sample=*/3, /*threshold=*/0.25, /*frequency=*/time::Duration::from_secs(3))
                .await
        });

        InteractionHandler { pending_cache, cache_monitor }
    }

    /// Given an interaction's unique Custom ID and its kind, returns a `tokio::sync::watch::Receiver`
    /// which can be used to await for the user's response to this interaction with `receiver.changed().await`.
    /// This method returns `None` if some failure occurred while attempting to add the interaction
    /// to the awaiting list.
    ///
    /// Additionally, while the Receiver has a type of `Option<serenity::Interaction>`, it should never
    /// return `None` unless `receiver.has_changed()` is false. (All further data will be wrapped in a `Some`.)
    pub async fn wait_for_interaction(&self, custom_id: String, kind: serenity::InteractionType)
            -> Option<watch::Receiver<Option<serenity::Interaction>>> {
        let (tx, rx) = watch::channel::<Option<serenity::Interaction>>(None);
        println!("[WAIT_FOR_INT] PRE-Waiting for {custom_id}");
        self.pending_cache
            .insert(custom_id.clone(),
                    PendingInteraction::new(kind, /*sender=*/tx, custom_id),
                    time::Duration::from_secs(15*60))
            .await;
        println!("[WAIT_FOR_INT] POST-Waiting.");
        Some(rx)
    }

    pub async fn handle_interaction_create(&self, interaction: serenity::Interaction) {
        println!("[HANDLE_INT_C] Handling: {:?}", interaction);
        let custom_id: String;
        match &interaction {
            serenity::Interaction::MessageComponent(component) => custom_id = component.data.custom_id.clone(),
            serenity::Interaction::ModalSubmit(modal_submit) => custom_id = modal_submit.data.custom_id.clone(),
            _ => return
        };
        println!("[HANDLE_INT_C] ID is {custom_id}");
        if let Some(pending_interaction) = self.pending_cache.get(&custom_id).await {
            println!("[HANDLE_INT_C] Inner 1");
            // remove from cache if could not send (e.g. due to missing receivers)
            if let Err(_) = pending_interaction.sender.send(Some(interaction)) {
                println!("[HANDLE_INT_C] Inner 2 (Removing from cache)");
                self.pending_cache.remove(&custom_id).await;
            }
        }
    }
}

impl Drop for InteractionHandler {
    fn drop(&mut self) {
        self.cache_monitor.abort();
    }
}
