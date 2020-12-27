extern crate tpd;

#[cfg(test)]
mod tests {
    use crate::tpd::types::*;
    use crate::tpd::parser::*;
    use crate::tpd::syntax::*;
    use crate::tpd::compiler::value::BramaPrimative;
    use crate::tpd::compiler::ast::{BramaAstType};
    use std::rc::Rc;

    #[warn(unused_macros)]
    macro_rules! test_compare {
        ($name:ident, $text:expr, $result:expr) => {
            #[test]
            fn $name () {
                let mut parser = Parser::new($text);
                match parser.parse() {
                    Err(_) => assert_eq!(true, false),
                    _ => ()
                };

                let syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
                assert_eq!(syntax.parse(), $result);
            }
        };
    }

    test_compare!(endless_1, r#"sonsuz:
    erhan=123
"#, Ok(BramaAstType::EndlessLoop(Box::new(BramaAstType::Assignment {
    variable: Rc::new("erhan".to_string()),
    operator: BramaOperatorType::Assign,
    expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
}))));
test_compare!(endless_2, r#"sonsuz:
    erhan=123   
    print(1)"#, Ok(BramaAstType::EndlessLoop(Box::new(BramaAstType::Block([BramaAstType::Assignment {
    variable: Rc::new("erhan".to_string()),
    operator: BramaOperatorType::Assign,
    expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
},
BramaAstType::FuncCall {
    names: ["print".to_string()].to_vec(),
    arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0))))].to_vec(),
    assign_to_temp: false
}
].to_vec())))));
test_compare!(endless_3, r#"sonsuz
    erhan=123   
    print(1)"#, Err(("':' missing", 0, 0)));
    test_compare!(endless_4, r#"sonsuz:
    erhan=123   
    print(1)
    kır"#, Ok(BramaAstType::EndlessLoop(Box::new(BramaAstType::Block([BramaAstType::Assignment {
    variable: Rc::new("erhan".to_string()),
    operator: BramaOperatorType::Assign,
    expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
},
BramaAstType::FuncCall {
    names: ["print".to_string()].to_vec(),
    arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0))))].to_vec(),
    assign_to_temp: false
},
BramaAstType::Break
].to_vec())))));
test_compare!(endless_5, r#"kır"#, Err(("break and continue belong to loops", 0, 0)));
test_compare!(endless_6, r#"devamet"#, Err(("break and continue belong to loops", 0, 0)));
}