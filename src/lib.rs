
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;

use syn::{parse_macro_input, ItemTrait, TraitItem, TraitItemMethod, FnArg, Type, Ident, GenericParam};


#[proc_macro_attribute]
pub fn closurize(_args: TokenStream, mut item: TokenStream) -> TokenStream {
    // TODO panic if any args are found

    let item2 = item.clone();
    let trait_definition = parse_macro_input!(item2 as ItemTrait);

    let self_methods: Vec<&TraitItemMethod> = trait_definition.items.iter().filter_map(|item| {
        match item {
            TraitItem::Method(method) => {
                method.sig.inputs.first().and_then(|arg| match arg {
                    FnArg::Receiver(_) => Some(method),
                    _ => None
                })
            },

            _ => None
        }
    }).collect();

    if self_methods.len() > 1 {
        panic!("A closure can only be derived for traits that have at most one method taking either `self`, `&self`, or `&mut self`.");
    }

    let method = self_methods.first()
        .expect("A closure can only be derived for traits that have a method taking either `self`, `&self`, or `&mut self`.");

    let fn_param_types: Vec<Type> = method.sig.inputs.iter().skip(1)
        .map(|item| match item {
            FnArg::Typed(arg) => arg.ty.as_ref().clone(),
            _ => panic!("Taking `self` is only allowed on the first argument of a function.")
        })
        .collect();

    let fn_param_names: Vec<Ident> = fn_param_types.iter().enumerate()
        .map(|(index, _ty)| quote::format_ident!("argument_{}", index))
        .collect();

    let self_parameters: Vec<_> = fn_param_names.iter().zip(&fn_param_types)
        .map(|(name, ty)| quote! { #name: #ty })
        .collect();

    let type_args = trait_definition.generics.params;
    let trait_generics: Vec<GenericParam> = type_args.into_iter().collect();
    let constraint = trait_definition.generics.where_clause;
    let trait_name = trait_definition.ident;

    let output: TokenStream = quote! {

        impl<F, #(#trait_generics),*>
            #trait_name < #(#trait_generics),* > for F
            where F: FnOnce( #(#fn_param_types),* ), #constraint
        {
            #[inline] fn consume(self, #(#self_parameters),* ) { self( #(#fn_param_names),* ) }
        }
    }.into();

    item.extend(output);
    item
}

