#[cfg(test)]
mod tests {
    use aurair::{
        decl::{fn_::FnBody, ValDecl},
        expr::{literal::Literal, Expr, StructExpr},
        module::{MainDecl, Mod, ModDecl},
        tyexpr::TypeExpr,
    };

    #[test]
    /// The AST for the following Aura code:
    /// 
    /// ```norun
    /// val msg String = "Hello, World!"
    /// main {
    ///     println(msg)
    /// }
    /// ```
    fn test_hello_world() {
        let main = Mod {
            decls: [
                ModDecl::Val(ValDecl {
                    symbol: "msg".into(),
                    ty: TypeExpr::Ident("String".into()),
                    value: Literal::String("Hello, World!".into()),
                }),
            ]
            .into(),
            main: MainDecl(FnBody::Expr(Expr::Call(
                Expr::Ident("println".into()).into(),
                StructExpr {
                    positional: vec![Expr::Ident("msg".into())],
                    named: vec![],
                },
            ))).into(),
        };

        dbg!(main);
    }
}
