use poise::Modal;
use crate::{common::ApplicationContext, error::Result};
use crate::util::HumanParseableDateTime;

/// Represents a Lecture creation modal,
/// asking for its text data.
#[derive(Modal, Debug, Clone)]
#[name = "Create Lecture"]
pub struct LectureCreateModal {
    #[name = "Lecture Topic"]
    #[placeholder = "Creating a Class in C++"]
    #[min_length = 1]
    #[max_length = 100]
    pub name: String,

    #[name = "Starting At (DD/MM, hh:mm)"]
    #[placeholder = "19/03, 19:30"]
    #[min_length = 5]
    #[max_length = 25]
    pub starts_at: String,

    #[name = "Lecture Description"]
    #[placeholder = "This lecture will teach you how to create a class in C++, from scratch."]
    #[paragraph]
    #[min_length = 1]
    #[max_length = 200]
    pub description: String,
}

/// Same as [`LectureCreateModal`], but in portuguese.
#[derive(Modal, Debug, Clone)]
#[name = "Criar Aula"]
pub struct LectureCreatePortugueseModal {
    #[name = "Tópico da Aula"]
    #[placeholder = "Criando uma Classe em C++"]
    #[min_length = 1]
    #[max_length = 100]
    pub name: String,

    #[name = "Começando em (DIA/MÊS, hora:minuto)"]
    #[placeholder = "19/03, 19:30"]
    #[min_length = 5]
    #[max_length = 25]
    pub starts_at: String,

    #[name = "Descrição da Aula"]
    #[placeholder = "Esta aula irá te ensinar a criar uma classe em C++, do zero."]
    #[paragraph]
    #[min_length = 1]
    #[max_length = 200]
    pub description: String,
}

/// Groups together the two modals.
#[derive(Debug, Clone)]
pub enum LectureCreateModals {
    Regular(LectureCreateModal),
    Portuguese(LectureCreatePortugueseModal)
}

impl LectureCreateModals {
    /// Executes either the English version of the Modal or
    /// the Portuguese one, based on the current context locale.
    pub async fn execute_based_on_locale(
        ctx: ApplicationContext<'_>
    ) -> Result<Option<Self>> {
        Ok(match ctx.locale() {
            Some("pt-BR") => {
                LectureCreatePortugueseModal::execute(ctx)
                    .await?
                    .map(Self::Portuguese)
            },
            _ => {
                LectureCreateModal::execute(ctx)
                    .await?
                    .map(Self::Regular)
            },
        })
    }

    /// Gets the user's given Lecture name.
    pub fn name(&self) -> &String {
        match self {
            Self::Regular(LectureCreateModal { name, .. }) => name,
            Self::Portuguese(LectureCreatePortugueseModal { name, .. }) => name,
        }
    }

    /// Gets the user's given Lecture 'starts_at' timestamp string.
    pub fn starts_at(&self) -> &String {
        match self {
            Self::Regular(LectureCreateModal { starts_at, .. }) => starts_at,
            Self::Portuguese(LectureCreatePortugueseModal { starts_at, .. }) => starts_at,
        }
    }

    /// Gets the user's given Lecture description.
    pub fn description(&self) -> &String {
        match self {
            Self::Regular(LectureCreateModal { description, .. }) => description,
            Self::Portuguese(LectureCreatePortugueseModal { description, .. }) => description,
        }
    }

    /// Attempts to parse the given Lecture 'starts_at' timestamp string
    /// as a [`chrono::DateTime`] with the [`chrono::Utc`] timezone.
    /// 
    /// # See also
    /// 
    /// [`HumanParseableDateTime`]
    pub fn parsed_starts_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        let starts_at = self.starts_at();

        starts_at.parse::<HumanParseableDateTime>().ok()
            .map(Into::into)
    }
}
