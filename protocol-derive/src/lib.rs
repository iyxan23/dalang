use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, TypePath, LitStr};

#[proc_macro_derive(Packet)]
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
            .iter()
            .find_map(|attr|
                attr.path.is_ident("opcode").then(|| {
                    let Ok(int) = syn::parse2::<syn::LitInt>(attr.tokens.clone())
                        else { panic!("Invalid opcode value"); };
                    int
                })) else {
                    panic!("Variant with no opcode");
                };

        packets.push((variant, opcode));
    }

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

    let match_arms = packets
        .into_iter()
        .map(|(variant, int)| {
            // fn (opcode: u16, payload: &[u8])
            // from the opcode, given, we turn the payload to construct the variant
            let name = variant.ident;

            let (map_decoding, variant_construction) = match variant.fields {
                syn::Fields::Named(fields) => {
                    let map_decoding = quote! {
                        use std::collections::Hashmap;
                        use rmpv::ValueRef;

                        // todo: decode_payload
                        let payload = decode_payload(&payload)?;

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

                                        $(
                                            $str_names => {
                                                acc.insert($str_names, get_str(val).ok_or(PayloadDecodeError::InvalidPayload)?);
                                            }
                                        )*
                                        _ => {},
                                    }

                                    Ok(acc)
                                })?;

                    };

                    // construction arms
                    let fields_construction = 
                        fields.named
                            .into_iter()
                            .map(|field| {
                                let ident = field.ident.unwrap();

                                match field.ty {
                                    syn::Type::Path(TypePath { path, .. }) => {
                                        // todo: add primitive types like u32
                                        if !path.is_ident("String") { panic!("Unsupported type {:?}", path); }
                                        let name = LitStr::new(ident.to_string().as_str(), ident.span());
                                        
                                        // generate code for String
                                        quote! {
                                            #path: map.remove(#name).ok()?.to_owned(),
                                        }
                                    },
                                    _ => panic!("unsupported type {:?}", field.ty),
                                }
                            })
                            .fold(proc_macro2::TokenStream::new(), |acc, ts| {
                                quote! { #acc
                                    #ts }
                            });

                    (map_decoding, quote! {
                        #enum_name::#name {
                            #fields_construction
                        }
                    })
                }
                syn::Fields::Unnamed(unnamed) => panic!(""),
                syn::Fields::Unit => panic!("")
            };
            
            quote! {
                #int => {
                    #map_decoding

                    #variant_construction
                },
            }
        }).fold(proc_macro2::TokenStream::new(), |acc, ts| quote! { #acc #ts });

    let decode_packet = quote! {
        // todo: Error
        fn decode_packet(opcode: u16, payload: &[u8]) -> Option<Self> {
            Some(match opcode {
                #match_arms
                _ => return None,
            })
        }
    };

    todo!()
}