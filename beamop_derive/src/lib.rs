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
                quote_spanned! { variant.span() => #op::CODE => crate::DecodeOperands::decode_operands(reader).map(Self::#name), }
            });
            quote! {
                match reader.read_u8()? {
                    #(#arms)*
                    opcode => Err(crate::DecodeError::UnknownOpcode{ opcode })
                }
            }
        }
        _ => unimplemented!(),
    }
}

#[proc_macro_derive(DecodeOperands)]
pub fn derive_decode_operands_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let decode_operands = generate_decode_operands_fun_body(&input.data);
    let expanded = quote! {
        impl crate::DecodeOperands for #name {
            fn decode_operands<R>(reader: &mut R) -> Result<Self, crate::DecodeError>
            where R: std::io::Read {
                #decode_operands
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

fn generate_decode_operands_fun_body(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let decode_operands = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! { f.span() => #name: crate::CompactTerm::decode(reader)?.try_into()? }
                });
                quote! {
                    Ok(Self{
                        #(#decode_operands ,)*
                    })
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
