extern crate proc_macro; // in 2018 edition, proc-macro still needs use extern crate
use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{ parse_macro_input,Pat,Type,  PatType, DeriveInput, NestedMeta, Meta, Data, Fields, ItemFn, FnArg, AttributeArgs, DataStruct };
use quote::{ quote, format_ident};
use proc_macro2::{Ident, Span};
use inflector::Inflector;
use std::env;
use regex::Regex;

// attribute macro
#[proc_macro_attribute]
pub fn async_init(_: TokenStream, func: TokenStream) -> TokenStream {
    let func = parse_macro_input!(func as ItemFn); // 我们传入的是一个函数，所以要用到ItemFn

    // println!("func {:?}", func);
    let func_vis = &func.vis; // pub
    let func_block = &func.block; // 函数主体实现部分{}

    let func_decl = &func.sig; // 函数申明
    let func_name = &func_decl.ident; // 函数名
    let func_generics = &func_decl.generics; // 函数泛型
    let func_inputs = &func_decl.inputs; // 函数输入参数
    let func_output = &func_decl.output; // 函数返回

    // 提取参数，参数可能是多个
    let params: Vec<_> = func_inputs.iter().map(|i| {
        match i {
            // 提取形参的pattern
            // https://docs.rs/syn/1.0.1/syn/struct.PatType.html
            FnArg::Typed(ref val) => {
                &val.pat
            },
            _ => unreachable!("it's not gonna happen."),
        }
    }).collect();

    let s: String = quote!{ #func_output }.to_string();
    let re = Regex::new(r"Result\s*<\s*([\w:\s]+),").unwrap();
    let error_message = format!("Can not extra result type from {}", s);
    let caps = re.captures(&s).expect(&error_message);
    // println!("func_output_func_output is {}", &caps[1]);

    let resultType = Ident::new(&caps[1], func_name.span());

    let new_name = format!("{}_VALUE", func_name.to_string().to_uppercase());
    let value_name = Ident::new(&new_name, func_name.span());

    let fn_new_name = format!("{}_orignal_fn", func_name.to_string());
    let fn_new_name = Ident::new(&fn_new_name, func_name.span());

    let expanded = quote! {
        thread_local!(static #value_name: std::cell::RefCell<Option<#resultType>> = std::cell::RefCell::new(None));
        #func_vis async fn #fn_new_name #func_generics(#func_inputs) #func_output #func_block

        #func_vis async fn #func_name #func_generics(#func_inputs) #func_output {
            let result = match #value_name.with(|f| {
                (*f.borrow()).clone()
            }) {
                Some(result) => result,
                None => { 
                    let value = #fn_new_name(#(#params,) *).await?;
                    #value_name.with(|f| {
                        *f.borrow_mut() = Some(value.clone());
                        value
                    })              
                }
            };
            Ok(result)
        }
    };

    // println!("func is {:?}", expanded.to_string());
    expanded.into()
}
