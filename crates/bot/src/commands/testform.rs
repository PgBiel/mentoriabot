#![allow(dead_code)]
use std::sync::Arc;

use poise::serenity_prelude as serenity;

use crate::{
    common::{ApplicationContext, Data},
    forms::{
        ButtonComponent, GenerateReply, InteractionForm, MessageFormComponent, SelectMenuOptionSpec,
    },
    lib::error::Error,
};

#[derive(Debug, Default, Clone, poise::Modal)]
#[name = "Random modal"]
pub struct TestFormModal {
    #[name = "Name"]
    #[placeholder = "Joseph"]
    #[min_length = 3]
    #[max_length = 20]
    name: String,

    #[name = "age"]
    #[max_length = 3]
    age: Option<String>,

    #[name = "More"]
    #[max_length = 500]
    #[paragraph]
    more: Option<String>,
}

// #[derive(SelectOption)]
// #[ctx_data = "Data"]
// #[ctx_error = "Error"]
// pub enum Test {
//     #[label = "Sussy"]
//     #[description = "Omg"]
//     #[is_default]
//     Sus,
//
//     #[label = "Oh no"]
//     Amogus(i32, Option<u64>),
//
//     #[label = "MOMENT"]
//     Rhombus {
//         caustic: i32,
//         mega: u64,
//         test: String,
//         wow: Option<String>,
//
//         #[initializer = "Box::new(Test::Sus)"]
//         giga: Box<Test>,
//     },
// }

#[derive(ButtonComponent, GenerateReply, Clone, Debug)]
#[form_data(ctx(Data, Error))]
#[button(
    label = { format!("I am {}", context.author().name) },
    danger
)]
#[reply(content = "bruh", ephemeral)]
pub struct Button(#[interaction] Arc<serenity::MessageComponentInteraction>);

#[derive(MessageFormComponent, Clone, Debug)]
#[form_data(ctx(Data, Error))]
#[component(reply(content = "hey", ephemeral))]
pub struct MyButtonComponent {
    #[field(button)]
    my_button: Option<Button>,
}

fn select_opts() -> Vec<SelectMenuOptionSpec> {
    vec![
        SelectMenuOptionSpec {
            label: "abcdef".to_string(),
            value_key: "test1".into(),
            description: "ghijkl".to_string().into(),
            ..Default::default()
        },
        SelectMenuOptionSpec {
            label: "abcdefgg".to_string(),
            value_key: "test2".into(),
            description: "ghijkl".to_string().into(),
            ..Default::default()
        },
    ]
}

// #[derive(MessageFormComponent, SelectMenuComponent, GenerateReply, Clone, Debug)]
// #[form_data(ctx(Data, Error))]
// #[select(options = select_opts())]
// #[reply(content = "test", ephemeral)]
// pub struct Selector {}

#[derive(InteractionForm, Debug, Clone)]
#[form_data(ctx(Data, Error))]
pub struct TestForm {
    #[modal]
    modal_answers: TestFormModal,

    #[component]
    button: MyButtonComponent,
}

/// Runs test form
#[poise::command(slash_command)]
pub async fn testform(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let data = TestForm::execute(ctx).await?;
    println!("Got testform data: {:?}", data);
    Ok(())
}
