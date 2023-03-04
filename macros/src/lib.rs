mod util;
mod form;
mod modal_component;

use proc_macro::TokenStream;

#[proc_macro_derive(
    InteractionForm,
    attributes(on_finish, modal, message_component)
)]
pub fn non_modal_form(input: TokenStream) -> TokenStream {
    let struct_ = syn::parse_macro_input!(input as syn::DeriveInput);

    match form::form(struct_, false) {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}

#[proc_macro_derive(
    InteractionModalForm,
    attributes(on_finish, modal, message_component)
)]
pub fn modal_form(input: TokenStream) -> TokenStream {
    let struct_ = syn::parse_macro_input!(input as syn::DeriveInput);

    match form::form(struct_, true) {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}

#[proc_macro_derive(
    ModalFormComponent,
    attributes(form)
)]
pub fn modal_form_component(input: TokenStream) -> TokenStream {
    let struct_ = syn::parse_macro_input!(input as syn::DeriveInput);

    match modal_component::modal_component(struct_) {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}
