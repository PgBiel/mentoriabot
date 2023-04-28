//! Implements the #[derive(InteractionForm)] derive macro
use darling::{util::Flag, FromAttributes, FromDeriveInput};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{
    common::{FormContextInfo, FormData, FormDataAttr},
    util::{self, parse_option},
};

/// Representation of the struct attributes
#[derive(Debug, darling::FromDeriveInput)]
#[darling(supports(struct_named), attributes(component))]
struct ComponentStruct {
    ident: syn::Ident,

    /// The struct's fields
    ///                      vvvvvvvvvvvvvvvvvvvvvv No enum variants allowed!
    data: darling::ast::Data<darling::util::Ignored, SubcomponentField>,

    /// Optionally, an async function with the same signature
    /// as `wait_for_response` to override.
    #[darling(map = "parse_option")]
    wait_for_response: Option<syn::Path>,

    /// Name of the async function to run when the component receives a response.
    /// It must take an ApplicationContext object, an `Arc<MessageComponentInteraction>`,
    /// a `&mut FormData`, the constructed `Self` (from its components),
    /// and return a `impl Future<Output = Result<Option<Self>>>`.
    /// Returning `None` indicates the component should be run again,
    /// while returning `Some(Self)` will advance the form.
    #[darling(map = "parse_option")]
    on_response: Option<syn::Path>,

    /// Indicates this whole struct is a single button row.
    button: Flag,

    /// Indicates this whole struct is a row with multiple buttons.
    buttons: Flag,

    /// Indicates this whole struct is a select menu.
    select: Flag,
}

/// Data from a subcomponent field
// #[darling(attributes(button, buttons, select))]
#[derive(Debug, Clone, darling::FromField)]
#[darling(attributes(field))]
#[darling(allow_unknown_fields)]
struct SubcomponentField {
    ident: Option<syn::Ident>,

    ty: syn::Type,

    /// Indicates this field is a single button row.
    button: Flag,

    /// Indicates this field is a row with multiple buttons.
    buttons: Flag,

    /// Indicates this field is a select menu.
    select: Flag,

    /// If this field should have a custom initializer
    /// (instead of Default::default)
    /// when not specified.
    initializer: Option<syn::Expr>,

    /// This subcomponent's index in the generated Component implementation.
    #[darling(skip)]
    count: Option<i32>,

    /// If this subcomponent is actually the whole component.
    #[darling(skip)]
    is_self: bool,
}

pub fn component(input: syn::DeriveInput) -> Result<TokenStream, darling::Error> {
    let component_struct = ComponentStruct::from_derive_input(&input)?;
    let component_subcomponent = SubcomponentField::from_component(&component_struct);
    let mut subcomponent_count = 0;
    let subcomponents = component_subcomponent
        .into_iter()
        .chain(
            component_struct
                .data
                .clone()
                .take_struct()
                .unwrap()
                .fields
                .into_iter(),
        )
        .map(|s| s.init_counter(&mut subcomponent_count))
        .collect::<Vec<SubcomponentField>>();

    // ---
    // form data
    let form_data = FormDataAttr::from_attributes(&input.attrs)?.form_data;
    let FormData {
        data: data_type,
        ctx: FormContextInfo {
            data: ctx_data,
            error: ctx_error,
        },
    } = &form_data;

    let (
        subcomponent_buildables_sections,
        (subcomponent_row_creators, mut subcomponent_create_from_interaction_ifs),
    ): (Vec<TokenStream>, (Vec<TokenStream>, Vec<TokenStream>)) = subcomponents
        .iter()
        .map(|c| {
            (
                c.gen_buildables_section(ctx_data, ctx_error, data_type),
                (
                    c.create_row(),
                    c.create_from_interaction(ctx_data, ctx_error, data_type, &component_struct),
                ),
            )
        })
        .unzip();

    // pop from stack in the reverse order of pushing
    subcomponent_create_from_interaction_ifs.reverse();

    // 'wait_for_response' override, if any
    let wait_for_response = component_struct.wait_for_response_func(ctx_data, ctx_error, data_type);

    // get the struct's generics
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let struct_ident = component_struct.ident;

    let result = quote! { const _: () = {
        #[::async_trait::async_trait]
        impl #impl_generics ::minirustbot_forms::MessageFormComponent<#ctx_data, #ctx_error, #data_type> for #struct_ident #ty_generics #where_clause {
            async fn send_component(
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                data: &mut ::minirustbot_forms::FormState<#data_type>,
            ) -> ::minirustbot_forms::error::ContextualResult<::std::vec::Vec<::minirustbot_forms::interaction::CustomId>, #ctx_error> {

                data.subcomponent_id_stack.clear();
                let mut __custom_ids = ::std::vec::Vec::new();

                #(#subcomponent_buildables_sections)*

                let __reply = <Self as ::minirustbot_forms::GenerateReply<#ctx_data, #ctx_error, #data_type>>::create_reply(context, data).await?;

                context.send(|f|
                    <::minirustbot_forms::ReplySpec as ::minirustbot_forms::Buildable<::poise::CreateReply>>::on_build(&__reply, f)
                        .components(|mut f| f
                            #(#subcomponent_row_creators)*)).await?;

                Ok(__custom_ids) //.into_iter().map(::core::clone::Clone::clone).collect())
            }

            #wait_for_response

            async fn on_response(
                context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                interaction: ::std::sync::Arc<::poise::serenity_prelude::MessageComponentInteraction>,
                data: &mut ::minirustbot_forms::FormState<#data_type>
            ) -> ::minirustbot_forms::error::ContextualResult<::core::option::Option<::std::boxed::Box<Self>>, #ctx_error> {
                #(#subcomponent_create_from_interaction_ifs)*

                ::core::result::Result::Err(::minirustbot_forms::error::FormError::InvalidUserResponse.into())
            }
        }
    };
    };

    Ok(result)
}

impl SubcomponentField {
    /// Take T from inside an Option<T>.
    fn init_type(mut self) -> Self {
        self.ty = util::extract_type_parameter("Option", &self.ty)
            .unwrap_or(&self.ty)
            .clone();
        self
    }
    /// Inits this instance's counter.
    fn init_counter(mut self, count: &mut i32) -> Self {
        self = self.init_type();
        self.count = Some(*count);
        *count += 1;
        self
    }

    /// Generates the "send_component" section relative to this subcomponent.
    /// Basically calls the buildables generator function, gets their custom IDs,
    /// and then pushes their custom IDs to the ID stack (so that we can compare them later,
    /// in order).
    fn gen_buildables_section(
        &self,
        ctx_data: &syn::Type,
        ctx_error: &syn::Type,
        data_type: &syn::Type,
    ) -> TokenStream {
        let ty = &self.ty;
        let count = self.count.unwrap();
        let buildables_var = util::string_to_ident(&format!("__buildables{}", count));
        let custom_ids_var = util::string_to_ident(&format!("__custom_ids{}", count));

        quote! {
            let #buildables_var = <#ty as ::minirustbot_forms::Subcomponent<#ctx_data, #ctx_error, #data_type>>::generate_buildables(context, data).await?;
            let mut #custom_ids_var = ::minirustbot_forms::util::id_vec_from_has_custom_ids(&#buildables_var).into_iter().map(::core::clone::Clone::clone).collect::<Vec<_>>();

            data.subcomponent_id_stack.push(::core::clone::Clone::clone(&#custom_ids_var));
            __custom_ids.append(&mut #custom_ids_var);
        }
    }

    /// Creates this subcomponent's row.
    fn create_row(&self) -> TokenStream {
        let count = self.count.unwrap();
        let buildables_var = util::string_to_ident(&format!("__buildables{}", count));
        if self.buttons.is_present() {
            quote! {
                .create_action_row(|b| {
                    let buildable = &#buildables_var.at(0);
                    ::minirustbot_forms::Buildable::<::poise::serenity_prelude::CreateActionRow>::on_build(buildable, b)
                })
            }
        } else {
            let (func, buildable_type) = if self.button.is_present() {
                (
                    quote! { create_button },
                    quote! { ::poise::serenity_prelude::CreateButton },
                )
            } else if self.select.is_present() {
                (
                    quote! { create_select_menu },
                    quote! { ::poise::serenity_prelude::CreateSelectMenu },
                )
            } else {
                panic!("Subcomponent is not button, buttons or select. {:?}", self)
            };

            quote! {
                .create_action_row(|mut f| {
                    for buildable in &#buildables_var {
                        f = f.#func(|b| ::minirustbot_forms::Buildable::<#buildable_type>::on_build(buildable, b));
                    }
                    f
                })
            }
        }
    }

    /// The necessary code to create this subcomponent from the received interaction.
    fn create_from_interaction(
        &self,
        ctx_data: &syn::Type,
        ctx_error: &syn::Type,
        data_type: &syn::Type,
        component_struct: &ComponentStruct,
    ) -> TokenStream {
        let ty = &self.ty;
        let return_expr = if self.is_self {
            let result = quote! {
                <Self as ::minirustbot_forms::Subcomponent<#ctx_data, #ctx_error, #data_type>>::build_from_interaction(context, interaction, data)
                        .await?
            };
            let result = component_struct.on_response_wrapper(result);
            quote! {  // return built self
                return ::core::result::Result::Ok(::core::option::Option::Some(#result));
            }
        } else {
            let result = quote! {
                (*<#ty as ::minirustbot_forms::Subcomponent<#ctx_data, #ctx_error, #data_type>>::build_from_interaction(context, interaction, data)
                        .await?).into()
            };
            let result = component_struct.on_response_wrapper(result);
            let initializer = component_struct.build_initializer(self.ident.as_ref(), Some(result));

            quote! {
                return ::core::result::Result::Ok(::core::option::Option::Some(::std::boxed::Box::new(#initializer)));
            }
        };

        // Pop this component's IDs from the ID stack
        // (Should be in order!)
        quote! {
            if data.subcomponent_id_stack.pop().ok_or(::minirustbot_forms::error::FormError::InvalidUserResponse)?.contains(&::minirustbot_forms::CustomId(interaction.data.custom_id.clone())) {
                #return_expr
            }
        }
    }

    /// Creates a subcomponent from a ComponentStruct, if applicable.
    fn from_component(component: &ComponentStruct) -> Option<Self> {
        (component.button.is_present()
            || component.buttons.is_present()
            || component.select.is_present())
        .then(|| Self {
            ident: Some(component.ident.clone()),
            ty: util::empty_tuple_type(),
            button: Default::default(),
            buttons: Default::default(),
            select: Default::default(),
            initializer: None,
            count: None,
            is_self: true,
        })
    }
}

impl ComponentStruct {
    /// Generate a "wait_for_response" function based on the given parameter for that.
    fn wait_for_response_func(
        &self,
        ctx_data: &syn::Type,
        ctx_error: &syn::Type,
        data_type: &syn::Type,
    ) -> Option<TokenStream> {
        self.wait_for_response.as_ref().map(|func| {
            quote! {
                async fn wait_for_response(
                    context: ::poise::ApplicationContext<'_, #ctx_data, #ctx_error>,
                    data: &mut ::minirustbot_forms::FormState<#data_type>,
                    custom_ids: &::std::vec::Vec<::minirustbot_forms::CustomId>,
                ) -> ::minirustbot_forms::error::ContextualResult<::core::option::Option<::std::sync::Arc<::poise::serenity_prelude::MessageComponentInteraction>>, #ctx_error> {
                    #func(context, data, custom_ids)
                }
            }
        })
    }

    /// Generate a "on_response" call based on the given path parameter.
    fn on_response_wrapper(&self, tokens: TokenStream) -> TokenStream {
        self.on_response
            .as_ref()
            .map(|func| {
                quote! {
                    #func(context, interaction, data, *(#tokens))
                }
            })
            .unwrap_or(tokens)
    }

    /// Builds a initializing statement for the Component struct.
    fn build_initializer(
        &self,
        replacing_ident: Option<&syn::Ident>,
        replacing_expr: Option<TokenStream>,
    ) -> TokenStream {
        let fields = self
            .data
            .as_ref()
            .take_struct()
            .unwrap()
            .fields
            .into_iter()
            .map(|f| {
                (
                    f.ident.clone().unwrap(),
                    f.initializer
                        .as_ref()
                        .map(|i| i.into_token_stream())
                        .unwrap_or(quote! { Default::default() }),
                )
            })
            .map(|(ident, init)| {
                if replacing_ident.map(|i| *i == ident).unwrap_or(false) {
                    (ident, replacing_expr.clone().unwrap())
                } else {
                    (ident, init)
                }
            })
            .map(|(ident, init)| quote! { #ident: #init, })
            .collect::<Vec<TokenStream>>();

        quote! {
            Self {
                #(#fields)*
            }
        }
    }
}
