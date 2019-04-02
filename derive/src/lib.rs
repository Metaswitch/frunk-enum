extern crate proc_macro;

use syn::parse_macro_input;
use quote::quote;

use syn::spanned::Spanned as _;

struct Field {
    pub ident: syn::Ident,
    pub ty: syn::Type,
}

fn simplify_fields<'a>(fields: &'a syn::Fields) -> Vec<Field> {
    use syn::Fields::*;
    match fields {
        Unit => Vec::new(),
        Named(named) => named.named.iter().map(|f| {
            Field {
                ident: f.ident.as_ref().unwrap().clone(),
                ty: f.ty.clone(),
            }
        }).collect(),
        Unnamed(unnamed) => unnamed.unnamed.iter().enumerate().map(|(i, f)| {
            Field {
                ident: syn::Ident::new(&format!("_{}", i), f.span()),
                ty: f.ty.clone(),
            }
        }).collect(),
    }
}

fn create_hlist_repr<'a>(mut fields: impl Iterator<Item = &'a Field>) -> proc_macro2::TokenStream {
    match fields.next() {
        None => quote!(HNil),
        Some(Field { ref ident, ref ty }) => {
            let tail = create_hlist_repr(fields);
            let ident = frunk_proc_macro_helpers::build_type_level_name_for(ident);
            quote!(HCons<Field<#ident, #ty>, #tail>)
        }
    }
}

fn create_repr_for0<'a>(mut variants: impl Iterator<Item = &'a syn::Variant>) -> proc_macro2::TokenStream {
    match variants.next() {
        None => quote!(Void),
        Some(v) => {
            let ident_ty = frunk_proc_macro_helpers::build_type_level_name_for(&v.ident);
            let fields = simplify_fields(&v.fields);
            let hlist = create_hlist_repr(fields.iter());
            let tail = create_repr_for0(variants);
            quote!{
                HEither<Variant<#ident_ty, #hlist>, #tail>
            }
        }
    }
}

fn create_into_case_body_for<'a>(ident: &syn::Ident, fields: impl Iterator<Item = &'a Field>, depth: usize) -> proc_macro2::TokenStream {
    let fields = fields.map(|f| {
        let ident = &f.ident;
        let ident_ty = frunk_proc_macro_helpers::build_type_level_name_for(ident);
        quote!(field!(#ident_ty, #ident))
    });
    let ident_ty = frunk_proc_macro_helpers::build_type_level_name_for(ident);
    let mut inner = quote!(HEither::Head(variant!(#ident_ty, hlist![#(#fields),*])));
    for _ in 0..depth {
        inner = quote!(HEither::Tail(#inner))
    }
    inner
}

fn create_into_cases_for<'a>(enum_ident: &'a syn::Ident, variants: impl Iterator<Item = &'a syn::Variant> + 'a) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    use syn::Fields::*;
    variants.enumerate().map(move |(idx, v)| {
        let variant_ident = &v.ident;
        let labelled_fields = simplify_fields(&v.fields);
        let pattern_vars = labelled_fields.iter().map(|f| &f.ident);
        let body = create_into_case_body_for(variant_ident, labelled_fields.iter(), idx);

        // Tediously patterns are rendered differently for the three styles so add appropriate wrapping
        // here.
        let pattern_vars = match v.fields {
            Unit => quote!(),
            Unnamed(_) => quote!((#(#pattern_vars),*)),
            Named(_) => quote!({#(#pattern_vars),*}),
        };

        quote!(#enum_ident::#variant_ident #pattern_vars => #body)
    })
}

fn create_into_for(input: &syn::ItemEnum) -> proc_macro2::TokenStream {
    let cases = create_into_cases_for(&input.ident, input.variants.iter());
    quote!{
        fn into(self) -> Self::Repr {
            match self {
                #(#cases),*
            }
        }
    }
}

fn generate_for_derive_input(input: syn::ItemEnum) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let generics = &input.generics;
    let repr = create_repr_for0(input.variants.iter());
    let into = create_into_for(&input);

    quote! {
        impl #generics LabelledGeneric for #ident #generics {
            type Repr = #repr;
            #into
        }
    }
}

#[proc_macro_derive(LabelledGenericEnum)]
pub fn derive_labelled_generic(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::ItemEnum);
    generate_for_derive_input(input).into()
}

#[test]
fn test_generate_for_enum() {
    let e = syn::parse_str::<syn::ItemEnum>(r#"
        enum Foo<C, E> {
            A,
            B(C, C, C),
            D { foo: E, bar: E },
        }
    "#).unwrap();

    println!("{}", generate_for_derive_input(e));
}
