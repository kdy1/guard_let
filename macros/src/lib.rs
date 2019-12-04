extern crate proc_macro;

use pmutil::ToTokensExt;
use syn::fold::{fold_block, Fold};
use syn::{Block, Expr, ImplItem, Item, Stmt};

/// Guard let for rust.
#[proc_macro_attribute]
pub fn guard(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tts = match syn::parse::<Item>(item.clone()) {
        Ok(item) => Expander.fold_item(item).dump(),
        _ => {
            let impl_item = syn::parse::<ImplItem>(item)
                .expect("input to guard_let should be Item or ImplItem");

            Expander.fold_impl_item(impl_item).dump()
        }
    };

    tts.into()
}

struct Expander;

impl Fold for Expander {
    fn fold_block(&mut self, b: Block) -> Block {
        let b = fold_block(self, b);
        let mut stmts = vec![];

        for stmt in b.stmts {
            match stmt {
                Stmt::Semi(Expr::Macro(mac), ..) | Stmt::Expr(Expr::Macro(mac))
                    if mac.mac.path.is_ident("guard_let") =>
                {
                    println!("Tokens: {}", mac.mac.tokens);
                }

                _ => stmts.push(stmt),
            }
        }

        Block { stmts, ..b }
    }
}
