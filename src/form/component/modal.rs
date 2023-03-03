use crate::common::ApplicationContext;
use crate::error::Result;
use crate::form::InteractionForm;
use async_trait::async_trait;

#[async_trait]
pub trait ModalFormComponent {
    type Form: InteractionForm;

    async fn run(
        &self,
        context: ApplicationContext<'_>,
        form_data: Self::Form,
    ) -> Result<Self::Form>;
}

// #[async_trait]
// pub trait ModalFormComponent {
//     type Modal: poise::Modal;
//     type Form: InteractionForm;

//     async fn on_response(&self, modal: Self::Modal, form_data: Self::Form) -> Result<Self::Form>;

//     async fn run(&self, context: ApplicationContext<'_>, form_data: Self::Form) -> Result<Self::Form>
//     {
//         let response = Self::Modal::execute(context).await?;

//         match response {
//             Some(modal) => {
//                 self.on_response(modal, form_data).await
//             },
//             None => Err(FormError::NoResponse.into())
//         }
//     }
// }
