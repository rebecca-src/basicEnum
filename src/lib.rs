extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::str::FromStr;

use quote::{ToTokens, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, parse_macro_input};

#[proc_macro_derive(ParseEnum)]
pub fn derive_parse_enum(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let data = &input.data;
    let mut attr_type = quote! { usize };
    for attr in input.attrs.iter() {
        if attr.path().is_ident("repr") {
            let _ = attr.parse_nested_meta(|meta| {
                if let Some(me) = meta.path.span().source_text() {
                    if let Ok(att) = TokenStream2::from_str(&me) {
                        attr_type = att;
                    }
                };
                Ok(())
            });
        }
    }

    let mut variant_number_match;
    let mut variant_enum_match;

    match data {
        Data::Enum(data_enum) => {
            variant_number_match = TokenStream2::new();
            variant_enum_match = TokenStream2::new();
            let mut id = 0_usize;
            let mut df_var = TokenStream2::new();
            let mut df_var_enum = TokenStream2::new();
            for variant in &data_enum.variants {
                let variant_name = &variant.ident;
                let count_token = TokenStream2::from_str(&format!("{}_{}", id, attr_type))
                    .unwrap_or(quote! { #id });
                let self_id = match &variant.discriminant {
                    Some((_, expr)) => match expr {
                        syn::Expr::Lit(expr_lit) => {
                            let lit_str = format!("{}_{}", expr_lit.into_token_stream(), attr_type);
                            TokenStream2::from_str(&lit_str).unwrap_or(count_token)
                        }
                        _ => count_token,
                    },
                    None => count_token,
                };

                if id == 0 {
                    df_var = quote_spanned! {variant.span()=>
                        _ => Self::#variant_name,
                    };
                    df_var_enum = quote_spanned! {variant.span()=>
                        _ => #self_id,
                    };
                } else {
                    variant_number_match.extend(quote_spanned! { variant.span()=>
                        #self_id => Self::#variant_name,
                    });
                    variant_enum_match.extend(quote_spanned! { variant.span()=>
                        Self::#variant_name => #self_id,
                    });
                }
                id += 1;
            }
            variant_number_match.extend(df_var);
            variant_enum_match.extend(df_var_enum);
        }
        _ => {
            return Error::new(Span::call_site(), "ParseEnum is only implemented for enums")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        impl #name {
            pub fn from_number(v: #attr_type) -> Self {
                match v { #variant_number_match }
            }

            pub fn to_number(v: Self) -> #attr_type {
                match v { #variant_enum_match }
            }

            pub fn into(self) -> #attr_type {
                Self::to_number(self)
            }
        }

        impl Into<#name> for #attr_type {
            fn into(self) -> #name {
                #name::from_number(self)
            }
        }

        impl Into<#attr_type> for #name {
            fn into(self) -> #attr_type {
                Self::to_number(self)
            }
        }
    };

    TokenStream::from(expanded)
}
