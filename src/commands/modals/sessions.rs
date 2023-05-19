use poise::Modal;

use crate::{
    common::ApplicationContext,
    lib::{error::Result, util::HumanParseableDateTime},
};

/// Represents a Session creation modal,
/// asking for its text data.
#[derive(Modal, Debug, Clone)]
#[name = "Create Session"]
pub struct SessionCreateModal {
    #[name = "Session Topic"]
    #[placeholder = "Creating a Class in C++"]
    #[min_length = 1]
    #[max_length = 100]
    pub summary: String,

    #[name = "Starting At (DD/MM, hh:mm)"]
    #[placeholder = "19/03, 19:30"]
    #[min_length = 5]
    #[max_length = 25]
    pub starts_at: String,
}

/// Same as [`SessionCreateModal`], but in portuguese.
#[derive(Modal, Debug, Clone)]
#[name = "Criar Aula"]
pub struct SessionCreatePortugueseModal {
    #[name = "Tópico da Aula"]
    #[placeholder = "Criando uma Classe em C++"]
    #[min_length = 1]
    #[max_length = 100]
    pub summary: String,

    #[name = "Começando em (DIA/MÊS, hora:minuto)"]
    #[placeholder = "19/03, 19:30"]
    #[min_length = 5]
    #[max_length = 25]
    pub starts_at: String,
}

/// Groups together the two modals.
#[derive(Debug, Clone)]
pub enum SessionCreateModals {
    Regular(SessionCreateModal),
    Portuguese(SessionCreatePortugueseModal),
}

impl SessionCreateModals {
    /// Executes either the English version of the Modal or
    /// the Portuguese one, based on the current context locale.
    pub async fn execute_based_on_locale(ctx: ApplicationContext<'_>) -> Result<Option<Self>> {
        Ok(match ctx.locale() {
            Some("pt-BR") => SessionCreatePortugueseModal::execute(ctx)
                .await?
                .map(Self::Portuguese),
            _ => SessionCreateModal::execute(ctx).await?.map(Self::Regular),
        })
    }

    /// Gets the user's given Session summary.
    pub fn summary(&self) -> &String {
        match self {
            Self::Regular(SessionCreateModal { summary, .. }) => summary,
            Self::Portuguese(SessionCreatePortugueseModal { summary, .. }) => summary,
        }
    }

    /// Gets the user's given Session 'starts_at' timestamp string.
    pub fn starts_at(&self) -> &String {
        match self {
            Self::Regular(SessionCreateModal { starts_at, .. }) => starts_at,
            Self::Portuguese(SessionCreatePortugueseModal { starts_at, .. }) => starts_at,
        }
    }

    /// Attempts to parse the given Session 'starts_at' timestamp string
    /// as a [`chrono::DateTime`] with the [`chrono::Utc`] timezone.
    ///
    /// # See also
    ///
    /// [`HumanParseableDateTime`]
    pub fn parsed_starts_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        let starts_at = self.starts_at();

        starts_at
            .parse::<HumanParseableDateTime>()
            .ok()
            .map(Into::into)
    }
}
