extern crate proc_macro;

use pmutil::ToTokensExt;
use syn::{
    fold::{fold_block, Fold},
    parse::{self, Parse, ParseStream},
    Block, Expr, ExprBlock, ExprIf, ExprLet, ExprPath, ImplItem, Item, Macro, Pat, Stmt, Token,
};

/// Guard let for rust.
///
///
/// ```rust
/// use guard_let::guard_let;
///
/// enum Enum {
///    A(String),
///    B(usize),
///    C(Struct),
/// }
///
/// struct Struct {
///    foo: String,
/// }
///
/// fn eat_string(_: String) {}
///
/// #[guard_let]
/// fn simple_ident() {
///    let v = Enum::A(String::from(""));
///
///    guard_let!(v as Enum::A(s), {
///        // Type of v is Enum at here.
///        println!("v is not A: {:?}", v);
///        return;
///    });
///
///    // Type of s is String
///    eat_string(s)
/// }
///
/// #[guard_let]
/// fn pattern() {
///    let v = Enum::A(String::from(""));
///
///    guard_let!(v as Enum::C(Struct { foo }), {
///        // Type of v is Enum at here.
///        println!("v is not C: {:?}", v);
///        return;
///    });
///
///    // Type of s is String
///    eat_string(foo)
/// }
/// ```
#[proc_macro_attribute]
pub fn guard_let(
    _: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
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

struct Input {
    expr: ExprPath,
    _as_token: Token![as],
    pat: Pat,
    _comma_token: Token![,],
    block: Block,
}

impl Parse for Input {
    fn parse(i: ParseStream) -> parse::Result<Self> {
        Ok(Input {
            expr: i.parse()?,
            _as_token: i.parse()?,
            pat: i.parse()?,
            _comma_token: i.parse()?,
            block: i.parse()?,
        })
    }
}

struct Expander;

impl Fold for Expander {
    fn fold_block(&mut self, b: Block) -> Block {
        let b = fold_block(self, b);
        let mut stmts = vec![];

        let mut base = b.stmts.into_iter();
        loop {
            let stmt = match base.next() {
                Some(v) => v,
                None => break,
            };

            match stmt {
                Stmt::Semi(Expr::Macro(mac), ..) if mac.mac.path.is_ident("guard_let") => {
                    let stmt = Stmt::Semi(
                        expand_mac(&mut base, &mut stmts, mac.mac),
                        Default::default(),
                    );
                    stmts.push(stmt);
                }
                Stmt::Expr(Expr::Macro(mac)) if mac.mac.path.is_ident("guard_let") => {
                    let stmt = Stmt::Expr(expand_mac(&mut base, &mut stmts, mac.mac));
                    stmts.push(stmt);
                }

                _ => stmts.push(stmt),
            }
        }

        Block { stmts, ..b }
    }
}

fn expand_mac<I>(base: &mut I, _: &mut Vec<Stmt>, mac: Macro) -> Expr
where
    I: Clone + Iterator<Item = Stmt>,
{
    let input: Input = syn::parse2(mac.tokens).expect("failed to parse input: ");

    let let_expr = Expr::Let(ExprLet {
        attrs: Default::default(),
        let_token: Default::default(),
        pat: input.pat,
        eq_token: Default::default(),
        expr: Box::new(Expr::Path(input.expr)),
    });

    let else_expr = Expr::Block(ExprBlock {
        attrs: Default::default(),
        label: Default::default(),
        block: input.block,
    });

    let if_let_expr = Expr::If(ExprIf {
        attrs: Default::default(),
        if_token: Default::default(),
        cond: Box::new(let_expr),
        then_branch: Block {
            stmts: base.clone().collect(),
            brace_token: Default::default(),
        },
        else_branch: Some((Default::default(), Box::new(else_expr))),
    });
    while let Some(..) = base.next() {}

    if_let_expr
}
