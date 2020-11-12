use crate::types::*;
use crate::syntax::SyntaxParser;
use crate::syntax::primative::PrimativeParser;

pub struct UnaryParser;

// https://github.com/rust-lang/rust/issues/75429
type ParseType = fn(parser: &SyntaxParser) -> AstResult;

fn map_parser(parser: &SyntaxParser, parser_funcs: &[ParseType]) -> AstResult {
    for parser_func in parser_funcs {
        match parser_func(parser) {
            Ok(BramaAstType::None) => (),
            Ok(ast) => return Ok(ast),
            Err(err) => return Err(err)
        }
    }

    Ok(BramaAstType::None)
}

impl SyntaxParserTrait for UnaryParser {
    type Item = BramaAstType;
    type In = SyntaxParser;

    fn parse(parser: &Self::In) -> AstResult {
        return map_parser(parser, &[Self::parse_prefix_unary, Self::parse_suffix_unary, PrimativeParser::parse]);
    }
}

impl UnaryParser {
    fn parse_suffix_unary(parser: &SyntaxParser) -> AstResult {
        match &parser.peek_token() {
            Ok(token) => {
                if token.token_type.is_symbol() {
                    let index = parser.index.get();
                    parser.consume_token();

                    if let Some(operator) = parser.match_operator(&[
                        BramaOperatorType::Increment,
                        BramaOperatorType::Deccrement]) {
                        return Ok(BramaAstType::SuffixUnary(operator, Box::new(BramaAstType::Symbol(token.token_type.get_symbol().to_string()))));
                    }
                    else
                    {
                        parser.index.set(index);
                    }
                }
            },
            _ => ()
        };

        return Ok(BramaAstType::None);
    }

    fn parse_prefix_unary(parser: &SyntaxParser) -> AstResult {
        if let Some(operator) = parser.match_operator(&[BramaOperatorType::Addition,
            BramaOperatorType::Subtraction,
            BramaOperatorType::Increment,
            BramaOperatorType::Deccrement,
            BramaOperatorType::Not,
            BramaOperatorType::BitwiseNot]) {

            let mut unary_ast = BramaAstType::None;
            let token         = &parser.peek_token().unwrap();

            match operator {
                /* +1024 -1024 */
                BramaOperatorType::Addition | BramaOperatorType::Subtraction => {
                    if token.token_type.is_integer() || token.token_type.is_double() {
                        match PrimativeParser::parse(parser) {
                            Ok(BramaAstType::None) => (),
                            Ok(ast) => unary_ast = ast,
                            Err(err) => return Err(err)
                        }
                    }
                },

                /* ! */
                BramaOperatorType::Not => {
                    if token.token_type.is_integer() || token.token_type.is_double() || token.token_type.is_bool() {
                        if let Ok(ast) = PrimativeParser::parse(parser) {
                            unary_ast = ast;
                        }
                    }
                },

                /* ++variable, --variable*/
                BramaOperatorType::Increment | BramaOperatorType::Deccrement => {
                    if token.token_type.is_symbol() {
                        unary_ast = BramaAstType::Symbol(token.token_type.get_symbol().to_string());
                    }
                },
                _ => return Err(("Invalid unary operation", 0, 0))
            }

            return match unary_ast {
                BramaAstType::None => Err(("Invalid unary operation", 0, 0)),
                _ => Ok(BramaAstType::PrefixUnary(operator, Box::new(unary_ast)))
            };
        }

        return Ok(BramaAstType::None);
    }
}