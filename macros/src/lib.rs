mod button;
mod form;
mod modal_component;
mod util;

use proc_macro::TokenStream;

/// Derives an InteractionForm implementation for your struct.
///
/// Provides (optional) global helper attributes `#[data = DataType]` (a data class that should be given to your components
/// if you wish for them to share data between them) and `#[on_finish = function_name]` (a function that should run after the
/// form is created).
///
/// For the fields, you have the helper attributes `#[modal]` (to indicate a field implements `ModalFormComponent`),
/// and `#[component]` (implements `MessageFormComponent`). Note that fields themselves must be the components,
/// without any indirection (other than, possibly, `Option<Component>`, but note that this does not mean
/// anything special; it will always be a `Some<Component>` in the end). The components are expected to
/// contain the data they collected from the user.
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
/// #[derive(InteractionForm)]
/// #[data = "MyDataType"]
/// #[on_finish = my_function]
/// pub struct MyForm {
///     #[modal]
///     modal_answers: MyModal,  // first we run the modal
///
///     #[component]
///     selection_menu: MySelectionMenuComponent,  // then this
///
///     #[component]
///     buttons: MyButtonsComponent,  // then this
/// }
/// ```
#[proc_macro_derive(InteractionForm, attributes(data, on_finish, modal, component))]
pub fn form(input: TokenStream) -> TokenStream {
    let struct_ = syn::parse_macro_input!(input as syn::DeriveInput);

    match form::form(struct_) {
        Ok(x) => x,
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
    attributes(
        data,
        label,
        label_function,
        primary,
        secondary,
        success,
        danger,
        link,
        link_function,
        emoji,
        emoji_function,
        disabled,
        disabled_function,
        message_content,
        message_content_function,
        message_attachment_function,
        message_allowed_mentions_function,
        message_embed_function,
        message_is_reply,
        message_ephemeral,
        message_ephemeral_function,
        interaction
    )
)]
pub fn button(input: TokenStream) -> TokenStream {
    let struct_ = syn::parse_macro_input!(input as syn::DeriveInput);

    match button::button(struct_) {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}
