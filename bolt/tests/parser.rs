#![allow(dead_code, unused_imports)]
use bolt::{
    lexer::{
        lexer,
        token::{Token, TokenType},
    },
    parser::{
        ast::{self, Expression, ExpressionStatement, Statement},
        parser::Parser,
    },
};

#[test]
fn test_integer_literal_xpression() {
    let input = "10;";
    let mut parser = Parser::new(&input);
    let p = parser.parse_program();
    match p {
        Ok(res) => {
            let stmt: &Box<dyn Statement> = &res.stmts[0];
            let expected_expression = Box::new(ast::IntegerLiteral {
                token: Token {
                    token_type: TokenType::INT,
                    literal: String::from("10"),
                    line: 0,
                },
                value: 10.0,
            });
            let actual_stmt = ExpressionStatement {
                token: Token {
                    token_type: TokenType::INT,
                    literal: String::from("10"),
                    line: 0,
                },
                value: expected_expression,
            };
            assert_eq!(format!("{:?}", stmt), format!("{:?}", actual_stmt));
        }
        Err(e) => {
            print!("Error - {:?}", e.message);
        }
    }
}
