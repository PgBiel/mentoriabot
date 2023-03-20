use poise::Modal;

#[derive(Default, Modal)]
#[name = "Modal title"]
pub struct TestModal {
    #[name = "Name"]
    #[placeholder = "Joseph"]
    #[min_length = 3]
    #[max_length = 20]
    pub name: String,

    #[name = "age"]
    #[max_length = 3]
    pub age: Option<String>,

    #[name = "More"]
    #[max_length = 500]
    #[paragraph]
    pub more: Option<String>,
}
