use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Opcode, attributes(opcode))]
pub fn derive_opcode_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    assert_eq!(input.attrs.len(), 1);
    let code = &input.attrs[0].tokens;
    let expanded = quote! {
        impl crate::Opcode for #name {
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
            fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::DecodeError> {
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
                quote_spanned! { variant.span() => #op::CODE => crate::Decode::decode(reader).map(Self::#name), }
            });
            quote! {
                use byteorder::ReadBytesExt as _;
                use std::io::Read as _;

                let opcode = reader.read_u8()?;
                let opcode_reader = [opcode];
                let reader = &mut opcode_reader.chain(reader);
                match opcode {
                    #(#arms)*
                    opcode => Err(crate::DecodeError::UnknownOpcode{ opcode })
                }
            }
        }
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let decode = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! { f.span() => #name: crate::Term::decode(reader)?.try_into()? }
                });
                quote! {
                    use byteorder::ReadBytesExt as _;

                    let opcode = reader.read_u8()?;
                    if opcode != Self::CODE {
                        return Err(crate::DecodeError::UnknownOpcode{ opcode });
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
