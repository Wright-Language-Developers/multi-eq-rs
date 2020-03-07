pub use quote::quote as multi_eq_quote;
pub extern crate proc_macro as multi_eq_proc_macro;
pub extern crate proc_macro2 as multi_eq_proc_macro2;
pub extern crate syn as multi_eq_syn;

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

#[macro_export]
macro_rules! multi_eq_make_derive {
    ($vis:vis, $trait_name:ident, $method_name:ident) => {
	#[proc_macro_derive($trait_name)]
	$vis fn $method_name(
	    input: multi_eq_proc_macro::TokenStream
	) -> multi_eq_proc_macro::TokenStream {
	    use multi_eq_quote as quote;
	    use multi_eq_syn as syn;
	    use multi_eq_proc_macro2 as proc_macro2;

	    use proc_macro2::TokenStream as TokenStream2;

	    let input = syn::parse::<syn::DeriveInput>(input).unwrap();
	    let input_ident = input.ident;
	    fn fields_eq<I: Iterator<Item = syn::Field>>(fields: I) -> TokenStream2 {
		fields.enumerate().fold(quote!(true), |acc, (i, item)| {
		    let name = match item.ident {
			Some(ident) => ident.to_string(),
			None => i.to_string(),
		    };
		    quote!(#acc && self.#name.$method_name(other.#name))
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
			    let cmp_expr = match fields {
				syn::Fields::Named(named) => fields_eq(named.named.iter().cloned()),
				syn::Fields::Unnamed(unnamed) => fields_eq(unnamed.unnamed.iter().cloned()),
				syn::Fields::Unit => quote!(true),
			    };
			    quote!((#input_ident::#ident, #input_ident::#ident) => #cmp_expr,)
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
