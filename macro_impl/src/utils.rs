use syn::{punctuated::Punctuated, token::Comma, Data, DeriveInput, Field, Fields, Type};

/// An array of idents that will be deemed parseable by the is_parseable function.
const PARSEABLE_TYPES: [&str; 13] = [
  "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64", "usize",
];

/// Extract the simple inner type of an outer type from a field
/// e.g x: Option<String> => Some(String)
/// e.g x: String => None
pub fn inner_type<'a>(type_str: &str, ty: &'a Type) -> Option<&'a Type> {
  if let Type::Path(syn::TypePath {
    qself: None,
    ref path,
  }) = ty
  {
    if path.segments.len() != 1 || path.segments[0].ident != type_str {
      return None;
    }

    // inner_type would be e.g <String> in Option<String> or <K, V> in HashMap<K, V>
    if let syn::PathArguments::AngleBracketed(ref inner_type) = path.segments[0].arguments {
      // Extract only if a single type is present
      if inner_type.args.len() != 1 {
        return None;
      }

      // Extract the type from the <[..args]>
      if let syn::GenericArgument::Type(ref ty) = inner_type.args[0] {
        return Some(ty);
      }
    }
  }
  None
}

pub fn type_is_vec(ty: &Type) -> bool {
  inner_type("Vec", ty).is_some()
}

pub fn is_type(type_str: &str, ty: &Type) -> bool {
  if let Type::Path(syn::TypePath {
    qself: None,
    ref path,
  }) = ty
  {
    return path.is_ident(type_str);
  }
  false
}

pub fn is_parseable(ty: &Type) -> bool {
  for parseable_type in PARSEABLE_TYPES {
    if is_type(parseable_type, ty) {
      return true;
    }
  }

  false
}

pub fn get_fields(ast: &DeriveInput) -> Punctuated<Field, Comma> {
  match &ast.data {
    Data::Struct(s) => {
      if let Fields::Named(named_fields) = &s.fields {
        named_fields.named.clone()
      } else {
        unimplemented!("derive(ConfigGenerator) only supports named fields.")
      }
    },
    _ => unimplemented!("derive(ConfigGenerator) only supports Struct."),
  }
}
