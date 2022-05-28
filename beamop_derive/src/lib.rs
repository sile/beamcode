use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Opcode, attributes(opcode))]
pub fn derive_opcode_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let code = &input.attrs.last().expect("missing `#[opcode(N)]`").tokens;
    let expanded = quote! {
        impl crate::instruction::Opcode for #name {
            const CODE: u8 = #code;
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(Decode)]
pub fn derive_decode_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let decode = generate_decode_fun_body(&input.data);
    let expanded = quote! {
        impl crate::Decode for #name {
            fn decode_with_tag<R: std::io::Read>(reader: &mut R, tag: u8) -> Result<Self, crate::DecodeError> {
                #decode
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

fn generate_decode_fun_body(data: &Data) -> TokenStream {
    match *data {
        Data::Enum(ref data) => {
            let arms = data.variants.iter().map(|variant| {
                let name = &variant.ident;
                let op =
                    if let Fields::Unnamed(fields) = &variant.fields {
                        assert_eq!(fields.unnamed.len(), 1);
                        &fields.unnamed.iter().next().expect("unreachable").ty
                    } else {
                        unimplemented!()
                    };
                quote_spanned! { variant.span() => #op::CODE => crate::Decode::decode_with_tag(reader, tag).map(Self::#name), }
            });
            quote! {
                match tag {
                    #(#arms)*
                    opcode => Err(crate::DecodeError::UnknownOpcode{ opcode })
                }
            }
        }
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let decode = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! { f.span() => #name: crate::Decode::decode(reader)? }
                });
                quote! {
                    if tag != Self::CODE {
                        return Err(crate::DecodeError::UnknownOpcode{ opcode: tag });
                    }
                    Ok(Self{
                        #(#decode ,)*
                    })
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

#[proc_macro_derive(Encode)]
pub fn derive_encode_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let encode = generate_encode_fun_body(&input.data);
    let expanded = quote! {
        impl crate::Encode for #name {
            fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::EncodeError> {
                #encode
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

fn generate_encode_fun_body(data: &Data) -> TokenStream {
    match *data {
        Data::Enum(ref data) => {
            let arms = data.variants.iter().map(|variant| {
                let name = &variant.ident;
                if let Fields::Unnamed(fields) = &variant.fields {
                    assert_eq!(fields.unnamed.len(), 1);
                } else {
                    unimplemented!();
                }
                quote_spanned! { variant.span() => Self::#name(x) => x.encode(writer), }
            });
            quote! {
                match self {
                    #(#arms)*
                }
            }
        }
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let encode = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! { f.span() => self.#name.encode(writer)? }
                });
                quote! {
                    writer.write_all(&[Self::CODE])?;
                    #(#encode ;)*;
                    Ok(())
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
