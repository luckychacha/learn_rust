extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // 被标注类型的名称
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify! 是一个内置的宏，接收一个表达式，编译的时候转换为字符串的字面值
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };

    gen.into()
}