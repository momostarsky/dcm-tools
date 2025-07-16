extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
// use regex::Regex;
use syn::{DeriveInput, LitInt, parenthesized, parse_macro_input, ItemFn, PatType, FnArg};

#[proc_macro_derive(DicomTagAccessors, attributes(dicom_tag))]
pub fn dicom_tag_accessors(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let mut getters = vec![];

    if let syn::Data::Struct(ref data) = input.data {
        for field in &data.fields {
            let field_ident = field.ident.as_ref().unwrap();
            for attr in &field.attrs {
                if attr.path().is_ident("dicom_tag") {
                    eprintln!("Processing field: {:?}", field_ident.to_string()); // Debug print
                    let mut group = None;
                    let mut element = None;

                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("group") {
                            let content;
                            parenthesized!(content in meta.input);
                            eprintln!("Parsing group: {:?}", content.to_string());
                            let lit: LitInt = content.parse()?;
                            let n: u16 = lit.base10_parse()?;
                            group = Some(n);
                            return Ok(());
                        }
                        if meta.path.is_ident("element") {
                            let content;
                            parenthesized!(content in meta.input);
                            eprintln!("Parsing element: {:?}", content.to_string());
                            let lit: LitInt = content.parse()?;
                            let n: u16 = lit.base10_parse()?;
                            element = Some(n);
                            return Ok(());
                        }

                        Err(meta.error("unrecognized repr"))
                    })
                    .unwrap();
                    if let (Some(group), Some(element)) = (group, element) {
                        let getter_name =
                            syn::Ident::new(&format!("{}_tag", field_ident), field_ident.span());
                        eprintln!(
                            "Generate Getter: {:?}->{:04X},{:04X}",
                            getter_name.to_string(),
                            group,
                            element
                        ); // Debug print
                        getters.push(quote! {
                            pub fn #getter_name() -> Tag {
                                Tag::from([#group, #element])
                            }
                        });
                    }
                }
            }
        }
    }

    let expanded = quote! {
        impl #name {
            #(#getters)*
        }
    };

    TokenStream::from(expanded)
}

// fn extract_tuple(s: &str) -> Option<(u16, u16)> {
//     eprintln!("extract_tuple field: {:?}",s); // Debug print
//     let re = Regex::new(r"\(([0-9A-Fa-f]+),([0-9A-Fa-f]+)\)").unwrap();
//     if let Some(caps) = re.captures(s) {
//         let g = u16::from_str_radix(&caps[1], 16).ok()?;
//         let e = u16::from_str_radix(&caps[2], 16).ok()?;
//         Some((g, e))
//     } else {
//         None
//     }
// }
// #[proc_macro_derive(TagMapAccessors, attributes(map_tag))]
// pub fn tag_map_accessors(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//
//     let name = &input.ident;
//
//     let mut getters = vec![];
//
//     if let syn::Data::Struct(ref data) = input.data {
//
//         for field in &data.fields {
//             let field_ident = field.ident.as_ref().unwrap();
//             for attr in &field.attrs {
//                 if attr.path().is_ident("map_tag") {
//                     eprintln!("Processing field: {:?}", field_ident.to_string()); // Debug print
//                     let mut tag_name = None;
//
//
//                     attr.parse_nested_meta(|meta| {
//                         if meta.path.is_ident("tag_name") {
//                             let content;
//                             parenthesized!(content in meta.input);
//                             eprintln!("Parsing group: {:?}", content.to_string());
//                             let lit: LitInt = content.parse()?;
//                             tag_name = Some(lit);
//                             return Ok(());
//                         }
//
//                         Err(meta.error("unrecognized repr"))
//                     })
//                         .unwrap();
//
//
//                         let getter_name =
//                             syn::Ident::new(&format!("{}_tag", field_ident), field_ident.span());
//                         eprintln!(
//                             "Generate Getter: {:?}->{:?}",
//                             getter_name.to_string(),
//                             tag_name,
//
//                         ); // Debug print
//                         getters.push(quote! {
//                             pub fn #getter_name() -> Tag {
//                                return tag_name;
//                             }
//                         });
//
//                 }
//             }
//         }
//     }
//
//     let expanded = quote! {
//         impl #name {
//             #(#getters)*
//         }
//     };
//
//     TokenStream::from(expanded)
// }
#[proc_macro_derive(TagMapAccessors, attributes(map_tag))]
pub fn tag_map_accessors(input: TokenStream) -> TokenStream {
    // 解析字段上的 #[map_tag()] 形式的参数
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let mut getters = vec![];
    eprintln!("Parsing root name: {:?}", name.to_string());
    
    if let syn::Data::Struct(ref data) = input.data {
        for field in &data.fields {
            //获取属性名称 eg: bit_allocated bits_stored  high_bit
            let field_ident = field.ident.as_ref().unwrap();
            eprintln!("field_ident: {:?}", field_ident.to_string()); 
            for attr in &field.attrs {
                if attr.path().is_ident("map_tag") {
                    let mut group = None;
                    let mut element = None;
                   
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("tag_name") {
                            let content;
                            parenthesized!(content in meta.input);
                            eprintln!("Parsing tag_name: {:?}", content.to_string());
                            let group_lit: LitInt = content.parse()?;
                            //解析 逗号。
                            let _: syn::Token![,] = content.parse()?; // parse the comma
                            let element_lit: LitInt = content.parse()?;
                            eprintln!("Parsing group: {:?}", group_lit.to_string());
                            eprintln!("Parsing element: {:?}", element_lit.to_string());
                            group = Some(group_lit.base10_parse::<u16>()?);
                            element = Some(element_lit.base10_parse::<u16>()?);
                            return Ok(());
                        }
                        Err(meta.error("unrecognized repr"))
                    }).unwrap();

                    if let (Some(group), Some(element)) = (group, element) {
                        let getter_name =
                            syn::Ident::new(&format!("{}_tag", field_ident), field_ident.span());
                        getters.push(quote! {
                            pub fn #getter_name() -> Tag {
                                Tag::from([#group, #element])
                            }
                        });
                    }
                }
            }
        }
    }  else {
        // Handle other data types, e.g., enums or unions
        eprintln!("Only structs are supported by this macro.");
    }

    let expanded = quote! {
        impl #name {
            #(#getters)*
        }
    };

    TokenStream::from(expanded)
}


#[proc_macro_derive(DicomTagMapAccessors, attributes(map_tag_name))]
pub fn dicom_tag_map_accessors(input: TokenStream) -> TokenStream {
    // 解析字段上的 #[map_tag()] 形式的参数
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let mut getters = vec![];
    eprintln!("Parsing root name: {:?}", name.to_string());

    if let syn::Data::Struct(ref data) = input.data {
        for field in &data.fields {
            //获取属性名称 eg: bit_allocated bits_stored  high_bit
            let field_ident = field.ident.as_ref().unwrap();
            eprintln!("field_ident: {:?}", field_ident.to_string());
            for attr in &field.attrs {
                if attr.path().is_ident("map_tag_name") {
                    let mut dicom_ctag: Option<syn::ExprPath> = None;


                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("tag_name") {
                            let content;
                            parenthesized!(content in meta.input);
                            eprintln!("Parsing dicom_tag_key: {:?}", content.to_string());

                            let tag_expr: syn::ExprPath = content.parse()?;
                            dicom_ctag = Some(tag_expr);
                            return Ok(());
                        }
                        Err(meta.error("unrecognized repr"))
                    }).unwrap();
                    if let Some(dicom_ctag) = dicom_ctag {
                        let const_name = syn::Ident::new(
                            &format!("{}_TAG", field_ident.to_string().to_uppercase()),
                            field_ident.span(),
                        );
                        eprintln!("Parsing dicom_tag_key: {:?}", const_name.to_string());
                        getters.push(quote::quote! {
                            pub const #const_name: Tag = #dicom_ctag;
                        });
                    }
                    // if let Some(dicom_ctag) = dicom_ctag {
                    //     let getter_name = syn::Ident::new(&format!("{}_tag", field_ident), field_ident.span());
                    //     getters.push(quote::quote! {
                    //         pub fn #getter_name() -> Tag {
                    //             #dicom_ctag
                    //         }
                    //     });
                    // }
                }
            }
        }
    }  else {
        // Handle other data types, e.g., enums or unions
        eprintln!("Only structs are supported by this macro.");
    }

    let expanded = quote! {
        impl #name {
            #(#getters)*
        }
    };

    TokenStream::from(expanded)
}



/// 自定义属性宏，在函数执行前后打印日志及参数



use syn::Ident;
#[proc_macro_attribute]
pub fn log_execution(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let signature = &input_fn.sig;
    let fn_name = &signature.ident;
    let vis = &input_fn.vis;
    let attrs = &input_fn.attrs;
    let block = &input_fn.block;

    // 参数列表（带类型）
    let args: Vec<_> = signature.inputs.iter().map(|arg| match arg {
        FnArg::Typed(pat_ty) => quote! { #pat_ty },
        FnArg::Receiver(recv) => quote! { #recv },
    }).collect();

    // 仅参数名
    let arg_names: Vec<_> = signature.inputs.iter().map(|arg| match arg {
        FnArg::Typed(PatType { pat, .. }) => quote! { #pat },
        FnArg::Receiver(_) => quote! { self },
    }).collect();

    // 参数名字符串
    let arg_name_strs: Vec<_> = signature.inputs.iter().map(|arg| match arg {
        FnArg::Typed(PatType { pat, .. }) => quote! { stringify!(#pat) },
        FnArg::Receiver(_) => quote! { "self" },
    }).collect();

    let inner_fn_name = format_ident!("__{}_impl", fn_name);

    let output = &signature.output;
    let asyncness = &signature.asyncness;

    let expanded = quote! {
        // 内部原始函数
        #(#attrs)*
        #[inline(always)]
        #vis #asyncness fn #inner_fn_name(#(#args),*) #output
        #block

        // 包装日志函数
        #vis #asyncness fn #fn_name(#(#args),*) #output {
            let __params = vec![#(
                format!("{} = {:?}", #arg_name_strs, #arg_names)
            ),*].join(", ");
            println!("Starting execution of function: {} with args: {}", stringify!(#fn_name), __params);

            let __result = #inner_fn_name(#(#arg_names),*);

            println!("Finished execution of function: {}", stringify!(#fn_name));
            __result
        }
    };

    TokenStream::from(expanded)
}