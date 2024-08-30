use proc_macro::TokenStream;
mod raw_builder;

// 使用者可以通过query!(...)来调用
#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() { println!(\"Hello world!\"); }"
        .parse()
        .unwrap()
}

#[proc_macro_derive(RawBuilder)]
pub fn derive_raw_builder(input: TokenStream) -> TokenStream {
    use crate::raw_builder::BuilderContext;
    BuilderContext::render(input).unwrap().parse().unwrap()
}
