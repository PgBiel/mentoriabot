//! Modals asking for the user's data (name and email).

use poise::Modal;

use crate::{common::ApplicationContext, lib::error::Result};

/// Represents a registration modal,
/// asking for its text data.
#[derive(Modal, Debug, Clone)]
#[name = "Register"]
pub struct RegisterModal {
    #[name = "Name and surname"]
    #[placeholder = "John Doe"]
    #[min_length = 1]
    #[max_length = 100]
    pub name: String,

    #[name = "Email (preferably Gmail)"]
    #[placeholder = "john.doe@gmail.com"]
    #[min_length = 1]
    #[max_length = 128]
    pub email: String,
}

/// Same as [`RegisterModal`], but in portuguese.
#[derive(Modal, Debug, Clone)]
#[name = "Cadastro de Informações"]
pub struct RegisterPortugueseModal {
    #[name = "Nome e sobrenome"]
    #[placeholder = "João Silva"]
    #[min_length = 1]
    #[max_length = 100]
    pub name: String,

    #[name = "E-mail (Gmail, de preferência)"]
    #[placeholder = "joao.silva@gmail.com"]
    #[min_length = 1]
    #[max_length = 128]
    pub email: String,
}

/// Groups together the two modals.
#[derive(Debug, Clone)]
pub enum RegisterModals {
    Regular(RegisterModal),
    Portuguese(RegisterPortugueseModal),
}

impl RegisterModals {
    /// Executes either the English version of the Modal or
    /// the Portuguese one, based on the current context locale.
    pub async fn execute_based_on_locale(ctx: ApplicationContext<'_>) -> Result<Option<Self>> {
        Ok(match ctx.locale() {
            Some("pt-BR") => RegisterPortugueseModal::execute(ctx)
                .await?
                .map(Self::Portuguese),
            _ => RegisterModal::execute(ctx).await?.map(Self::Regular),
        })
    }

    /// Gets the user's given name on registration.
    pub fn name(&self) -> &String {
        match self {
            Self::Regular(RegisterModal { name, .. })
            | Self::Portuguese(RegisterPortugueseModal { name, .. }) => name,
        }
    }

    /// Gets the user's given email on registration.
    pub fn email(&self) -> &String {
        match self {
            Self::Regular(RegisterModal { email, .. })
            | Self::Portuguese(RegisterPortugueseModal { email, .. }) => email,
        }
    }
}
