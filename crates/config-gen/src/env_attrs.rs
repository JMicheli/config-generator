use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Expr, Field, Lit, LitStr, Meta, Type};

use crate::utils::{inner_type, is_parseable, is_type};

/// This unholy contraption matches #[env_key = "SOMETHING"]
/// on an incoming Field and generates an environment variable
/// loader for it.
pub fn derive_env_loader(field: &Field) -> TokenStream {
  for attr in &field.attrs {
    if attr.path().is_ident("env_key") {
      if let Meta::NameValue(meta) = &attr.meta {
        if let Expr::Lit(expr) = &meta.value {
          if let Lit::Str(lit_str) = &expr.lit {
            let field_ident = &field.ident;
            let field_ty = &field.ty;
            return gen_env_loader(field_ident, field_ty, lit_str);
          }
        }
      }
    }
  }

  // Return empty TokenStream when no env_key set
  quote! {}
}

fn gen_env_loader(ident: &Option<Ident>, ty: &Type, lit_str: &LitStr) -> TokenStream {
  if let Some(inner_ty) = inner_type("Vec", ty) {
    gen_env_loader_vec(ident, inner_ty, lit_str)
  } else if let Some(inner_ty) = inner_type("Option", ty) {
    gen_env_loader_option(ident, inner_ty, lit_str)
  } else {
    gen_env_loader_single(ident, ty, lit_str)
  }
}

fn gen_env_loader_single(ident: &Option<Ident>, ty: &Type, lit_str: &LitStr) -> TokenStream {
  // Handle "parseable" types like u32, f32, etc.
  if is_parseable(ty) {
    return quote! {
     if let ::core::result::Result::Ok(env_val) = ::std::env::var(#lit_str) {
       if let ::core::result::Result::Ok(parsed_val) = env_val.parse() {
         env_configs.#ident = ::core::option::Option::Some(parsed_val);
       }
     }
    };
  }

  // Handle String
  if is_type("String", ty) {
    quote! {
     if let ::core::result::Result::Ok(env_val) = ::std::env::var(#lit_str) {
       env_configs.#ident = ::core::option::Option::Some(env_val);
     }
    }
  } else {
    unimplemented!("Cannot parse type: {:?}", ty)
  }
}

fn gen_env_loader_vec(ident: &Option<Ident>, inner_ty: &Type, lit_str: &LitStr) -> TokenStream {
  // Handle "parseable" types like u32, f32, etc.
  if is_parseable(inner_ty) {
    return quote! {
    if let ::core::result::Result::Ok(env_val) = ::std::env::var(#lit_str) {
     if !env_val.is_empty() {
       env_configs.#ident = env_val
         .split(',')
         .map(|val| val.trim().to_string().parse().unwrap())
         .collect();
       }
     };
    };
  }

  // Handle String
  if is_type("String", inner_ty) {
    quote! {
      if let ::core::result::Result::Ok(env_val) = ::std::env::var(#lit_str) {
        if !env_val.is_empty() {
          env_configs.#ident = env_val
            .split(',')
            .map(|val| val.trim().to_string())
            .collect();
        }
      };
    }
  } else {
    unimplemented!("Cannot parse inner Vec type: {:?}", inner_ty)
  }
}

fn gen_env_loader_option(ident: &Option<Ident>, inner_ty: &Type, lit_str: &LitStr) -> TokenStream {
  gen_env_loader_single(ident, inner_ty, lit_str)
}
