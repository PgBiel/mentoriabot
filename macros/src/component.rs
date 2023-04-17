//! Implements the #[derive(InteractionForm)] derive macro
use darling::util::Flag;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::{
    common::{FormContextInfo, FormData},
    util::{self, parse_option},
};

/// Representation of the struct attributes
#[derive(Debug, darling::FromMeta)]
#[darling(allow_unknown_fields)]
struct StructAttributes {
    /// Gather form type parameters.
    form_data: FormData,

    /// Optionally, an async function with the same signature
    /// as `wait_for_response` to override.
    #[darling(map = "parse_option")]
    wait_for_response: Option<syn::Path>,

    /// Name of the async function to run when the form is finished.
    /// It must take an ApplicationContext object, an `Arc<MessageComponentInteraction>`,
    /// a `&mut FormData`, the constructed `Self` (from its components),
    /// and return a `impl Future<Output = Result<Option<Self>>>`.
    /// Returning `None` indicates the component should be run again,
    /// while returning `Some(Self)` will advance the form.
    #[darling(map = "parse_option")]
    on_response: Option<syn::Path>,
}

/// Representation of the struct field attributes
#[derive(Debug, Default, darling::FromMeta)]
#[darling(allow_unknown_fields, default)]
struct FieldAttributes {
    /// Indicates this field is a single button row.
    button: Flag,

    /// Indicates this field is a row with multiple buttons.
    buttons: Flag,

    /// Indicates this field is a select menu.
    select: Flag,
}

// pub fn form(input: syn::DeriveInput) -> Result<TokenStream, darling::Error> {
//     let fields = match input.data {
//         syn::Data::Struct(syn::DataStruct {
//             fields: syn::Fields::Named(fields),
//             ..
//         }) => fields.named,
//         _ => {
//             return Err(syn::Error::new(
//                 // use Darling errors to indicate visually where the error occurred
//                 input.ident.span(), /* <-- Error will display at the struct's name
//                                      * ('ident'/identity) */
//                 "Only structs with named fields can be used for deriving a Component.",
//             )
//             .into());
//         }
//     };
//
//     let struct_attrs: StructAttributes = util::get_darling_attrs(&input.attrs)?;
//
//     let FormData {
//         data: data_type,
//         ctx: FormContextInfo {
//             data: ctx_data,
//             error: ctx_error,
//         },
//     } = &struct_attrs.form_data;
//
//     let mut components = Vec::new();
//     let mut create_fields = Vec::new();
//     let mut modal_creation: Option<TokenStream2> = None;
//
//     for field in fields {
//         let field_name: &syn::Ident = field.ident.as_ref().expect("Unnamed field");
//         // Extract data from syn::Field
//         let field_attrs: FieldAttributes = util::get_darling_attrs(&field.attrs)?;
//
//         let field_type: &syn::Type = &field.ty;
//
//         let field_inner_type =
//             util::extract_type_parameter("Option", field_type).unwrap_or(field_type);
//
//         if field_attrs.component.is_present() {
//             // is a message component
//             components.push(generate_message_component(
//                 field_name,
//                 field_type,
//                 field_inner_type,
//                 &data_type,
//                 ctx_data,
//                 ctx_error,
//             ));
//             create_fields.push(quote! { #field_name });
//         } else if field_attrs.modal.is_present() {
//             if modal_creation.is_some() {
//                 return Err(syn::Error::new(
//                     syn::spanned::Spanned::span(field_name),
//                     "Multiple #[modal] are not allowed.",
//                 )
//                 .into());
//             }
//
//             modal_creation = Some(generate_modal_creation(
//                 field_name,
//                 /* modal_type: */ field_type,
//                 /* modal_inner_type: */ field_inner_type,
//                 &data_type,
//                 ctx_data,
//                 ctx_error,
//             ));
//             create_fields.push(quote! { #field_name });
//         } else {
//             create_fields.push(quote! { #field_name: Default::default() });
//         }
//     }
//
//     // for '#[on_finish = function_name]'
//     let on_finish = parse_on_finish(&struct_attrs);
//
//     let struct_ident = input.ident; // struct's name as an object
//
//     // get the struct's generics
//     let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
//
//     Ok(quote! { const _: () = {
//         #[::async_trait::async_trait]
//         impl #impl_generics ::minirustbot_forms::MessageFormComponent<#ctx_data, #ctx_error,
// #data_type> for #struct_ident #ty_generics #where_clause {             async fn send_component(
//                 context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
//                 data: &mut #data_type,
//             ) ->
// ::minirustbot_forms::error::ContextualResult<::std::vec::Vec<::minirustbot_forms::interaction::CustomId>> {
//
//                 let __buildables = <Self as ::minirustbot_forms::Subcomponent<#ctx_data,
// #ctx_error, #data_type>>::generate_buildables(context, data).await?;                 let
// __custom_ids = ::minirustbot_forms::util::id_vec_from_has_custom_ids(&__buildables);
//
//                 let __reply = <Self as ::minirustbot_forms::GenerateReply<#ctx_data, #ctx_error,
// #data_type>>::create_reply(context, data).await?;
//
//                 context.send(|f|
//                     <::minirustbot_forms::ReplySpec as
// ::minirustbot_forms::Buildable<::poise::CreateReply>>::on_build(&__reply, f)
// .components(|f| f                             .create_action_row(|mut f| {
//                                 for buildable in &__buildables {
//                                     f = f.create_button(|b|
// ::minirustbot_forms::Buildable::<::poise::serenity_prelude::CreateButton>::on_build(buildable,
// b));                                 }
//                                 f
//                             }))).await?;
//
//                 Ok(__custom_ids.into_iter().map(::core::clone::Clone::clone).collect())
//             }
//
//             async fn on_response(
//                 context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
//                 interaction:
// ::std::sync::Arc<::poise::serenity_prelude::MessageComponentInteraction>,                 data:
// &mut #data_type,             ) ->
// ::minirustbot_forms::error::ContextualResult<::core::option::Option<::std::boxed::Box<Self>>> {
// ::core::result::Result::Ok(::core::option::Option::Some(                     <Self as
// ::minirustbot_forms::Subcomponent<#ctx_data, #ctx_error,
// #data_type>>::build_from_interaction(context, interaction, data)                         .await?
//                 ))
//             }
//         }
//     }; }.into())
// }
//
// fn generate_message_component(
//     field_name: &syn::Ident,
//     field_type: &syn::Type,
//     field_inner_type: &syn::Type,
//     data_type: &syn::Type,
//     ctx_data: &syn::Type,
//     ctx_error: &syn::Type,
// ) -> TokenStream2 {
//     // use .into() in case it's an Option<>, Box<> etc.
//     quote! {
//         let #field_name: #field_type = (*<#field_inner_type as
// ::minirustbot_forms::MessageFormComponent<#ctx_data, #ctx_error, #data_type>>::run(context, &mut
// __component_data).await?).into();     }
// }
//
// fn generate_modal_creation(
//     modal_field_name: &syn::Ident,
//     modal_type: &syn::Type,
//     modal_inner_type: &syn::Type,
//     data_type: &syn::Type,
//     ctx_data: &syn::Type,
//     ctx_error: &syn::Type,
// ) -> TokenStream2 {
//     quote! {
//         let #modal_field_name: #modal_type = (*<#modal_inner_type as
// ::minirustbot_forms::ModalFormComponent<#ctx_data, #ctx_error, #data_type>>::run(context, &mut
// __component_data).await?).into();     }
// }
//
// fn parse_on_finish(struct_attrs: &StructAttributes) -> Option<TokenStream2> {
//     struct_attrs.on_finish.as_ref().map(|on_finish| quote! {
//             async fn on_finish(self, context: ::poise::ApplicationContext<'_, Self::ContextData,
// Self::ContextError>) -> ::minirustbot_forms::error::ContextualResult<::std::boxed::Box<Self>> {
// #on_finish(context).into()             }
//         })
// }
