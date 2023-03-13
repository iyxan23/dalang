use proc_macro::{TokenStream};
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

    // let's imagine we have an enum error type
    // enum Error {
    //             
    // }

    // generate the `Packet` trait
    // its functions are:
    //
    //   fn decode_packet(opcode: u16, payload: &[u8]) -> Option<Self> // None on invalid payload / opcode
    //
    //   fn as_opcode(&self) -> u16
    //   fn encode_payload(self) -> Vec<u8>

    let decode_packet_match_arms = packets
        .into_iter()
        .map(|(variant_name, fields, opcode)| {
            // fn (opcode: u16, payload: &[u8])
            // from the opcode, given, we turn the payload to construct the variant

            let (map_decoding, variant_construction) = match fields {
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
                                        quote! {
                                            #path: map.remove(#name)?,
                                        }
                                    },
                                    _ => panic!("unsupported type (todo: insert type)"),
                                }
                            })
                            .fold(proc_macro2::TokenStream::new(), |acc, ts| {
                                quote! { #acc
                                    #ts }
                            });
                    
                    let map_decode_match_arm = names
                        .into_iter()
                        .fold(proc_macro2::TokenStream::new(), |acc, name| {
                            quote! {
                                #acc
                                #name => {
                                    acc.insert(#name, #get_str);
                                },
                            }
                        });
        
                    let map_decoding = quote! {
                        use std::collections::Hashmap;
                        use rmpv::ValueRef;

                        let payload = #decode_payload;

                        let mut map =
                            payload
                                .into_iter()
                                .filter_map(|(key, val)| {
                                    let ValueRef::String(key) = key else { None? };
                                    key.into_str().map(|s| (s, val))
                                })
                                .try_fold::<_, _, Result<_, PayloadDecodeError>>(
                                    HashMap::new(),
                                    |mut acc, (key, val)| {

                                    match key {
                                        #map_decode_match_arm
                                        _ => {},
                                    }

                                    Ok(acc)
                                })?;
                    };

                    (map_decoding, quote! {
                        #enum_name::#variant_name {
                            #fields_construction
                        }
                    })
                }
                syn::Fields::Unnamed(_unnamed) => todo!("implement unnamed fields / array"),
                syn::Fields::Unit => {
                    // Construct a field that doesn't have any payload
                    (quote! {}, quote! { #enum_name::#variant_name })
                },
            };
            
            quote! {
                #opcode => {
                    #map_decoding

                    #variant_construction
                },
            }
        }).fold(proc_macro2::TokenStream::new(), |acc, ts| quote! { #acc #ts });

    let decode_packet = quote! {
        fn decode_packet(opcode: u16, payload: &[u8]) -> Option<Self> {
            Some(match opcode {
                #decode_packet_match_arms
                _ => return None,
            })
        }
    };

    let as_opcode_packets = packets
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

    // todo: an error type for this
    quote! {
        impl Packet for #enum_name {
            #decode_packet
            #as_opcode
            fn encode_payload(self) -> Vec<u8> { todo!() }
        }
    }.into()
}