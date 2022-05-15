use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

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
