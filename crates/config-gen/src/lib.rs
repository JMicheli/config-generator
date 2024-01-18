mod env_attrs;
mod utils;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, DeriveInput, Field};

use crate::{
  env_attrs::derive_env_loader,
  utils::{get_fields, inner_type, type_is_vec},
};

#[proc_macro_derive(ConfigGenerator, attributes(env_key))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);

  let fields = derive_optional_fields(&ast);

  let new_struct_ident = derive_optional_struct_name(&ast);
  let old_struct_impls = derive_old_struct_impls(&ast);
  let new_struct_impls = derive_new_struct_impls(&ast);

  let new_struct = quote! {
    #old_struct_impls

    #[derive(::serde::Deserialize)]
    struct #new_struct_ident {
      #fields
    }

    #new_struct_impls
  };

  new_struct.into()
}

fn derive_optional_struct_name(ast: &DeriveInput) -> Ident {
  let original_struct_name = ast.ident.to_string();
  format_ident!("Optional{}", original_struct_name)
}

fn derive_optional_fields(ast: &DeriveInput) -> TokenStream {
  let fields = get_fields(ast);

  let optional_fields = fields.iter().map(|field| {
    let name = &field.ident;
    let ty = &field.ty;

    if inner_type("Option", ty).is_some() {
      quote! { #name: ::std::option::#ty }
    } else if type_is_vec(ty) {
      quote! { #name: ::std::vec::#ty }
    } else {
      quote! { #name: ::std::option::Option<#ty> }
    }
  });

  // Output interior portion of new struct definition
  quote! {
    #(#optional_fields),*
  }
}

fn derive_old_struct_impls(ast: &DeriveInput) -> TokenStream {
  let old_struct_ident = &ast.ident;
  let new_struct_ident = derive_optional_struct_name(ast);

  // Generate the loaders used in the from_env function
  // to load env values into the
  let env_loaders: Vec<TokenStream> = get_fields(ast)
    .iter()
    .map(|field| derive_env_loader(field))
    .collect();

  // Write new struct implementations for the input struct
  // These will define the from_toml and from_env functions
  // for the struct, which are the main functionality exposed
  // by this crate.
  quote! {
    impl #old_struct_ident {
      pub fn with_toml<P: ::core::convert::AsRef<::std::path::Path>>(mut self, path: &P) -> Self {
        let file_contents = ::std::fs::read_to_string(path).unwrap();
        let optional_config = ::toml::from_str::<#new_struct_ident>(file_contents.as_str()).unwrap();

        // Apply loaded optional_config and return
        optional_config.apply_to(self)
      }

      pub fn with_env(mut self) -> Self {
        let mut env_configs = #new_struct_ident::new();

        #(#env_loaders)*

        // Apply optional_config from environment and return
        env_configs.apply_to(self)
      }
    }
  }
}

fn derive_new_struct_impls(ast: &DeriveInput) -> TokenStream {
  let old_struct_ident = &ast.ident;
  let new_struct_ident = derive_optional_struct_name(ast);
  let fields = get_fields(ast);

  let new_settings = fields.iter().map(|field| {
    let name = &field.ident;
    let ty = &field.ty;

    if type_is_vec(ty) {
      quote! { #name: ::std::vec::Vec::new() }
    } else {
      quote! { #name: ::std::option::Option::None }
    }
  });

  let apply_to_fn = gen_apply_to_fn(old_struct_ident, &fields);

  quote! {
    impl #new_struct_ident {
      pub fn new() -> Self {
        Self {
          #(#new_settings),*
        }
      }

      #apply_to_fn
    }
  }
}

fn gen_apply_to_fn(old_struct_ident: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream {
  // Build up a vector of field appliction tokenstreams
  let field_applyors: Vec<TokenStream> = fields
    .iter()
    .map(|field| {
      let name = &field.ident;
      let ty = &field.ty;
      if inner_type("Option", ty).is_some() {
        quote! {
          if self.#name.is_some() {
            old.#name = self.#name.clone();
          }
        }
      } else if type_is_vec(ty) {
        quote! {
          for item in &self.#name {
            if !old.#name.contains(item) {
              old.#name.push(item.clone());
            }
          }
        }
      } else {
        quote! {
          if let ::std::option::Option::Some(val) = self.#name {
            old.#name = val;
          }
        }
      }
    })
    .collect();

  quote! {
    pub fn apply_to(&self, mut old: #old_struct_ident) -> #old_struct_ident {
      #(#field_applyors)*

      old
    }
  }
}
