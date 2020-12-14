pub mod primative;
pub mod unary;
pub mod util;
pub mod binary;
pub mod control;
pub mod block;
pub mod assignment;
pub mod func_call;
pub mod newline;
pub mod if_condition;
pub mod statement;

use std::vec::Vec;
use std::cell::Cell;

use crate::types::*;
use self::block::MultiLineBlockParser;
use crate::compiler::ast::BramaAstType;

pub type ParseType = fn(parser: &SyntaxParser) -> AstResult;

pub struct SyntaxParser {
    pub tokens: Box<Vec<Token>>,
    pub index: Cell<usize>,
    pub backup_index: Cell<usize>,
    pub indentation: Cell<usize>,
    pub setable_indentation: Cell<bool>
}

pub trait SyntaxParserTrait {
    fn parse(parser: &SyntaxParser) -> AstResult;
}

impl SyntaxParser {
    pub fn new(tokens: Box<Vec<Token>>) -> SyntaxParser {
        SyntaxParser {
            tokens: tokens,
            index: Cell::new(0),
            backup_index: Cell::new(0),
            indentation: Cell::new(0),
            setable_indentation: Cell::new(false)
        }
    }

    pub fn parse(&self) -> AstResult {
        return match MultiLineBlockParser::parse(&self) {
            Ok(ast) => {
                if let Ok(token) = self.next_token() {
                    println!("{:?}", token);
                    return Err(("Syntax error, undefined syntax", token.line, token.column));
                }
                Ok(ast)
            },
            other => other
        };
    }

    pub fn backup(&self) {
        self.backup_index.set(self.index.get());
    }

    pub fn restore(&self) {
        self.index.set(self.backup_index.get());
    }

    pub fn set_indentation(&self, indentation: usize) {
        self.indentation.set(indentation);
        self.setable_indentation.set(false);
    }

    pub fn get_indentation(&self) -> usize {
        self.indentation.get()
    }

    pub fn is_same_indentation(&self, indentation: usize) -> bool {
        if let Err((_,_,_)) = self.indentation_check() {
            return false;
        }
        else if indentation != self.get_indentation() {
            return false;
        }
        else if self.peek_token().is_err() {
            return false;
        }

        return true;
    }

    pub fn indentation_setable(&self) {
        self.setable_indentation.set(true);
    }

    pub fn peek_token(&self) -> Result<&Token, ()> {
        match self.tokens.get(self.index.get()) {
            Some(token) => Ok(token),
            None => Err(())
        }
    }

    pub fn next_token(&self) -> Result<&Token, ()> {
        match self.tokens.get(self.index.get() + 1) {
            Some(token) => Ok(token),
            None => Err(())
        }
    }
    
    pub fn consume_token(&self) -> Option<&Token> {
        self.index.set(self.index.get() + 1);
        self.tokens.get(self.index.get())
    }

    fn match_keyword(&self, keyword: BramaKeywordType) -> bool {
        let token = self.peek_token();
        if token.is_err() { return false; }
        return match token.unwrap().token_type {
            BramaTokenType::Keyword(token_keyword) => {
                if keyword == token_keyword {
                    self.consume_token();
                    return true;
                }
                return false;
            },
            _ => false
        }
    }

    fn get_newline(&self) -> (bool, usize) {
        let token = self.peek_token();
        if token.is_err() { return (false, 0); }
        return match token.unwrap().token_type {
            BramaTokenType::NewLine(size) => (true, size as usize),
            _ => (false, 0)
        }
    }

    fn check_operator(&self, operator: &BramaOperatorType) -> bool {
        let token = self.peek_token();
        if token.is_err() { return false; }
        return match token.unwrap().token_type {
            BramaTokenType::Operator(token_operator) => *operator == token_operator,
            _ => false
        }
    }

    fn match_operator(&self, operators: &[BramaOperatorType]) -> Option<BramaOperatorType> {
        for operator in operators {
            if self.check_operator(operator) {
                self.consume_token();
                return Some(*operator);
            }
        }

        return None;
    }

    fn clear_whitespaces(&self) {
        loop {
            if let Ok(current_token) = self.peek_token() {
                let done = match current_token.token_type {
                    BramaTokenType::WhiteSpace(_) => false,
                    _ => true
                };

                if done {
                    break;
                }

                self.consume_token();
            }
            else {
                break;
            }
        }
    }

    fn indentation_check(&self) -> AstResult {
        if self.next_token().is_err() {
            return Ok(BramaAstType::None);
        }

        loop {
            if let Ok(current_token) = self.peek_token() {
                
                let success = match current_token.token_type {
                    BramaTokenType::NewLine(size) => {
                        let token_type = &self.next_token().unwrap().token_type;
                        if let BramaTokenType::NewLine(_) = token_type {
                            /* If next token is newline, no need to check */
                            true
                        }
                        else {
                            /* Next token not a new line but space should be bigger than the current */
                            size == self.indentation.get() as u8 
                        }
                    },
                    
                    BramaTokenType::WhiteSpace(size) => {
                        size == self.indentation.get() as u8 
                    },
                    _ => break
                };

                if !success {
                    let token = self.peek_token().unwrap();
                    return Err(("Indentation issue", token.line, token.column));
                }

                self.consume_token();
            }
            else {
                break;
            }
        }

        Ok(BramaAstType::None)
    }

    fn in_indication(&self) -> AstResult {
        if self.next_token().is_err() {
            return Ok(BramaAstType::None);
        }

        loop {
            if let Ok(current_token) = self.peek_token() {
                
                let success = match current_token.token_type {
                    BramaTokenType::NewLine(size) => {
                        let token_type = &self.next_token().unwrap().token_type;
                        if let BramaTokenType::NewLine(_) = token_type {
                            /* If next token is newline, no need to check */
                            true
                        }
                        else {
                            /* Next token not a new line but space should be bigger than the current */
                            if size > self.indentation.get() as u8 {
                                self.set_indentation(size as usize);
                                true
                            }
                            else {
                                false
                            }
                        }
                    },
                    _ => break
                };

                if !success {
                    let token = self.peek_token().unwrap();
                    return Err(("Indentation issue", token.line, token.column));
                }

                self.consume_token();
            }
            else {
                break;
            }
        }

        Ok(BramaAstType::None)
    }
}