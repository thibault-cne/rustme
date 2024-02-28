use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, FnArg, Type};

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, items: TokenStream) -> TokenStream {
    let cache_name = attr.to_string();
    let cache_ident = syn::Ident::new(&cache_name, proc_macro2::Span::call_site());
    let items = syn::parse_macro_input!(items as syn::ItemFn);

    let fn_name = items.sig.ident;
    let fn_vis = items.vis;
    let fn_block = items.block;
    let fn_params = items.sig.inputs;
    let fn_args = fn_arguments(fn_params.clone());
    let inner_fn_name = syn::Ident::new(
        &format!("__{}_inner", fn_name),
        proc_macro2::Span::call_site(),
    );

    quote! {
        #fn_vis async fn #fn_name(req: worker::Request, mut ctx: worker::RouteContext<crate::Caches>) -> Result<worker::Response> {
            let url: String = req.url().unwrap().clone().into();
            if let Some(resp) = ctx.data.#cache_ident().get(&url, false).await? {
                worker::console_log!("Cache hit");
                return Ok(resp);
            }

            async fn #inner_fn_name(#fn_params) -> Result<worker::Response> {
                #fn_block
            }

            worker::console_log!("Cache miss");
            let mut resp = #inner_fn_name(#fn_args).await?;
            resp.headers_mut()
                .set("Cache-Control", "public, max-age=3600")?;
            ctx.data.#cache_ident().put(&url, resp).await?;
            ctx.data
                .#cache_ident()
                .get(&url, false)
                .await
                .map(|r| r.unwrap())
        }
    }.into()
}

fn fn_arguments(fn_args: Punctuated<FnArg, Comma>) -> Punctuated<proc_macro2::TokenStream, Comma> {
    fn_args
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(pat_type) => {
                let ident = if match_type(&pat_type.ty, "Request") {
                    quote! { req }
                } else {
                    quote! { ctx }
                };
                match pat_type.ty.as_ref() {
                    Type::Reference(ty_ref) => {
                        let lifetime = ty_ref.lifetime.as_ref();
                        let mutability = ty_ref.mutability.as_ref();
                        quote! {
                            &#lifetime #mutability #ident
                        }
                    }
                    _ => quote! { #ident },
                }
            }
            _ => panic!("Expected a typed argument"),
        })
        .collect::<Punctuated<proc_macro2::TokenStream, Comma>>()
}

fn match_type(ty: &Type, name: &str) -> bool {
    match ty {
        Type::Reference(ty_ref) => match_type(ty_ref.elem.as_ref(), name),
        Type::Path(path) => {
            if let Some(last) = path.path.segments.last() {
                last.ident == name
            } else {
                false
            }
        }
        _ => false,
    }
}
