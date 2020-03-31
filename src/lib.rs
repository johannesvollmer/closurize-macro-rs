
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;

use syn::*;
use syn::export::Span;


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
    let ret_expr = &method.sig.output;
    let fn_name = &method.sig.ident;

    let (receiver, closure_type) = match &method.sig.inputs[0] {
        FnArg::Receiver(receiver) => {
            (receiver.clone(), match &receiver.reference {
                Some(_) if receiver.mutability.is_some() => "FnMut",
                Some(_) => "Fn",
                None => "FnOnce",
            })
        },
        _ => unreachable!("first arg is not self")
    };

    let closure_type = Ident::new(closure_type, Span::call_site());
    let trait_generics = quote! { #(#trait_generics),* };
    let fn_param_types = quote! { #(#fn_param_types),* };
    let fn_params = quote! { #(#self_parameters),* };
    let fn_args = quote! { #(#fn_param_names),* };


    // TODO fn generics? lifetimes???
    // TODO fn where clause?
    // TODO Fn, FnMut
    // TODO unsafe, async
    // TODO trait types

    // TEST return type


    let output: TokenStream = quote! {

        impl<F, #trait_generics>
            #trait_name < #trait_generics > for F
            where
                F: #closure_type ( #fn_param_types ) #ret_expr,
                #constraint
        {
            #[inline] fn #fn_name (#receiver, #fn_params ) #ret_expr {
                self( #fn_args )
            }
        }
    }.into();

    item.extend(output);
    item
}

