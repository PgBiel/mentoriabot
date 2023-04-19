use std::sync::Arc;

use poise::serenity_prelude as serenity;

use crate::{
    common::{ApplicationContext, Data},
    error::Error,
    forms::{ButtonComponent, GenerateReply, InteractionForm, MessageFormComponent},
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

fn label_function(ctx: ApplicationContext<'_>, _: &()) -> String {
    format!("I am {}", ctx.author().name)
}

#[derive(ButtonComponent, GenerateReply, Clone, Debug)]
#[form_data(ctx(Data, Error))]
#[button(label_function = "label_function", danger)]
#[reply(content = "bruh", ephemeral)]
pub struct Button(#[interaction] Arc<serenity::MessageComponentInteraction>);

#[derive(MessageFormComponent, GenerateReply, Clone, Debug)]
#[forms(form_data(ctx(Data, Error)))]
#[form_data(ctx(Data, Error))]
#[reply(content = "hey", ephemeral)]
pub struct MyButtonComponent {
    #[field(button)]
    my_button: Option<Button>,
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

#[derive(InteractionForm, Debug, Clone)]
#[form_data(ctx(Data, Error))]
pub struct TestForm {
    #[modal]
    modal_answers: TestFormModal,

    // #[component]
    // first_sel_comp: TestFormFirstSelection,
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
