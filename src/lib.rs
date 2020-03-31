
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
        panic!("a closure can only be derived for traits that have at most one method taking either `self`, `&self`, or `&mut self`.");
    }

    let method = self_methods.first()
        .expect("a closure can only be derived for traits that have a method taking either `self`, `&self`, or `&mut self`.");

    let fn_param_types: Vec<Type> = method.sig.inputs.iter().skip(1)
        .map(|item| match item {
            FnArg::Typed(arg) => arg.ty.as_ref().clone(),
            _ => panic!("taking `self` is only allowed on the first argument of a function.")
        })
        .collect();

    let fn_param_names: Vec<Ident> = fn_param_types.iter().enumerate()
        .map(|(index, _ty)| quote::format_ident!("argument_{}", index))
        .collect();

    let self_parameters: Vec<_> = fn_param_names.iter().zip(&fn_param_types)
        .map(|(name, ty)| quote! { #name: #ty })
        .collect();

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

    let constraint = trait_definition.generics.where_clause
        .map(|clause| clause.predicates);

    let type_args = trait_definition.generics.params;
    let trait_generics: Vec<GenericParam> = type_args.into_iter().collect();
    let trait_name = trait_definition.ident;
    let ret_expr = &method.sig.output;
    let fn_name = &method.sig.ident;
    let fn_where = &method.sig.generics.where_clause;
    let fn_type_parameters = &method.sig.generics.params;

    let closure_type = Ident::new(closure_type, Span::call_site());
    let trait_generics = quote! { #(#trait_generics),* };
    let fn_param_types = quote! { #(#fn_param_types),* };
    let fn_params = quote! { #(#self_parameters),* };
    let fn_args = quote! { #(#fn_param_names),* };
    let unsafety = &method.sig.unsafety;
    let asyncness = &method.sig.asyncness;

    let supertraits = trait_definition.supertraits;
    let supertraits = if supertraits.is_empty() { quote! {} } else {
        quote! { _FNTY___: #supertraits, }
    };

    // TODO fn generics? lifetimes??? fn where clause?
    // TODO associated return type

    // TEST trait `where`
    // TEST async

    if !fn_type_parameters.is_empty() {
        panic!("the type parameters on the trait function `{}` are currently not supported", method.sig.ident);
    }

    let output: TokenStream = quote! {

        impl<_FNTY___, #trait_generics>
            #trait_name < #trait_generics > for _FNTY___

            where
                _FNTY___: #closure_type ( #fn_param_types ) #ret_expr,
                #supertraits
                #constraint

        {
            #[inline]
            #unsafety #asyncness

            fn #fn_name // < #fn_type_parameters >

            (#receiver, #fn_params ) #ret_expr
                #fn_where
            {
                self( #fn_args )
            }
        }

    }.into();

    item.extend(output);
    item
}

