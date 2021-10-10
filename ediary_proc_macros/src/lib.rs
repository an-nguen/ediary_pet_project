extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn gen_repo_impl(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(item.clone());
    let user_struct = syn::parse_macro_input!(item as syn::ItemStruct);
    let name = user_struct.ident;
    let tokens = quote! {
        #input

        impl #name {
            fn get_conn(&self) -> r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>> {
                self.0.get().unwrap()
            }
        }
    };

    tokens.into()
}
