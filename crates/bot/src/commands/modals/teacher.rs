//! Modals asking for the mentor's basic data (name, email, specialty, bio).

use lazy_static::lazy_static;
use poise::Modal;

use crate::{
    common::ApplicationContext,
    lib::{error::Result, model::NewTeacher, util::Unvalidated},
};

lazy_static! {
    static ref EMAIL_REGEX: regex::Regex = regex::Regex::new("^.+@.+\\..{2,}$").unwrap();
}

/// Represents a teacher registration modal,
/// asking for its text data.
#[derive(Modal, Debug, Clone)]
#[name = "Register Mentor"]
struct TeacherEnglishModal {
    #[name = "Name and surname"]
    #[placeholder = "John Doe"]
    #[min_length = 1]
    #[max_length = 512]
    pub name: String,

    #[name = "Email"]
    #[placeholder = "john.doe@gmail.com"]
    #[min_length = 6]
    #[max_length = 512]
    pub email: String,

    #[name = "Specialty"]
    #[placeholder = "Calculus and Computer Science"]
    #[min_length = 1]
    #[max_length = 512]
    pub specialty: String,

    #[name = "Bio (optional)"]
    #[placeholder = "A small description about them"]
    #[min_length = 0]
    #[max_length = 512]
    #[paragraph]
    pub bio: Option<String>,
}

/// Same as [`TeacherModal`], but in portuguese.
#[derive(Modal, Debug, Clone)]
#[name = "Cadastrar Mentor"]
struct TeacherPortugueseModal {
    #[name = "Nome e sobrenome"]
    #[placeholder = "João Silva"]
    #[min_length = 1]
    #[max_length = 512]
    pub name: String,

    #[name = "E-mail"]
    #[placeholder = "joao.silva@gmail.com"]
    #[min_length = 6]
    #[max_length = 512]
    pub email: String,

    #[name = "Especialidade"]
    #[placeholder = "Calculus and Computer Science"]
    #[min_length = 1]
    #[max_length = 512]
    pub specialty: String,

    #[name = "Bio (opcional)"]
    #[placeholder = "Uma pequena descrição sobre o mentor"]
    #[min_length = 0]
    #[max_length = 512]
    #[paragraph]
    pub bio: Option<String>,
}

/// Represents a generic teacher registration modal.
#[derive(Debug, validator::Validate, Clone)]
pub struct TeacherModal {
    /// The mentor's name.
    #[validate(length(min = 1, max = 512))]
    pub name: String,

    /// The mentor's email.
    #[validate(length(min = 6, max = 512))]
    #[validate(regex = "EMAIL_REGEX")]
    pub email: String,

    /// The mentor's specialty.
    #[validate(length(min = 6, max = 512))]
    #[validate(regex = "EMAIL_REGEX")]
    pub specialty: String,

    /// The mentor's bio.
    #[validate(length(min = 0, max = 512))]
    pub bio: Option<String>,
}

impl From<TeacherEnglishModal> for TeacherModal {
    fn from(
        TeacherEnglishModal {
            name,
            email,
            specialty,
            bio,
        }: TeacherEnglishModal,
    ) -> Self {
        Self {
            name,
            email,
            specialty,
            bio,
        }
    }
}

impl From<TeacherPortugueseModal> for TeacherModal {
    fn from(
        TeacherPortugueseModal {
            name,
            email,
            specialty,
            bio,
        }: TeacherPortugueseModal,
    ) -> Self {
        Self {
            name,
            email,
            specialty,
            bio,
        }
    }
}

impl TeacherModal {
    /// Executes either the English version of the Modal or
    /// the Portuguese one, based on the current context locale.
    pub async fn execute_based_on_locale(
        ctx: ApplicationContext<'_>,
    ) -> Result<Option<Unvalidated<Self>>> {
        Ok(match ctx.locale() {
            Some("pt-BR") => TeacherPortugueseModal::execute(ctx)
                .await?
                .map(Self::from)
                .map(Unvalidated::new),

            _ => TeacherEnglishModal::execute(ctx)
                .await?
                .map(Self::from)
                .map(Unvalidated::new),
        })
    }

    /// Executes either the English version of the Modal or
    /// the Portuguese one, based on the current context locale,
    /// with some defaults.
    #[allow(dead_code)]
    pub async fn execute_with_defaults_based_on_locale(
        ctx: ApplicationContext<'_>,
        name: String,
        email: String,
        specialty: String,
        bio: Option<String>,
    ) -> Result<Option<Unvalidated<Self>>> {
        // Apply length limits beforehand so Discord doesn't complain
        let name = {
            let mut name = name;
            name.truncate(512);
            name
        };
        let email = {
            let mut email = email;
            email.truncate(512);
            email
        };
        let specialty = {
            let mut specialty = specialty;
            specialty.truncate(512);
            specialty
        };
        let bio = {
            let mut bio = bio;
            if let Some(bio) = bio.as_mut() {
                bio.truncate(512);
            }
            bio
        };

        Ok(match ctx.locale() {
            Some("pt-BR") => TeacherPortugueseModal::execute_with_defaults(
                ctx,
                TeacherPortugueseModal {
                    name,
                    email,
                    specialty,
                    bio,
                },
            )
            .await?
            .map(Self::from)
            .map(Unvalidated::new),

            _ => TeacherEnglishModal::execute_with_defaults(
                ctx,
                TeacherEnglishModal {
                    name,
                    email,
                    specialty,
                    bio,
                },
            )
            .await?
            .map(Self::from)
            .map(Unvalidated::new),
        })
    }

    /// Converts this modal response into a 'NewTeacher' instance.
    pub fn generate_new_teacher(self) -> NewTeacher {
        NewTeacher {
            name: self.name,
            email: self.email,
            specialty: self.specialty,
            bio: self.bio,
            applied_at: None,
            course_info: None,
            company: None,
            company_role: None,
            whatsapp: None,
            linkedin: None,
            comment_general: None,
            comment_experience: None,
        }
    }

    /// Present the modal to the user, and notify them of any possible
    /// validation errors.
    pub async fn ask(ctx: ApplicationContext<'_>) -> Result<Option<Self>> {
        Self::validate_or_warn_user(ctx, Self::execute_based_on_locale(ctx).await?).await
    }

    /// Present the modal to the user with some defaults, and notify them of
    /// any possible validation errors.
    #[allow(dead_code)]
    pub async fn ask_with_defaults(
        ctx: ApplicationContext<'_>,
        name: String,
        email: String,
        specialty: String,
        bio: Option<String>,
    ) -> Result<Option<Self>> {
        Self::validate_or_warn_user(
            ctx,
            Self::execute_with_defaults_based_on_locale(ctx, name, email, specialty, bio).await?,
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
