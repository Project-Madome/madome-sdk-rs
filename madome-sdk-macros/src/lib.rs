use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::{
    parse::{Parse, Parser},
    punctuated::Punctuated,
    token, AngleBracketedGenericArguments, Block, Expr, ExprCall, ExprPath, ExprTuple, FnArg,
    GenericArgument, Item, ItemFn, PatType, Path, PathArguments, PathSegment, ReturnType,
    Signature, Stmt, TraitBound, TraitBoundModifier, Type, TypeImplTrait, TypeParamBound,
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

fn is_impl_into(ty: &Type) -> bool {
    match ty {
        Type::ImplTrait(impl_trait) => impl_trait.bounds.iter().any(|x| {
            if let TypeParamBound::Trait(bounds) = x {
                bounds
                    .path
                    .segments
                    .iter()
                    .any(|x| x.ident.clone().into_token_stream().to_string() == "Into")
            } else {
                false
            }
        }),
        _ => false,
    }
}

fn impl_into_ty(ty: &Type) -> Type {
    /* let ty_st = ty.into_token_stream().to_string();

    println!("{ty:?}");
    println!("{ty_st}"); */

    // ty.clone()
    Type::ImplTrait(TypeImplTrait {
        impl_token: token::Impl::default(),
        bounds: Punctuated::from_iter([TypeParamBound::Trait(TraitBound {
            paren_token: None,                  // (...)
            modifier: TraitBoundModifier::None, // ? in ?Sized
            lifetimes: None,                    // 'a
            path: Path::from(PathSegment {
                ident: Ident::new("Into", Span::call_site()),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,             // ::
                    lt_token: token::Lt::default(), // <
                    gt_token: token::Gt::default(), // >
                    args: Punctuated::from_iter([GenericArgument::Type(ty.clone())]),
                }),
            }),
        })]),
    })
}

#[proc_macro_attribute]
pub fn impl_into_args(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::Item::parse.parse(item).unwrap();

    match item {
        Item::Fn(item_fn) => {
            // println!("{}\n", item_fn.clone().into_token_stream().to_string());

            let args = item_fn.sig.inputs.clone().into_iter().map(|x| match x {
                FnArg::Typed(ref pt) => {
                    // println!("{}", pt.ty.clone().into_token_stream().to_string());
                    // println!("{}", is_impl_into(&pt.ty));

                    let is_num = is_num_ty(&pt.ty);
                    let is_into = is_impl_into(&pt.ty);

                    if is_num || is_into {
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

fn is_unit_ty(ty: &Type) -> bool {
    let ty_st = ty.clone().into_token_stream().to_string();

    ty_st == "()"
}

fn unwrap_result(ty: &Type) -> Option<(Type, Type)> {
    match ty {
        Type::Path(ty_path) => {
            if let Some(path_seg) = ty_path.path.segments.clone().into_iter().next() {
                let ty_ident = path_seg.ident.into_token_stream().to_string();

                if ty_ident == "Result" {
                    let path_args = path_seg.arguments;

                    let (ok_ty, err_ty) = match path_args {
                        PathArguments::AngleBracketed(arg) => {
                            let mut args = arg.args.into_iter().filter_map(|x| match x {
                                GenericArgument::Type(ty) => Some(ty),
                                _ => None,
                            });

                            let ok_ty = args.next().unwrap();
                            let err_ty = args.next().unwrap();

                            (ok_ty, err_ty)
                        }
                        _ => return None,
                    };

                    Some((ok_ty, err_ty))
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn is_result_unit_ty(ty: &Type) -> bool {
    let (ok_ty, _err_ty) = unwrap_result(ty).unwrap();

    is_unit_ty(&ok_ty)
}

#[proc_macro_attribute]
pub fn ret_ty_or_unit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    /*

    fn abcd() -> () {
        ()
    }

    or

    fn abcd() -> u32 {
        do_something
    }

    */

    match Item::parse.parse(item).unwrap() {
        Item::Fn(item_fn) => {
            // println!("{}", item_fn.clone().into_token_stream().to_string());

            let ret = &item_fn.sig.output;

            let (is_unit, is_result_unit) = match ret {
                ReturnType::Default => (true, false),
                ReturnType::Type(_r, ty) => (is_unit_ty(&ty), is_result_unit_ty(&ty)),
            };

            // TODO: is_option_unit
            let block = match (is_unit, is_result_unit) {
                (true, _) => Box::new(Block {
                    stmts: vec![],
                    brace_token: token::Brace::default(),
                }),

                (_, true) => Box::new(Block {
                    stmts: vec![Stmt::Expr(Expr::Call(ExprCall {
                        attrs: Vec::new(),
                        func: Box::new(ok_expr_path()),
                        paren_token: token::Paren::default(),
                        args: Punctuated::from_iter([Expr::Tuple(ExprTuple {
                            attrs: Vec::new(),
                            paren_token: token::Paren::default(),
                            elems: Punctuated::default(),
                        })]),
                    }))],
                    brace_token: token::Brace::default(),
                }),
                _ => item_fn.block.clone(),
            };

            let item_fn = ItemFn { block, ..item_fn };

            // println!("{}", item_fn.clone().into_token_stream().to_string());

            item_fn.into_token_stream().into()
        }
        _ => panic!("no function"),
    }
}

fn ok_expr_path() -> Expr {
    Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: Path {
            leading_colon: None,
            segments: Punctuated::from_iter([PathSegment {
                ident: Ident::new("Ok", Span::call_site()),
                arguments: PathArguments::None,
            }]),
        },
    })
}
