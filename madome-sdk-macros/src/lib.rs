use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::{
    parse::{Parse, Parser},
    punctuated::Punctuated,
    token::{self},
    AngleBracketedGenericArguments, FnArg, GenericArgument, Item, ItemFn, PatType, Path,
    PathArguments, PathSegment, Signature, TraitBound, TraitBoundModifier, Type, TypeImplTrait,
    TypeParamBound,
};

fn is_num(ty_str: &str) -> bool {
    match ty_str {
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => true,
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => true,
        _ => false,
    }
}

fn is_num_ty(ty: &Type) -> bool {
    is_num(&ty.into_token_stream().to_string())

    // matches!(ty, Type::Path(path) if is_num(&path.clone().into_token_stream().to_string()))
}

fn impl_into_ty(ty: &Type) -> Type {
    /* let ty_st = ty.into_token_stream().to_string();

    println!("{ty:?}");
    println!("{ty_st}"); */

    // ty.clone()
    let a = Type::ImplTrait(TypeImplTrait {
        impl_token: token::Impl::default(),
        bounds: Punctuated::from_iter([TypeParamBound::Trait(TraitBound {
            paren_token: None,
            modifier: TraitBoundModifier::None, // ?
            lifetimes: None,                    // 'a
            path: Path::from(PathSegment {
                ident: Ident::new("Into", Span::call_site()),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: token::Lt::default(), // <
                    gt_token: token::Gt::default(), // >
                    args: Punctuated::from_iter([GenericArgument::Type(ty.clone())]),
                }),
            }),
        })]),
    });

    a
}

#[proc_macro_attribute]
pub fn impl_into_args(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::Item::parse.parse(item).unwrap();

    match item {
        Item::Fn(item_fn) => {
            // println!("{}\n", item_fn.clone().into_token_stream().to_string());

            let args = item_fn.sig.inputs.clone().into_iter().map(|x| match x {
                FnArg::Typed(ref pt) => {
                    let is_num = is_num_ty(&pt.ty);

                    if is_num {
                        x
                    } else {
                        let arg = FnArg::Typed(PatType {
                            ty: Box::new(impl_into_ty(&pt.ty)),
                            ..pt.clone()
                        });
                        arg
                    }
                }
                _ => x,
            });
            let item_fn = ItemFn {
                sig: Signature {
                    inputs: Punctuated::from_iter(args),
                    ..item_fn.sig
                },
                ..item_fn
            };

            // println!("{}\n\n", item_fn.clone().into_token_stream().to_string());

            item_fn.into_token_stream().into()
        }
        _ => panic!("no function"),
    }
}
