use proc_macro::{TokenStream};
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, TypePath, LitStr};

#[proc_macro_derive(Packet, attributes(opcode))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = input.ident;
    let variants = match input.data {
        Data::Enum(variants) => variants.variants,
        _ => panic!("Only works with enum"),
    };

    // take every variants and put it in a vector of variants with their opcode values
    let mut packets = Vec::new();
    for variant in variants {
        let Some(opcode) = variant
            .attrs
            .into_iter()
            .find_map(|attr|
                attr.path.is_ident("opcode").then(|| Some(attr.parse_args::<syn::Expr>().ok()?)).flatten()
            ) else { panic!("Variant with no opcode") };

        packets.push((variant.ident, variant.fields, opcode));
    }

    let decode_payload = quote! {
        rmpv::decode::read_value_ref(&mut payload)
            .ok()
            .map(|v| match v {
                ValueRef::Map(map) => Some(map),
                _ => None,
            })
            .flatten()?
    };

    let get_str = quote! {
        {
            let ValueRef::String(val) = val else { None? };
            let Some(val) = val.into_str() else { None? };

            val.to_owned()
        }
    };

    // generate the `Packet` trait
    // its functions are:
    //
    //   fn decode_packet(opcode: u16, payload: &[u8]) -> Option<Self> // None on invalid payload / opcode
    //
    //   fn as_opcode(&self) -> u16
    //   fn encode_payload(self) -> Vec<u8>

    let decode_packet_match_arms = packets
        .clone()
        .into_iter()
        .map(|(variant_name, fields, opcode)| {
            // fn (opcode: u16, payload: &[u8])
            // from the opcode, given, we turn the payload to construct the variant

            let (initialization, variant_construction) = match fields {
                syn::Fields::Named(fields) => {
                    // Construct a variant that has named field
                    let mut names = Vec::new();

                    // construction arms
                    let fields_construction = 
                        fields.named
                            .into_iter()
                            .map(|field| {
                                let ident = field.ident.unwrap();

                                match field.ty {
                                    syn::Type::Path(TypePath { path, .. }) => {
                                        // todo: add primitive types like u32
                                        if !path.is_ident("String") { panic!("Unsupported type (todo: insert type)"); }
                                        let name = LitStr::new(ident.to_string().as_str(), ident.span());
                                        names.push(name.clone());
                                        
                                        // generate code for String
                                        quote!(#ident: map.remove(#name)?)
                                    },
                                    _ => panic!("unsupported type (todo: insert type)"),
                                }
                            }).collect::<Vec<proc_macro2::TokenStream>>();
                    
                    // let map_decode_match_arm = names
                    //     .into_iter()
                    //     .fold(proc_macro2::TokenStream::new(), |acc, name| {
                    //         quote! {
                    //             #acc
                    //             #name => acc.insert(#name, #get_str),
                    //         }
                    //     });
        
                    let initialization = quote! {
                        use std::collections::HashMap;
                        use rmpv::ValueRef;

                        let payload = #decode_payload;

                        let mut map =
                            payload
                                .into_iter()
                                .filter_map(|(key, val)| {
                                    let ValueRef::String(key) = key else { None? };
                                    key.into_str().map(|s| (s, val))
                                })
                                .try_fold(
                                    HashMap::new(),
                                    |mut acc, (key, val)| {

                                    match key {
                                        #(#names => acc.insert(#names, #get_str),)*
                                        _ => None,
                                    };

                                    Some(acc)
                                })?;
                    };

                    (initialization, quote! {
                        #enum_name::#variant_name {
                            #(#fields_construction),*
                        }
                    })
                }
                syn::Fields::Unnamed(fields) => {
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

                    let fields = fields
                        .unnamed
                        .into_iter()
                        .map(|field| {
                            let retrival = match field.ty {
                                syn::Type::Path(TypePath { path, .. }) => {
                                    // todo: add primitive types like u32
                                    if !path.is_ident("String") { panic!("Unsupported type (todo: insert type)"); }
                                    
                                    quote!(ValueRef::String(s) => s.into_string()?)
                                },
                                _ => panic!("unsupported type (todo: insert type)"),
                            };

                            quote! {{
                                match payload.remove(0) {
                                    #retrival,
                                    _ => return None
                                }
                            }}
                        });

                    (initialization, quote! {
                        #enum_name::#variant_name(#(#fields),*)
                    })
                },
                syn::Fields::Unit => {
                    // Construct a field that doesn't have any payload
                    (quote! {}, quote! { #enum_name::#variant_name })
                },
            };
            
            quote! {
                #opcode => {
                    #initialization

                    #variant_construction
                },
            }
        }).fold(proc_macro2::TokenStream::new(), |acc, ts| quote! { #acc #ts });

    let decode_packet = quote! {
        fn decode_packet(opcode: u16, mut payload: &[u8]) -> Option<Self> {
            Some(match opcode {
                #decode_packet_match_arms
                _ => return None,
            })
        }
    };

    let as_opcode_packets = packets
        .clone()
        .into_iter()
        .map(|(ident, fields, opcode)| {
            if fields.is_empty() {
                quote!(Self::#ident => #opcode)
            } else {
                match fields {
                    syn::Fields::Named(_) => quote!(Self::#ident { .. } => #opcode),
                    syn::Fields::Unnamed(_) => quote!(Self::#ident(..) => #opcode),
                    _ => unreachable!()
                }
            }
        });

    let as_opcode = quote! {
        fn as_opcode(&self) -> u16 {
            match self {
                #(#as_opcode_packets),*
            }
        }
    };

    let encode_payload_packets = packets
        .into_iter()
        .map(|(ident, fields, _opcode)| {
            if fields.is_empty() {
                quote!(#enum_name::#ident => vec![])
            } else {
                match fields {
                    syn::Fields::Named(named) => {
                        let names = named.named
                            .into_iter()
                            .map(|field| field.ident.unwrap())
                            .collect::<Vec<syn::Ident>>();

                        let names_lit = names
                            .clone().into_iter()
                            .map(|name| LitStr::new(&name.to_string(), name.span()));

                        quote! {
                            #enum_name::#ident { #(#names),* } => {
                                use rmpv::Value;
                                let mut res = Vec::new();
                                rmpv::encode::write_value(&mut res, &Value::Map(vec![
                                    #((#names_lit.into(), #names.into())),*
                                ])).ok()?;
                                res
                            }
                        }
                    },
                    syn::Fields::Unnamed(unnamed) => {
                        let len = unnamed.unnamed.len();
                        let names = (0..len).map(|num| {
                            let ident = syn::Ident::new(&format!("p{}", num), Span::call_site());
                            quote!(#ident)
                        });
                        let retrival = names.clone().map(|name| quote!(#name.into()));

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
                    },
                    _ => unreachable!(),
                }
            }
        });

    let encode_payload = quote! {
        fn encode_payload(self) -> Option<Vec<u8>> {
            Some(match self {
                #(#encode_payload_packets),*
            })
        }
    };

    // todo: an error type for this
    quote! {
        impl Packet for #enum_name {
            #decode_packet
            #as_opcode
            #encode_payload
        }
    }.into()
}