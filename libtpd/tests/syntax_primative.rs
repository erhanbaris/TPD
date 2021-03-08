extern crate libtpd;

#[cfg(test)]
mod tests {
    use crate::libtpd::parser::*;
    use crate::libtpd::types::*;
    use crate::libtpd::syntax::*;
    use crate::libtpd::compiler::value::BramaPrimative;
    use crate::libtpd::compiler::ast::*;
    use std::rc::Rc;

    #[warn(unused_macros)]
    macro_rules! test_success {
        ($name:ident, $text:expr, $result:expr) => {
            #[test]
            fn $name () {
                let mut parser   = Parser::new($text);
                let parse_result = parser.parse();
                match parse_result {
                    Ok(_) => {
                        let syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
                        assert_eq!(syntax.parse(), $result);
                    },
                    Err((message, l, c)) => {
                        let _err: Result<BramaAstType, (&'static str, u32, u32)> = Err((message, l, c));
                        assert_eq!(_err, $result);
                    }
                };
            }
        };
    }


    test_success!(integer_1, "1024", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))));
    test_success!(integer_2, "1024000", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024000.0)))));
    test_success!(integer_3, "123", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0)))));
    test_success!(integer_4, "9223372036854775807", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(9223372036854775807.0)))));
    test_success!(integer_5, "0999999", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(999999.0)))));
    test_success!(integer_6, "1_234_567", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1234567.0)))));
    test_success!(integer_7, "1_234_5_6_7", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1234567.0)))));
    test_success!(integer_8, "1_234_5_6_7_", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1234567.0)))));

    test_success!(hex_1, "0x12", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(18.0)))));
    test_success!(hex_2, "0xffffff", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(16777215.0)))));
    test_success!(hex_3, "0x1FFFFFFFFFFFFF", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(9007199254740991.0)))));

    test_success!(oct_1, "062", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(50.0)))));
    test_success!(oct_2, "06211111111111", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(430723863113.0)))));

    test_success!(binary_1, "0b10000000000000000000000000000000", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2147483648.0)))));
    test_success!(binary_2, "0b01111111100000000000000000000000", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2139095040.0)))));
    test_success!(binary_3, "0b01", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0)))));
    test_success!(binary_4, "0B00000000011111111111111111111111", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(8388607.0)))));

    test_success!(test_1, "'merhaba dünya'", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Text(Rc::new("merhaba dünya".to_string()))))));
    test_success!(test_2, "\"merhaba dünya\"", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Text(Rc::new("merhaba dünya".to_string()))))));
    test_success!(test_3, "'merhaba dünya", Err(("Missing string deliminator", 0, 14)));
    test_success!(test_4, "\"merhaba dünya", Err(("Missing string deliminator", 0, 14)));
    test_success!(test_5, "merhaba dünya'", Err(("Missing string deliminator", 0, 14)));

    test_success!(bool_1, "true", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))));
    test_success!(bool_2, "doğru", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))));
    test_success!(bool_3, "false", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(false)))));
    test_success!(bool_4, "yanlış", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(false)))));

    test_success!(atom_1, ":merhaba", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Atom("merhaba".atom())))));
    test_success!(atom_2, ":dünya", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Atom("dünya".atom())))));
    test_success!(atom_3, ":_", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Atom("_".atom())))));
    test_success!(atom_4, ":__1__", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Atom("__1__".atom())))));

    test_success!(dict_1, "{}", Ok(BramaAstType::Dict([].to_vec())));
    test_success!(dict_2, "{'1':1}", Ok(BramaAstType::Dict([Box::new(BramaDictItem {
        key: Rc::new(BramaPrimative::Text(Rc::new("1".to_string()))),
        value: Rc::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0))))
    })].to_vec())));
    test_success!(dict_3, r#"{
        '1' : 1, 
        '2': 2
}"#, Ok(BramaAstType::Dict([Box::new(BramaDictItem {
        key: Rc::new(BramaPrimative::Text(Rc::new("1".to_string()))),
        value: Rc::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0))))
    }),
    Box::new(BramaDictItem {
        key: Rc::new(BramaPrimative::Text(Rc::new("2".to_string()))),
        value: Rc::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))
    })].to_vec())));
    test_success!(dict_4, r#"{
        '1': 1, 
        '2': 2,
        '1': 2
}"#, Ok(BramaAstType::Dict([Box::new(BramaDictItem {
        key: Rc::new(BramaPrimative::Text(Rc::new("1".to_string()))),
        value: Rc::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0))))
    }),
    Box::new(BramaDictItem {
        key: Rc::new(BramaPrimative::Text(Rc::new("2".to_string()))),
        value: Rc::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))
    }),
    Box::new(BramaDictItem {
        key: Rc::new(BramaPrimative::Text(Rc::new("1".to_string()))),
        value: Rc::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))
    })].to_vec())));
    

    test_success!(list_1, "[]", Ok(BramaAstType::List([].to_vec())));
    test_success!(list_2, "[1]", Ok(BramaAstType::List([Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0))))].to_vec())));
    test_success!(list_3, "[doğru]", Ok(BramaAstType::List([Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true))))].to_vec())));
    test_success!(list_4, "[ ]", Ok(BramaAstType::List([].to_vec())));
    test_success!(list_5, "[123,doğru,:erhan_barış,'merhaba dünya',1.3]", Ok(BramaAstType::List([Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0)))), Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))), Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Atom("erhan_barış".atom())))), Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Text(Rc::new("merhaba dünya".to_string()))))), Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.3))))].to_vec())));
    test_success!(list_6, "[[]]", Ok(BramaAstType::List([Box::new(BramaAstType::List([].to_vec()))].to_vec())));

    test_success!(list_7, "[123", Err(("Array not closed", 0, 0)));
    test_success!(list_8, "[data]", Ok(BramaAstType::List([Box::new(BramaAstType::Symbol("data".to_string()))].to_vec())));

    test_success!(empty_1, "yok", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Empty))));
    test_success!(empty_2, "empty", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Empty))));

    test_success!(symbol_1, "data", Ok(BramaAstType::Symbol("data".to_string())));
    test_success!(symbol_2, "data_test", Ok(BramaAstType::Symbol("data_test".to_string())));

    test_success!(parenthesis_1, "(10*10)", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
    }));
    test_success!(parenthesis_2, "(10+10)-10", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0)))), 
            operator: BramaOperatorType::Addition, 
            right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
        }), 
        operator: BramaOperatorType::Subtraction, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10.0))))
    }));
}