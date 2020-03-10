pub extern crate proc_macro as multi_eq_proc_macro;
pub extern crate proc_macro2 as multi_eq_proc_macro2;
pub extern crate quote as multi_eq_quote;
pub extern crate syn as multi_eq_syn;

/// Macro to define a comparison trait
///
/// The format of the generated trait is the same as
/// [`PartialEq`](std::cmp::PartialEq), but with potentially different names.
///
/// ## Parameters:
///   * `vis` - optional visibility specifier
///   * `trait_name` - name of the trait being defined
///   * `method_name` - name of the method in the trait
///
/// ## Example:
/// ```rust
/// use multi_eq::*;
///
/// multi_eq_make_trait!(pub, PublicCustomEq, custom_eq);
/// multi_eq_make_trait!(PrivateCustomEq, eq);
/// ```
///
/// ## Generated code:
/// ```rust
/// pub trait PublicCustomEq {
///     fn custom_eq(&self, other: &Self) -> bool;
/// }
///
/// trait PrivateCustomEq {
///     fn eq(&self, other: &Self) -> bool;
/// }
/// ```
#[macro_export]
macro_rules! multi_eq_make_trait {
    ($vis:vis, $trait_name:ident, $method_name:ident) => {
        $vis trait $trait_name {
            fn $method_name(&self, other: &Self) -> bool;
        }
    };
    ($trait_name:ident, $method_name:ident) => {
        trait $trait_name {
            fn $method_name(&self, other: &Self) -> bool;
        }
    };
}

/// Macro to define a derive macro for a comparison trait
///
/// (Yes, this macro generates another macro that generates code) The format of
/// the derived trait is the same as [`PartialEq`](std::cmp::PartialEq), but
/// with potentially different names.
///
/// ## Note:
/// This macro can only be used in crates with the `proc-macro` crate type.
///
/// ## Parameters:
///   * `vis` - visibility specifier of the generated derive macro
///   * `trait_name` - name of the trait to derive
///   * `method_name` - name of the method in the trait, also used as the name
///                     of the proc macro
///
/// ## Example:
/// ```ignore
/// use multi_eq::*; // This global import is required for the macro to function
///
/// multi_eq_make_derive!(pub, CustomEq, custom_eq);
/// ```
#[macro_export]
macro_rules! multi_eq_make_derive {
    ($vis:vis, $trait_name:ident, $method_name:ident) => {
        #[proc_macro_derive($trait_name, attributes($method_name))]
        $vis fn $method_name(
            input: multi_eq_proc_macro::TokenStream
        ) -> multi_eq_proc_macro::TokenStream {
            use multi_eq_quote::quote;
            use multi_eq_quote::ToTokens;
            use multi_eq_quote::format_ident;
            use multi_eq_syn as syn;
            use multi_eq_proc_macro2::TokenStream as TokenStream2;

            let input = syn::parse::<syn::DeriveInput>(input).unwrap();
            let input_ident = input.ident;

            fn path_is(path: &syn::Path, s: &str) -> bool {
                let segs = &path.segments;
                segs.len() == 1 && {
                    let seg = &segs[0];
                    seg.arguments.is_empty() && seg.ident.to_string() == s
                }
            }

            fn lit_is_str(lit: &syn::Lit, s: &str) -> bool {
                match lit {
                    syn::Lit::Str(lit_str) => lit_str.value() == s,
                    _ => false,
                }
            }

            fn get_cmp_method_name(attr: &syn::Attribute) -> Option<String> {
                let method_name = stringify!($method_name);

                match attr.parse_meta() {
                    Ok(syn::Meta::List(meta_list)) if path_is(&meta_list.path, method_name) => {
                        meta_list.nested.iter().find_map(|nested_meta| match nested_meta {
                            syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                                path, lit: syn::Lit::Str(lit_str), ..
                            })) if path_is(path, "cmp") => Some(lit_str.value()),
                            _ => None,
                        })
                    }
                    _ => None,
                }
            }

            fn is_ignore(attr: &syn::Attribute) -> bool {
                let method_name = stringify!($method_name);

                match attr.parse_meta() {
                    Ok(syn::Meta::List(meta_list)) if path_is(&meta_list.path, method_name) => {
                        meta_list.nested.iter().any(|nested_meta| match nested_meta {
                            syn::NestedMeta::Meta(syn::Meta::Path(path)) => path_is(path, "ignore"),
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }

            fn fields_eq<I: Iterator<Item = syn::Field>>(fields: I) -> TokenStream2 {
                fields.enumerate().fold(quote!(true), |acc, (i, field)| {
                    let name = match field.ident {
                        Some(ident) => format_ident!("{}", ident),
                        None => format_ident!("v{}", i),
                    };
                    let method_name = match field.attrs.iter().find_map(get_cmp_method_name) {
                        Some(name) => format_ident!("{}", name),
                        None => format_ident!("{}", stringify!($method_name)),
                    };
                    if field.attrs.iter().any(is_ignore) {
                        acc
                    } else {
                        quote!(#acc && self.#name.#method_name(&other.#name))
                    }
                })
            };

            struct ArmAcc {
                pattern_left: TokenStream2,
                pattern_right: TokenStream2,
                body: TokenStream2,
            }

            fn gen_match_arm<I: Iterator<Item = syn::Field>>(fields: I) -> ArmAcc {
                fields.enumerate().fold(ArmAcc {
                    pattern_left: TokenStream2::new(),
                    pattern_right: TokenStream2::new(),
                    body: quote!(true),
                }, |ArmAcc { pattern_left, pattern_right, body }, (i, field)| {
                    let named = field.ident.is_some();
                    let (name_base) = match field.ident {
                        Some(ident) => format_ident!("{}", ident),
                        None => format_ident!("v{}", i),
                    };
                    let name_1 = format_ident!("{}_1", name_base);
                    let name_2 = format_ident!("{}_2", name_base);
                    let method_name = match field.attrs.iter().find_map(get_cmp_method_name) {
                        Some(name) => format_ident!("{}", name),
                        None => format_ident!("{}", stringify!($method_name)),
                    };
                    let cmp_expr = if field.attrs.iter().any(is_ignore) {
                        quote!(true)
                    } else {
                        quote!(#name_1.#method_name(#name_2))
                    };
                    ArmAcc {
                        pattern_left: match (named, i == 0) {
                            (true, true) => quote!(#name_base: #name_1),
                            (false, true) => quote!(#name_1),
                            (true, false) => quote!(#pattern_left, #name_base: #name_1),
                            (false, false) => quote!(#pattern_left, #name_1),
                        },
                        pattern_right: match (named, i == 0) {
                            (true, true) => quote!(#name_base: #name_2),
                            (false, true) => quote!(#name_2),
                            (true, false) => quote!(#pattern_right, #name_base: #name_2),
                            (false, false) => quote!(#pattern_right, #name_2),
                        },
                        body: quote!(#body && #cmp_expr),
                    }
                })
            };

            let expr = match input.data {
                syn::Data::Struct(syn::DataStruct {
                    fields: syn::Fields::Named(fields),
                    ..
                }) => fields_eq(fields.named.iter().cloned()),
                syn::Data::Struct(syn::DataStruct {
                    fields: syn::Fields::Unnamed(fields),
                    ..
                }) => fields_eq(fields.unnamed.iter().cloned()),
                syn::Data::Struct(syn::DataStruct {
                    fields: syn::Fields::Unit,
                    ..
                }) => quote!(true).into(),
                syn::Data::Enum(inner) => {
                    let arms = inner
                        .variants
                        .iter()
                        .map(|syn::Variant { ident, fields, .. }| {
                            match fields {
                                syn::Fields::Named(named) => {
                                    let ArmAcc {
                                        pattern_left,
                                        pattern_right,
                                        body
                                    } = gen_match_arm(named.named.iter().cloned());
                                    quote!((#input_ident::#ident { #pattern_left },
                                            #input_ident::#ident { #pattern_right }) => #body,)
                                }
                                syn::Fields::Unnamed(unnamed) => {
                                    let ArmAcc {
                                        pattern_left,
                                        pattern_right,
                                        body
                                    } = gen_match_arm(unnamed.unnamed.iter().cloned());
                                    quote!((#input_ident::#ident(#pattern_left),
                                            #input_ident::#ident(#pattern_right)) => #body,)
                                }
                                syn::Fields::Unit => quote!((#input_ident::#ident, #input_ident::#ident) => true,),
                            }
                        });
                    let arms = arms.fold(quote!(), |accum, arm| quote!(#accum #arm));
                    let arms = quote!(#arms (_, _) => false,);
                    let match_expr = quote!( match (self, other) { #arms } );
                    match_expr
                }
                syn::Data::Union(_) => panic!("unions are not supported"),
            };

            let ret = quote! {
                impl $trait_name for #input_ident {
                    fn $method_name(&self, other: &Self) -> bool {
                        #expr
                    }
                }
            };
            ret.into()
        }
    }
}
