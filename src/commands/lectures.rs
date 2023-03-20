use std::ops::Add;
use poise::Modal;
use poise::serenity_prelude as serenity;
use crate::{common::{ApplicationContext, Context}, error::Result, util};
use crate::error::Error;
use crate::model::{Lecture, NewLecture, NewUser, User};
use crate::repository::Repository;
use crate::util::{brazil_timezone, HumanParseableDateTime};

#[derive(Modal, Debug, Clone)]
#[name = "Create Lecture"]
struct LectureCreateModal {
    #[name = "Lecture Topic"]
    #[placeholder = "Creating a Class in C++"]
    #[min_length = 1]
    #[max_length = 100]
    name: String,

    #[name = "Starting At (DD/MM, hh:mm)"]
    #[placeholder = "19/03, 19:30"]
    #[min_length = 5]
    #[max_length = 25]
    starts_at: String,

    #[name = "Lecture Description"]
    #[placeholder = "This lecture will teach you how to create a class in C++, from scratch."]
    #[paragraph]
    #[min_length = 1]
    #[max_length = 200]
    description: String,
}

#[derive(Modal, Debug, Clone)]
#[name = "Criar Aula"]
struct LectureCreatePortugueseModal {
    #[name = "Tópico da Aula"]
    #[placeholder = "Criando uma Classe em C++"]
    #[min_length = 1]
    #[max_length = 100]
    name: String,

    #[name = "Começando em (DIA/MÊS, hora:minuto)"]
    #[placeholder = "19/03, 19:30"]
    #[min_length = 5]
    #[max_length = 25]
    starts_at: String,

    #[name = "Descrição da Aula"]
    #[placeholder = "Esta aula irá te ensinar a criar uma classe em C++, do zero."]
    #[paragraph]
    #[min_length = 1]
    #[max_length = 200]
    description: String,
}

#[derive(Debug, Clone)]
enum LectureCreateModals {
    Regular(LectureCreateModal),
    Portuguese(LectureCreatePortugueseModal)
}

/// Manages lectures.
#[poise::command(
    slash_command,
    ephemeral,
    name_localized("pt-BR", "aulas"),
    description_localized("pt-BR", "Gerencia aulas."),
    subcommands("create", "get", "remove", "all")
)]
pub async fn lectures(ctx: Context<'_>) -> Result<()> {
    ctx.send(|reply| reply.content("Please specify a subcommand").ephemeral(true))
        .await?;
    Ok(())
}

/// Creates a lecture at a certain date.
#[poise::command(
    slash_command,
    ephemeral,
    owners_only,
    name_localized("pt-BR", "criar"),
    description_localized("pt-BR", "Cria uma aula.")
)]
async fn create(
    ctx: ApplicationContext<'_>,

    // #[name_localized("pt-BR", "nome")]
    // #[description_localized("pt-BR", "O tópico desta aula")]
    // #[description = "This lecture's topic"]
    // name: String,
    //
    // #[name_localized("pt-BR", "descrição")]
    // #[description_localized("pt-BR", "Uma curta descrição para esta aula")]
    // #[description = "A brief summary of this lecture"]
    // description: String,

    #[name_localized("pt-BR", "vagas")]
    #[description_localized("pt-BR", "Número máximo de alunos na aula")]
    #[description = "Max amount of students which can take the class"]
    #[min = 1]
    #[max = 256]
    student_limit: i32,

    // #[name_localized("pt-BR", "começa_em")]
    // #[description_localized(
    //     "pt-BR",
    //     "Quando esta aula irá começar, no formato: [DD/MM/YYYY] HH:MM[:SS]"
    // )]
    // #[description = "When this lecture is planned to start"]
    // starts_at: HumanParseableDateTime,

    #[name_localized("pt-BR", "horas")]
    #[description_localized("pt-BR", "A duração desta aula, em horas")]
    #[description = "The duration of this lecture, in hours"]
    #[min = 0]
    #[max = 12]
    hours: i64,
) -> Result<()> {
    let name: String;
    let description: String;
    let starts_at: chrono::DateTime<chrono::Utc>;

    let modal_res: Option<LectureCreateModals> = if ctx.locale() == Some("pt-BR") {
        LectureCreatePortugueseModal::execute(ctx).await?.map(|modal| {
            LectureCreateModals::Portuguese(modal)
        })
    } else {
        LectureCreateModal::execute(ctx).await?.map(|modal| {
            LectureCreateModals::Regular(modal)
        })
    };

    let unparsed_starts_at: String;

    match modal_res {
        Some(LectureCreateModals::Portuguese(LectureCreatePortugueseModal {
            name: given_name,
            description: given_description,
            starts_at: given_starts_at,
        })) => {
            name = given_name;
            description = given_description;
            unparsed_starts_at = given_starts_at;
        }
        Some(LectureCreateModals::Regular(LectureCreateModal {
            name: given_name,
            description: given_description,
            starts_at: given_starts_at,
        })) => {
            name = given_name;
            description = given_description;
            unparsed_starts_at = given_starts_at;
        }
        None => return {
            ctx.send(|b| {
                b.content("Could not receive your response to the modal, sorry.")
            }).await?;
            Ok(())
        }
    };

    match unparsed_starts_at.parse::<HumanParseableDateTime>() {
        Ok(HumanParseableDateTime(date_time)) => starts_at = date_time,
        Err(_) => return {
            ctx.send(|b| {
                b
                    .content(format!(
                        "Sorry, I could not parse the date '{}'. Please use the format \
                        `DD/MM/YYYY HH:MM`.",
                        unparsed_starts_at
                    ))
                    .ephemeral(true)
            }).await?;

            Ok(())
        }
    };

    if starts_at <= chrono::Utc::now().add(chrono::Duration::seconds(1)) {
        ctx.send(
            |b| b.content("Please provide a future timestamp for 'starts_at'."))
            .await?;
        return Ok(());
    }
    let author = ctx.author();
    let author_id = author.id.into();

    ctx.defer_ephemeral().await?;
    let res = ctx.data().db.user_repository().insert_if_not_exists(&NewUser {
        discord_id: author_id,
        name: author.name.clone(),
        bio: None
    }).await?;
    println!("Default insert: {res}");

    let res = ctx.data().db.lecture_repository().insert(&NewLecture {
        name: name.clone(),
        description: description.clone(),
        teacher_id: author_id,
        student_limit,
        notified: false,
        start_at: starts_at.clone().into(),
        end_at: starts_at.add(chrono::Duration::hours(hours))
    }).await?;

    let Lecture {
        id: inserted_id,
        name: inserted_name,
        // description: inserted_description,
        // teacher_id: inserted_teacher_id,
        // student_limit: inserted_student_limit,
        // notified,
        // start_at: inserted_start_at,
        // end_at: inserted_end_at,
        ..
    } = res;

    ctx.send(|b| b.content(
        if ctx.locale() == Some("pt-BR") {
            format!(
                "Aula '{}' criada com sucesso. \
                (Veja mais informações usando '/aulas obter {}'.) 👍",
                inserted_name,
                inserted_id,
            )
        } else {
            format!(
                "Lecture '{}' created successfully. \
                (View more info with '/lectures get {}'.) 👍",
                inserted_name,
                inserted_id,
            )
        }
    )).await?;
    Ok(())
}

/// Gets a lecture from the database
#[poise::command(
    slash_command,
    ephemeral,
    owners_only,
    name_localized("pt-BR", "obter"),
    description_localized("pt-BR", "Obtém informações sobre uma aula do banco de dados.")
)]
pub async fn get(
    ctx: ApplicationContext<'_>,

    #[description = "ID of the Lecture to get"]
    #[description_localized("pt-BR", "Identificador da aula a obter.")]
    id: i64,
) -> Result<()> {
    let lecture_and_teacher = ctx.data.db.lecture_repository().get_with_teacher(id).await?;
    if let Some((lecture, teacher)) = lecture_and_teacher {
        let Lecture {
            name,
            description,
            teacher_id,
            student_limit,
            start_at,
            end_at,
            ..
        } = lecture;

        let User { name: teacher_name, .. } = teacher;
        let teacher_mention = teacher_id.as_user_mention();

        let timezone = brazil_timezone()
            .ok_or_else(|| Error::Other("Failed to fetch the Brazilian timezone"))?;

        let start_at = start_at.with_timezone(&timezone);
        let duration = end_at.signed_duration_since(start_at);

        ctx.send(|f| {
            f.ephemeral(true).embed(|f| {
                if ctx.locale() == Some("pt-BR") {
                    let duration = util::convert_chrono_duration_to_brazilian_string(duration);
                    f.title(format!("Aula '{}'", name))
                        .field("Professor", format!("{teacher_name} ({teacher_mention})"), true)
                        .field("Começa em", format!("{start_at}"), false)
                        .field("Vagas", format!("{student_limit}"), true)
                        .field("Duração", format!("{duration}"), true)
                        .description(format!("\"{}\"", description))
                        .color(serenity::Colour::BLITZ_BLUE)
                } else {
                    let duration = util::convert_chrono_duration_to_string(duration);
                    f.title(format!("Lecture '{}'", name))
                        .field("Teacher", format!("{teacher_name} ({teacher_mention})"), true)
                        .field("Starts at", format!("{start_at}"), false)
                        .field("Max Students", format!("{student_limit}"), true)
                        .field("Duration", format!("{duration}"), true)
                        .description(format!("\"{}\"", description))
                        .color(serenity::Colour::BLITZ_BLUE)
                }
            })
        })
            .await?;
    } else {
        ctx.send(|b| b.content("Lecture not found.").ephemeral(true))
            .await?;
    }

    Ok(())
}

/// Removes a lecture from the database
#[poise::command(
    slash_command,
    ephemeral,
    owners_only,
    name_localized("pt-BR", "remover"),
    description_localized("pt-BR", "Remove uma aula permanentemente do banco de dados.")
)]
pub async fn remove(
    ctx: ApplicationContext<'_>,

    #[description = "ID of the lecture to remove"]
    #[description_localized("pt-BR", "Identificador da aula a ser removida.")]
    id: i64,
) -> Result<()> {
    let lecture = ctx.data().db.lecture_repository().get(id).await?;
    if let Some(lecture) = lecture {
        ctx.data().db.lecture_repository().remove(&lecture).await?;

        let removed_msg = format!(
            "Successfully removed lecture '{}' from the database.",
            lecture.name
        );
        ctx.send(|b| b.content(removed_msg).ephemeral(true)).await?;
    } else {
        ctx.send(|b| {
            b.content("Unknown lecture (maybe it was already deleted).")
                .ephemeral(true)
        })
            .await?;
    }

    Ok(())
}

#[poise::command(
    slash_command,
    ephemeral,
    owners_only,
    name_localized("pt-BR", "listar"),
    description_localized("pt-BR", "Lista todas as aulas no banco de dados.")
)]
pub async fn all(ctx: ApplicationContext<'_>) -> Result<()> {
    let lectures = ctx.data().db.lecture_repository().find_all().await?;
    let text = if lectures.is_empty() {
        "No lectures registered.".to_string()
    } else {
        let mut text = String::from("Lectures:");
        for lecture in lectures {
            text.push_str(&format!("\n- {:?}", lecture));
        }
        text
    };
    ctx.send(|b| b.content(text).ephemeral(true)).await?;

    Ok(())
}
