use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, parse_quote, DeriveInput, ItemStruct};

#[proc_macro_derive(HelpArgs)]
pub fn derive_cmd_line_helper(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    let struct_name = item.ident;

    let gen = quote! {
        // 構造体に対してtry_parse_from_iterを実装する。
        impl #struct_name {
            fn try_parse_from_iter<I, T>(cmd: I) -> CmdlineResult<Self> where I: IntoIterator<Item = T>, T: Into<OsString> + Clone, {
                if let Ok(cmd) = #struct_name::try_parse_from(cmd) {
                    if cmd.version {
                        return CmdlineResult::Msg(format!("{}", #struct_name::command().render_version()),CmdlineMsgHint::Version);
                    } else if cmd.help {
                        return CmdlineResult::Msg(format!("{}", #struct_name::command().render_help()),CmdlineMsgHint::Help);
                    }
                    CmdlineResult::Ok(cmd)
                } else {
                    CmdlineResult::Msg(format!("{}", #struct_name::command().render_help()),CmdlineMsgHint::PerseErrorHelp)
                }
            }
        }
    };
    gen.into()
}
/// 使い方: オプション引数（短縮オプション）の設定が可能です。
/// ヘルプとバージョン両方の指定 #\[helpargs_perser(help='i',version='v')]
/// ヘルプのみの指定 #\[ helpargs_perser(help='i')]
/// ロングオプションの指定(help) #\[ helpargs_perser(long_help='help123')]
/// ロングオプションの指定(version) #\[ helpargs_perser(long_version='vErsion123')]
/// 既定値 #\[helpargs_perser]
#[proc_macro_attribute]
pub fn helpargs_perser(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut tokens: HashMap<String, syn::ExprLit> = HashMap::new();
    tokens.insert("version".to_owned(), parse_quote!('V'));
    tokens.insert("help".to_owned(), parse_quote!('h'));
    tokens.insert("long_version".to_owned(), parse_quote!("version"));
    tokens.insert("long_help".to_owned(), parse_quote!("help"));
    let args = parse_macro_input!(args with syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated);
    for arg in args.iter() {
        match &arg.require_name_value().unwrap().value {
            syn::Expr::Lit(token) => {
                tokens.insert(
                    arg.require_name_value()
                        .unwrap()
                        .path
                        .segments
                        .first()
                        .unwrap()
                        .ident
                        .to_string(),
                    token.to_owned(),
                );
            }
            _ => {}
        }
    }
    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    // 構造体に対してtry_parse_from_iteratorに使用されるメンバ（version/help）を実装する。
                    let version = tokens.get("long_version").unwrap();
                    let help = tokens.get("long_help").unwrap();
                    let h = tokens.get("help").unwrap();
                    let v = tokens.get("version").unwrap();
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! { #[arg(short=#v,long=#version)] version:bool })
                            .unwrap(),
                    );
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! { #[arg(short=#h,long=#help)] help: bool})
                            .unwrap(),
                    );
                }
                _ => (),
            }

            return quote! {
                #ast
            }
            .into();
        }
        _ => panic!("`add_field` has to be used with structs "),
    }
}
