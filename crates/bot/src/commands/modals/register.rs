//! Modals asking for the user's data (name and email).

use lazy_static::lazy_static;
use mentoriabot_lib::util::Unvalidated;
use poise::Modal;

use crate::{
    common::ApplicationContext,
    lib::{
        error::Result,
        model::{DiscordId, NewUser},
    },
};

lazy_static! {
    static ref EMAIL_REGEX: regex::Regex = regex::Regex::new("^.+@.+\\..{2,}$").unwrap();
}

/// Represents a registration modal,
/// asking for its text data.
#[derive(Modal, Debug, Clone)]
#[name = "Register"]
struct RegisterEnglishModal {
    #[name = "Name and surname"]
    #[placeholder = "John Doe"]
    #[min_length = 1]
    #[max_length = 128]
    pub name: String,

    #[name = "Email (preferably Gmail)"]
    #[placeholder = "john.doe@gmail.com"]
    #[min_length = 6]
    #[max_length = 256]
    pub email: String,

    #[name = "Bio (optional)"]
    #[placeholder = "A small description about yourself, if you wish"]
    #[min_length = 0]
    #[max_length = 512]
    #[paragraph]
    pub bio: Option<String>,
}

/// Same as [`RegisterModal`], but in portuguese.
#[derive(Modal, Debug, Clone)]
#[name = "Cadastro de Informações"]
struct RegisterPortugueseModal {
    #[name = "Nome e sobrenome"]
    #[placeholder = "João Silva"]
    #[min_length = 1]
    #[max_length = 128]
    pub name: String,

    #[name = "E-mail (Gmail, de preferência)"]
    #[placeholder = "joao.silva@gmail.com"]
    #[min_length = 6]
    #[max_length = 256]
    pub email: String,

    #[name = "Bio (opcional)"]
    #[placeholder = "Uma pequena descrição sobre si mesmo, se quiser"]
    #[min_length = 0]
    #[max_length = 512]
    #[paragraph]
    pub bio: Option<String>,
}

/// Represents a generic register modal.
#[derive(Debug, validator::Validate, Clone)]
pub struct RegisterModal {
    /// The user's name response.
    #[validate(length(min = 1, max = 128))]
    pub name: String,

    /// The user's email response.
    #[validate(length(min = 6, max = 256))]
    #[validate(regex = "EMAIL_REGEX")]
    pub email: String,

    /// The user's bio response.
    #[validate(length(min = 0, max = 512))]
    pub bio: Option<String>,
}

impl From<RegisterEnglishModal> for RegisterModal {
    fn from(RegisterEnglishModal { name, email, bio }: RegisterEnglishModal) -> Self {
        Self { name, email, bio }
    }
}

impl From<RegisterPortugueseModal> for RegisterModal {
    fn from(RegisterPortugueseModal { name, email, bio }: RegisterPortugueseModal) -> Self {
        Self { name, email, bio }
    }
}

impl RegisterModal {
    /// Executes either the English version of the Modal or
    /// the Portuguese one, based on the current context locale.
    pub async fn execute_based_on_locale(
        ctx: ApplicationContext<'_>,
    ) -> Result<Option<Unvalidated<Self>>> {
        Ok(match ctx.locale() {
            Some("pt-BR") => RegisterPortugueseModal::execute(ctx)
                .await?
                .map(Self::from)
                .map(Unvalidated::new),

            _ => RegisterEnglishModal::execute(ctx)
                .await?
                .map(Self::from)
                .map(Unvalidated::new),
        })
    }

    /// Executes either the English version of the Modal or
    /// the Portuguese one, based on the current context locale,
    /// with some defaults.
    pub async fn execute_with_defaults_based_on_locale(
        ctx: ApplicationContext<'_>,
        name: String,
        email: String,
        bio: Option<String>,
    ) -> Result<Option<Unvalidated<Self>>> {
        // Apply length limits beforehand so Discord doesn't complain
        let name = {
            let mut name = name;
            name.truncate(128);
            name
        };
        let email = {
            let mut email = email;
            email.truncate(256);
            email
        };
        let bio = {
            let mut bio = bio;
            if let Some(bio) = bio.as_mut() {
                bio.truncate(512);
            }
            bio
        };

        Ok(match ctx.locale() {
            Some("pt-BR") => RegisterPortugueseModal::execute_with_defaults(
                ctx,
                RegisterPortugueseModal { name, email, bio },
            )
            .await?
            .map(Self::from)
            .map(Unvalidated::new),

            _ => RegisterEnglishModal::execute_with_defaults(
                ctx,
                RegisterEnglishModal { name, email, bio },
            )
            .await?
            .map(Self::from)
            .map(Unvalidated::new),
        })
    }

    /// Converts this modal response into a 'NewUser' instance.
    pub fn generate_new_user(self, discord_id: DiscordId) -> NewUser {
        NewUser {
            discord_id,
            name: self.name,
            email: self.email,
            bio: self.bio,
        }
    }

    /// Present the modal to the user, and notify them of any possible
    /// validation errors.
    pub async fn ask(ctx: ApplicationContext<'_>) -> Result<Option<Self>> {
        Self::validate_or_warn_user(ctx, Self::execute_based_on_locale(ctx).await?).await
    }

    /// Present the modal to the user with some defaults, and notify them of
    /// any possible validation errors.
    pub async fn ask_with_defaults(
        ctx: ApplicationContext<'_>,
        name: String,
        email: String,
        bio: Option<String>,
    ) -> Result<Option<Self>> {
        Self::validate_or_warn_user(
            ctx,
            Self::execute_with_defaults_based_on_locale(ctx, name, email, bio).await?,
        )
        .await
    }

    /// Notify the user of possible validation errors.
    async fn validate_or_warn_user(
        ctx: ApplicationContext<'_>,
        unvalidated: Option<Unvalidated<Self>>,
    ) -> Result<Option<Self>> {
        match unvalidated.map(Unvalidated::validate).transpose() {
            Err(errs) => {
                ctx.send(|b| {
                    use crate::lib::tr;

                    let content = if errs.field_errors().contains_key("email") {
                        tr!("commands.general.invalid_email", ctx = ctx)
                    } else {
                        tr!("commands.general.invalid_modal_response", ctx = ctx)
                    };

                    b.content(content).ephemeral(true)
                })
                .await?;
                Ok(None)
            }
            Ok(response) => Ok(response),
        }
    }
}
