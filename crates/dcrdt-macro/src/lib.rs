use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn, DeriveInput};
use quote::{quote, format_ident};

#[proc_macro_attribute]
pub fn dcrdt_mutator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);
    let mutator_name = input.sig.ident.clone();
    let mutator_return = input.sig.output.clone();
    let fn_name = format!("__dcrdt_mutator_{}", &mutator_name);
    let fn_name_ident = format_ident!("{}", &fn_name);

    input.sig.ident = syn::Ident::new(
        &fn_name,
        input.sig.ident.span()
    );

    let output = quote! {
        pub fn #mutator_name(&mut self) #mutator_return {
            let delta = self.#fn_name_ident();
            let new_state = Self::join(&self.state, &delta);

            self.state = new_state;

            delta
        }        

        #input
    };

    output.into()
}

#[proc_macro_derive(ApplyDelta)]
pub fn dcrdt_apply_delta_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;
    let (_impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #generics DCRDTApplyState for #name #ty_generics #where_clause {
            fn apply(&mut self, delta: &S) -> S {
                let new_state = Self::join(self.state(), delta);
        
                self.state = new_state.clone();
        
                new_state
            }
        }
    };

    expanded.into()
}
