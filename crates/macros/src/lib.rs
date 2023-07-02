mod button;
mod common;
mod component;
mod form;
mod modal_component;
mod model;
mod reply;
mod select;
mod select_option;
mod util;

use proc_macro::TokenStream;

/// Derives an InteractionForm implementation for your struct.
///
/// Provides (optional) global helper attributes `#[data = DataType]` (a data class that should be
/// given to your components if you wish for them to share data between them) and `#[on_finish =
/// function_name]` (a function that should run after the form is created).
///
/// For the fields, you have the helper attributes `#[modal]` (to indicate a field implements
/// `ModalFormComponent`), and `#[component]` (implements `MessageFormComponent`). Note that fields
/// themselves must be the components, without any indirection (other than, possibly,
/// `Option<Component>`, but note that this does not mean anything special; it will always be a
/// `Some<Component>` in the end). The components are expected to contain the data they collected
/// from the user.
///
/// Use `YourStruct::execute(application_context)` to run. It returns a `Result`
/// with the generated `YourStruct` object.
///
/// The modal is always executed first, and can only be run once (due to Discord limitations).
/// The components are run in the order of the given fields (always after the modal).
///
/// Usage:
///
/// ```
/// use minirustbot_macros::InteractionForm;
///
/// #[derive(InteractionForm)]
/// #[form_data(data(MyDataType), ctx(Data, Error))]
/// #[on_finish(my_function)]
/// pub struct MyForm {
///     #[modal]
///     modal_answers: MyModal, // first we run the modal
///
///     #[component]
///     selection_menu: MySelectionMenuComponent, // then this
///
///     #[component]
///     buttons: MyButtonsComponent, // then this
/// }
/// ```
#[proc_macro_derive(
    InteractionForm,
    attributes(form_data, on_finish, modal, component, from_data_field)
)]
pub fn form(input: TokenStream) -> TokenStream {
    let struct_ = syn::parse_macro_input!(input as syn::DeriveInput);

    match form::form(struct_) {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}

#[proc_macro_derive(
    MessageFormComponent,
    attributes(forms, field, form_data, component, initializer)
)]
pub fn component(input: TokenStream) -> TokenStream {
    let struct_ = syn::parse_macro_input!(input as syn::DeriveInput);

    match component::component(struct_) {
        Ok(x) => x.into(),
        Err(e) => e.write_errors().into(),
    }
}

#[proc_macro_derive(ModalFormComponent, attributes(form))]
pub fn modal_form_component(input: TokenStream) -> TokenStream {
    let struct_ = syn::parse_macro_input!(input as syn::DeriveInput);

    match modal_component::modal_component(struct_) {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}

#[proc_macro_derive(
    ButtonComponent,
    attributes(form_data, button, interaction, initializer)
)]
pub fn button(input: TokenStream) -> TokenStream {
    let struct_ = syn::parse_macro_input!(input as syn::DeriveInput);

    match button::button(struct_) {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}

#[proc_macro_derive(SelectMenuComponent, attributes(form_data, select, field))]
pub fn select(input: TokenStream) -> TokenStream {
    let struct_ = syn::parse_macro_input!(input as syn::DeriveInput);

    match select::select(struct_) {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}

#[proc_macro_derive(
    SelectOption,
    attributes(
        form_data,
        label,
        label_function,
        value_key,
        description,
        description_function,
        emoji,
        emoji_function,
        is_default,
        initializer,
    )
)]
pub fn select_option(input: TokenStream) -> TokenStream {
    let enum_ = syn::parse_macro_input!(input as syn::DeriveInput);

    match select_option::select_option(enum_) {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}

#[proc_macro_derive(GenerateReply, attributes(form_data, reply,))]
pub fn reply(input: TokenStream) -> TokenStream {
    let struct_ = syn::parse_macro_input!(input as syn::DeriveInput);

    match reply::reply(struct_) {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}
