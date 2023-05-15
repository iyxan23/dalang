use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Packet, attributes(opcode, from_cloned))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let (enum_name, variants) = expect_enum(input);

    let packets = get_packets_from_variants(variants);

    // =>> generate the `Packet` trait impl <<=
    // its functions are:
    //
    //   fn decode_packet(opcode: u16, payload: &[u8]) -> Option<Self> // None on invalid payload / opcode
    //
    //   fn as_opcode(&self) -> u16
    //   fn encode_payload(self) -> Vec<u8>

    let decode_packet =
        decode_packet::generate_decode_packet_function(enum_name.clone(), packets.clone());

    let encode_payload =
        encode_payload::generate_encode_packet_function(enum_name.clone(), packets.clone());

    let as_opcode = generate_as_opcode_function(packets);

    // todo: an error type for this
    quote! {
        impl Packet for #enum_name {
            #decode_packet
            #as_opcode
            #encode_payload
        }
    }
    .into()
}

fn expect_enum(
    input: syn::DeriveInput,
) -> (
    syn::Ident,
    syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
) {
    let enum_name = input.ident;
    let variants = match input.data {
        syn::Data::Enum(variants) => variants.variants,
        _ => panic!("This macro could only derive from an enum"),
    };

    (enum_name, variants)
}

type Packets = Vec<(syn::Ident, syn::Fields, syn::Expr)>;

fn get_packets_from_variants(
    variants: syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
) -> Packets {
    // transform variants into Vec<(Ident, Fields, Expr)> which represents the variant identifier,
    // variant fields, and the opcode value given, respectively.
    variants.into_iter().map(|variant| {
        let Some(opcode) = variant
            .attrs
            .into_iter()
            .find_map(|attr|
                attr.path.is_ident("opcode").then(|| Some(attr.parse_args::<syn::Expr>().ok()?)).flatten()
            ) else {
                panic!("#[opcode] attribute is required for every enum variants")
            };

        (variant.ident, variant.fields, opcode)
    }).collect()
}

/// Contains functions to perform `decode_packet()` function generation
mod decode_packet {
    use quote::quote;

    pub(crate) fn generate_decode_packet_function(
        enum_name: syn::Ident,
        packets: super::Packets,
    ) -> proc_macro2::TokenStream {
        let decode_packet_match_arms = packets
            .clone()
            .into_iter()
            .map(|(variant_name, fields, opcode)| {
                generate_variant_decode(enum_name.clone(), variant_name, fields, opcode)
            })
            .fold(
                proc_macro2::TokenStream::new(),
                |acc, ts| quote! { #acc #ts },
            );

        quote! {
            fn decode_packet(opcode: u16, mut payload: &[u8]) -> Option<Self> {
                Some(match opcode {
                    #decode_packet_match_arms
                    _ => return None,
                })
            }
        }
    }

    fn generate_variant_decode(
        enum_name: syn::Ident,
        variant_name: syn::Ident,
        fields: syn::Fields,
        opcode: syn::Expr,
    ) -> proc_macro2::TokenStream {
        // from the opcode, given, we turn the payload to construct the variant
        let code = match fields {
            syn::Fields::Named(fields) => {
                generate_named_variant_decode(enum_name, variant_name, fields)
            }
            syn::Fields::Unnamed(fields) => {
                generate_unnamed_variant_decode(enum_name, variant_name, fields)
            }
            syn::Fields::Unit => {
                quote! { #enum_name::#variant_name }
            }
        };

        quote! {
            #opcode => {
                #code
            },
        }
    }

    fn generate_named_variant_decode(
        enum_name: syn::Ident,
        variant_name: syn::Ident,
        fields: syn::FieldsNamed,
    ) -> proc_macro2::TokenStream {
        let decode_payload = quote! {
            rmpv::decode::read_value_ref(&mut payload)
                .ok()
                .map(|v| match v {
                    ValueRef::Map(map) => Some(map),
                    _ => None,
                })
                .flatten()?
        };

        // Construct a variant that has named field
        let mut names = Vec::new();

        // construction arms
        let fields_construction = fields
            .named
            .into_iter()
            .map(|field| {
                let has_clone_attr = field
                    .attrs
                    .into_iter()
                    .find(|attr| attr.path.is_ident("from_cloned"))
                    .is_some();

                let ident = field.ident.unwrap();
                let typ = field.ty;

                let name = syn::LitStr::new(ident.to_string().as_str(), ident.span());
                names.push(name.clone());

                if !has_clone_attr {
                    quote!(#ident: <#typ as std::convert::TryFrom<rmpv::ValueRef>>::try_from(map.remove(#name)?).ok()?)
                } else {
                    quote!(#ident: <#typ as std::convert::TryFrom<rmpv::Value>>::try_from(map.remove(#name)?.to_owned()).ok()?)
                }
            })
            .collect::<Vec<proc_macro2::TokenStream>>();

        let initialization = quote! {
            use std::collections::HashMap;
            use rmpv::ValueRef;

            let payload = #decode_payload;

            let mut map = payload
                .into_iter()
                .filter_map(|(key, val)| {
                    let ValueRef::String(key) = key else { None? };
                    key.into_str().map(|s| (s, val))
                })
                .fold(HashMap::new(), |mut acc, (key, val)| {
                    match key {
                        #(#names => acc.insert(#names, val),)*
                        _ => None,
                    };

                    acc
                });
        };

        quote! {
            #initialization
            #enum_name::#variant_name {
                #(#fields_construction),*
            }
        }
    }

    fn generate_unnamed_variant_decode(
        enum_name: syn::Ident,
        variant_name: syn::Ident,
        fields: syn::FieldsUnnamed,
    ) -> proc_macro2::TokenStream {
        let initialization = quote! {
            use rmpv::ValueRef;

            let mut payload = rmpv::decode::read_value_ref(&mut payload)
                .ok()
                .map(|v| match v {
                    ValueRef::Array(arr) => Some(arr),
                    _ => None,
                })
                .flatten()?;
        };

        let fields = fields.unnamed.into_iter().enumerate().map(|(idx, field)| {
            let has_clone_attr = field
                .attrs
                .into_iter()
                .find(|attr| attr.path.is_ident("from_cloned"))
                .is_some();

            let typ = field.ty;

            if !has_clone_attr {
                quote!(<#typ as std::convert::TryFrom<rmpv::ValueRef>>::try_from(payload.remove(#idx)).ok()?)
            } else {
                quote!(<#typ as std::convert::TryFrom<rmpv::Value>>::try_from(payload.remove(#idx).to_owned()).ok()?)
            }
        });

        quote! {
            #initialization
            #enum_name::#variant_name(#(#fields),*)
        }
    }
}

mod encode_payload {
    use quote::quote;

    pub(crate) fn generate_encode_packet_function(
        enum_name: syn::Ident,
        packets: super::Packets,
    ) -> proc_macro2::TokenStream {
        let encode_payload_packets = packets.into_iter().map(|(ident, fields, _opcode)| {
            generate_variant_encode(enum_name.clone(), fields, ident)
        });

        quote! {
            fn encode_payload(self) -> Option<Vec<u8>> {
                Some(match self {
                    #(#encode_payload_packets),*
                })
            }
        }
    }

    fn generate_variant_encode(
        enum_name: syn::Ident,
        fields: syn::Fields,
        ident: syn::Ident,
    ) -> proc_macro2::TokenStream {
        if fields.is_empty() {
            quote!(#enum_name::#ident => vec![])
        } else {
            match fields {
                syn::Fields::Named(named) => {
                    let mut names = vec![];
                    let mut typs = vec![];

                    for field in named.named {
                        names.push(field.ident.unwrap());
                        typs.push(field.ty);
                    }

                    let names_lit = names
                        .clone()
                        .into_iter()
                        .map(|name| syn::LitStr::new(&name.to_string(), name.span()));

                    quote! {
                        #enum_name::#ident { #(#names),* } => {
                            use rmpv::Value;
                            let mut res = Vec::new();
                            rmpv::encode::write_value(&mut res, &Value::Map(vec![
                                #((#names_lit.into(), <rmpv::Value as From<#typs>>::from(#names))),*
                            ])).ok()?;
                            res
                        }
                    }
                }
                syn::Fields::Unnamed(unnamed) => {
                    let len = unnamed.unnamed.len();
                    let names = (0..len).map(|num| {
                        let ident =
                            syn::Ident::new(&format!("p{}", num), proc_macro2::Span::call_site());
                        quote!(#ident)
                    });

                    let retrival =
                        names
                            .clone()
                            .zip(unnamed.unnamed.into_iter())
                            .map(|(name, field)| {
                                let ty = field.ty;
                                quote!(<rmpv::Value as From<#ty>>::from(#name))
                            });

                    quote! {
                        #enum_name::#ident(#(#names),*) => {
                            use rmpv::Value;
                            let mut res = Vec::new();
                            rmpv::encode::write_value(&mut res, &Value::Array(vec![
                                #(#retrival),*
                            ])).ok()?;
                            res
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn generate_as_opcode_function(packets: Packets) -> proc_macro2::TokenStream {
    let variant_arms = packets
        .into_iter()
        .map(|(ident, fields, opcode)| generate_variant_as_opcode(fields, ident, opcode));

    quote! {
        fn as_opcode(&self) -> u16 {
            match self {
                #(#variant_arms),*
            }
        }
    }
}

fn generate_variant_as_opcode(
    fields: syn::Fields,
    ident: syn::Ident,
    opcode: syn::Expr,
) -> proc_macro2::TokenStream {
    if fields.is_empty() {
        quote!(Self::#ident => #opcode)
    } else {
        match fields {
            syn::Fields::Named(_) => quote!(Self::#ident { .. } => #opcode),
            syn::Fields::Unnamed(_) => quote!(Self::#ident(..) => #opcode),
            _ => unreachable!(),
        }
    }
}

// /// An utility extension for Iterator that could transform an
// /// Iterator<Item = (A, B)> and split it into (Iterator<Item=A>, Iterator<Item=B>)
// trait SplitExt<A, B>: Iterator {
//     fn split<S>(self) -> S
//     where
//         S: Split<A, B, Self::Item>,
//         Self: Sized,
//     {
//         S::split(self)
//     }
// }

// impl<A, B, I: Iterator<Item = (A, B)>> SplitExt<A, B> for I {}

// trait Split<A, B, Item = (A, B)> {
//     fn split<I>(iter: I) -> Self
//     where
//         I: Iterator<Item = Item>;
// }

// impl<A, B, AIter: Iterator<Item = A>, BIter: Iterator<Item = B>> Split<A, B> for (AIter, BIter) {
//     fn split<I>(iter: I) -> Self
//     where
//         I: Iterator<Item = (A, B)>,
//     {
//         let items = iter.collect::<Vec<(A, B)>>();

//         let mut a_items: Vec<A> = vec![];
//         let mut b_items: Vec<B> = vec![];

//         for (a, b) in items {
//             a_items.push(a);
//             b_items.push(b);
//         }

//         (a_items.into_iter(), b_items.into_iter())
//     }
// }
