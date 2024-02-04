#![allow(dead_code, unused_imports)]

use std::{rc::Rc, vec};

use bolt::{
    error::BoltError,
    lexer::{
        lexer,
        token::{self, Token, TokenType},
    },
    parser::{
        ast::{
            self, BinaryExpression, BlockStatement, Boolean, Expression, ExpressionStatement,
            FunctionLiteral, Identifier, IfExpression, IntegerLiteral, LetStatement,
            PrefixExpression, ReturnStatement, Statement,
        },
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
            panic!("Error - {:?}", e.get_message());
        }
    }
}

#[test]
fn test_prefix_minus_expression() {
    // Test -5;
    let input = "-5;";
    let mut parser = Parser::new(&input);
    let p = parser.parse_program();
    match p {
        Ok(res) => {
            let stmt: &Box<dyn Statement> = &res.stmts[0];
            let expected_expression = Box::new(ast::PrefixExpression {
                token: Token {
                    token_type: TokenType::MINUS,
                    literal: String::from("-"),
                    line: 0,
                },
                operator: String::from("-"),
                right: Box::new(IntegerLiteral {
                    token: Token {
                        token_type: TokenType::INT,
                        literal: String::from("5"),
                        line: 0,
                    },
                    value: 5.0,
                }),
            });
            let expected_stmt = ExpressionStatement {
                token: Token {
                    token_type: TokenType::MINUS,
                    literal: String::from("-"),
                    line: 0,
                },
                value: expected_expression,
            };
            assert_eq!(format!("{:?}", stmt), format!("{:?}", expected_stmt));
        }
        Err(e) => {
            panic!("Error - {:?}", e.get_message());
        }
    }
}

#[test]
fn test_prefix_bang_expression() {
    // Test !true;
    let input = "!true;";
    let mut parser = Parser::new(&input);
    let p = parser.parse_program();
    match p {
        Ok(res) => {
            let stmt: &Box<dyn Statement> = &res.stmts[0];
            let expected_expression = Box::new(ast::PrefixExpression {
                token: Token {
                    token_type: TokenType::BANG,
                    literal: String::from("!"),
                    line: 0,
                },
                operator: String::from("!"),
                right: Box::new(Boolean {
                    token: Token {
                        token_type: TokenType::TRUE,
                        literal: String::from("true"),
                        line: 0,
                    },
                    value: true,
                }),
            });
            let expected_stmt = ExpressionStatement {
                token: Token {
                    token_type: TokenType::BANG,
                    literal: String::from("!"),
                    line: 0,
                },
                value: expected_expression,
            };
            assert_eq!(format!("{:?}", stmt), format!("{:?}", expected_stmt));
        }
        Err(e) => {
            panic!("Error - {:?}", e.get_message());
        }
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = [
        "-a * b",
        "!-a;",
        "a  + b + c",
        "a + b - c",
        "a + b * c",
        "a * b / c",
        "5 < 4 == 3 > 4",
        "1 != 2",
    ];
    let expected_results = vec![
        ExpressionStatement {
            token: Token {
                token_type: TokenType::MINUS,
                literal: String::from("-"),
                line: 0,
            },
            value: Box::new(ast::BinaryExpression {
                token: Token {
                    token_type: TokenType::ASTERISK,
                    literal: String::from("*"),
                    line: 0,
                },
                operator: String::from("*"),
                left: Rc::new(Box::new(PrefixExpression {
                    token: Token {
                        token_type: TokenType::MINUS,
                        literal: String::from("-"),
                        line: 0,
                    },
                    operator: String::from("-"),
                    right: Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("a"),
                            line: 0,
                        },
                        value: String::from("a"),
                    }),
                })),
                right: Box::new(Identifier {
                    token: Token {
                        token_type: TokenType::IDENTIFIER,
                        literal: String::from("b"),
                        line: 0,
                    },
                    value: String::from("b"),
                }),
            }),
        },
        ExpressionStatement {
            token: Token {
                token_type: TokenType::BANG,
                literal: String::from("!"),
                line: 0,
            },
            value: Box::new(PrefixExpression {
                token: Token {
                    token_type: TokenType::BANG,
                    literal: String::from("!"),
                    line: 0,
                },
                operator: String::from("!"),
                right: Box::new(PrefixExpression {
                    token: Token {
                        token_type: TokenType::MINUS,
                        literal: String::from("-"),
                        line: 0,
                    },
                    operator: String::from("-"),
                    right: Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("a"),
                            line: 0,
                        },
                        value: String::from("a"),
                    }),
                }),
            }),
        },
        ExpressionStatement {
            token: Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("a"),
                line: 0,
            },
            value: Box::new(BinaryExpression {
                token: Token {
                    token_type: TokenType::PLUS,
                    literal: String::from("+"),
                    line: 0,
                },
                operator: String::from("+"),
                left: Rc::new(Box::new(BinaryExpression {
                    token: Token {
                        token_type: TokenType::PLUS,
                        literal: String::from("+"),
                        line: 0,
                    },
                    operator: String::from("+"),
                    left: Rc::new(Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("a"),
                            line: 0,
                        },
                        value: String::from("a"),
                    })),
                    right: Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("b"),
                            line: 0,
                        },
                        value: String::from("b"),
                    }),
                })),
                right: Box::new(Identifier {
                    token: Token {
                        token_type: TokenType::IDENTIFIER,
                        literal: String::from("c"),
                        line: 0,
                    },
                    value: String::from("c"),
                }),
            }),
        },
        ExpressionStatement {
            token: Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("a"),
                line: 0,
            },
            value: Box::new(BinaryExpression {
                token: Token {
                    token_type: TokenType::MINUS,
                    literal: String::from("-"),
                    line: 0,
                },
                operator: String::from("-"),
                left: Rc::new(Box::new(BinaryExpression {
                    token: Token {
                        token_type: TokenType::PLUS,
                        literal: String::from("+"),
                        line: 0,
                    },
                    operator: String::from("+"),
                    left: Rc::new(Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("a"),
                            line: 0,
                        },
                        value: String::from("a"),
                    })),
                    right: Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("b"),
                            line: 0,
                        },
                        value: String::from("b"),
                    }),
                })),
                right: Box::new(Identifier {
                    token: Token {
                        token_type: TokenType::IDENTIFIER,
                        literal: String::from("c"),
                        line: 0,
                    },
                    value: String::from("c"),
                }),
            }),
        },
        ExpressionStatement {
            token: Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("a"),
                line: 0,
            },
            value: Box::new(BinaryExpression {
                token: Token {
                    token_type: TokenType::PLUS,
                    literal: String::from("+"),
                    line: 0,
                },
                operator: String::from("+"),
                right: Box::new(BinaryExpression {
                    token: Token {
                        token_type: TokenType::ASTERISK,
                        literal: String::from("*"),
                        line: 0,
                    },
                    operator: String::from("*"),
                    left: Rc::new(Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("b"),
                            line: 0,
                        },
                        value: String::from("b"),
                    })),
                    right: Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("c"),
                            line: 0,
                        },
                        value: String::from("c"),
                    }),
                }),
                left: Rc::new(Box::new(Identifier {
                    token: Token {
                        token_type: TokenType::IDENTIFIER,
                        literal: String::from("a"),
                        line: 0,
                    },
                    value: String::from("a"),
                })),
            }),
        },
        ExpressionStatement {
            token: Token {
                token_type: TokenType::IDENTIFIER,
                literal: String::from("a"),
                line: 0,
            },
            value: Box::new(BinaryExpression {
                token: Token {
                    token_type: TokenType::SLASH,
                    literal: String::from("/"),
                    line: 0,
                },
                operator: String::from("/"),
                left: Rc::new(Box::new(BinaryExpression {
                    token: Token {
                        token_type: TokenType::ASTERISK,
                        literal: String::from("*"),
                        line: 0,
                    },
                    operator: String::from("*"),
                    left: Rc::new(Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("a"),
                            line: 0,
                        },
                        value: String::from("a"),
                    })),
                    right: Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("b"),
                            line: 0,
                        },
                        value: String::from("b"),
                    }),
                })),
                right: Box::new(Identifier {
                    token: Token {
                        token_type: TokenType::IDENTIFIER,
                        literal: String::from("c"),
                        line: 0,
                    },
                    value: String::from("c"),
                }),
            }),
        },
        ExpressionStatement {
            token: Token {
                token_type: TokenType::INT,
                literal: String::from("5"),
                line: 0,
            },
            value: Box::new(BinaryExpression {
                token: Token {
                    token_type: TokenType::EQ,
                    literal: String::from("=="),
                    line: 0,
                },
                operator: String::from("=="),
                left: Rc::new(Box::new(BinaryExpression {
                    token: Token {
                        token_type: TokenType::LT,
                        literal: String::from("<"),
                        line: 0,
                    },
                    operator: String::from("<"),
                    left: Rc::new(Box::new(IntegerLiteral {
                        token: Token {
                            token_type: TokenType::INT,
                            literal: String::from("5"),
                            line: 0,
                        },
                        value: 5.0,
                    })),
                    right: Box::new(IntegerLiteral {
                        token: Token {
                            token_type: TokenType::INT,
                            literal: String::from("4"),
                            line: 0,
                        },
                        value: 4.0,
                    }),
                })),
                right: Box::new(BinaryExpression {
                    token: Token {
                        token_type: TokenType::GT,
                        literal: String::from(">"),
                        line: 0,
                    },
                    operator: String::from(">"),
                    left: Rc::new(Box::new(IntegerLiteral {
                        token: Token {
                            token_type: TokenType::INT,
                            literal: String::from("3"),
                            line: 0,
                        },
                        value: 3.0,
                    })),
                    right: Box::new(IntegerLiteral {
                        token: Token {
                            token_type: TokenType::INT,
                            literal: String::from("4"),
                            line: 0,
                        },
                        value: 4.0,
                    }),
                }),
            }),
        },
        ExpressionStatement {
            token: Token {
                token_type: TokenType::INT,
                literal: String::from("1"),
                line: 0,
            },
            value: Box::new(BinaryExpression {
                token: Token {
                    token_type: TokenType::NOTEQ,
                    literal: String::from("!="),
                    line: 0,
                },
                operator: String::from("!="),
                left: Rc::new(Box::new(IntegerLiteral {
                    token: Token {
                        token_type: TokenType::INT,
                        literal: String::from("1"),
                        line: 0,
                    },
                    value: 1.0,
                })),
                right: Box::new(IntegerLiteral {
                    token: Token {
                        token_type: TokenType::INT,
                        literal: String::from("2"),
                        line: 0,
                    },
                    value: 2.0,
                }),
            }),
        },
    ];
    let size = tests.len();
    for i in 0..size {
        let mut parser = Parser::new(tests[i]);
        let p = parser.parse_program();
        match p {
            Ok(res) => {
                let stmt: &Box<dyn Statement> = &res.stmts[0];
                assert_eq!(format!("{:?}", stmt), format!("{:?}", expected_results[i]));
            }
            Err(e) => {
                panic!("Error - {:?}", e.get_message());
            }
        }
    }
}

#[test]
fn test_boolean_statement() {
    let tests = ["let a = true;", "let b = false"];
    let expected_results = vec![
        LetStatement {
            token: Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
                line: 0,
            },
            identifier: Identifier {
                token: Token {
                    token_type: TokenType::IDENTIFIER,
                    literal: String::from("a"),
                    line: 0,
                },
                value: String::from("a"),
            },
            value: Box::new(Boolean {
                token: Token {
                    token_type: TokenType::TRUE,
                    literal: String::from("true"),
                    line: 0,
                },
                value: true,
            }),
        },
        LetStatement {
            token: Token {
                token_type: TokenType::LET,
                literal: String::from("let"),
                line: 0,
            },
            identifier: Identifier {
                token: Token {
                    token_type: TokenType::IDENTIFIER,
                    literal: String::from("b"),
                    line: 0,
                },
                value: String::from("b"),
            },
            value: Box::new(Boolean {
                token: Token {
                    token_type: TokenType::FALSE,
                    literal: String::from("false"),
                    line: 0,
                },
                value: false,
            }),
        },
    ];
    let size = tests.len();
    for i in 0..size {
        let mut parser = Parser::new(tests[i]);
        let p = parser.parse_program();
        match p {
            Ok(res) => {
                let stmt: &Box<dyn Statement> = &res.stmts[0];
                assert_eq!(format!("{:?}", stmt), format!("{:?}", expected_results[i]));
            }
            Err(e) => {
                panic!("Error - {:?}", e.get_message());
            }
        }
    }
}

#[test]
fn test_boolean_expression() {
    let tests = ["true", "false", "5 < 8 == true;", "3 > 4 == false;"];
    let expected_results = vec![
        ExpressionStatement {
            token: Token {
                token_type: TokenType::TRUE,
                literal: String::from("true"),
                line: 0,
            },
            value: Box::new(Boolean {
                token: Token {
                    token_type: TokenType::TRUE,
                    literal: String::from("true"),
                    line: 0,
                },
                value: true,
            }),
        },
        ExpressionStatement {
            token: Token {
                token_type: TokenType::FALSE,
                literal: String::from("false"),
                line: 0,
            },
            value: Box::new(Boolean {
                token: Token {
                    token_type: TokenType::FALSE,
                    literal: String::from("false"),
                    line: 0,
                },
                value: false,
            }),
        },
        ExpressionStatement {
            token: Token {
                token_type: TokenType::INT,
                literal: String::from("5"),
                line: 0,
            },
            value: Box::new(BinaryExpression {
                token: Token {
                    token_type: TokenType::EQ,
                    literal: String::from("=="),
                    line: 0,
                },
                operator: String::from("=="),
                left: Rc::new(Box::new(BinaryExpression {
                    token: Token {
                        token_type: TokenType::LT,
                        literal: String::from("<"),
                        line: 0,
                    },
                    operator: String::from("<"),
                    left: Rc::new(Box::new(IntegerLiteral {
                        token: Token {
                            token_type: TokenType::INT,
                            literal: String::from("5"),
                            line: 0,
                        },
                        value: 5.0,
                    })),
                    right: Box::new(IntegerLiteral {
                        token: Token {
                            token_type: TokenType::INT,
                            literal: String::from("8"),
                            line: 0,
                        },
                        value: 8.0,
                    }),
                })),
                right: Box::new(Boolean {
                    token: Token {
                        token_type: TokenType::TRUE,
                        literal: String::from("true"),
                        line: 0,
                    },
                    value: true,
                }),
            }),
        },
        ExpressionStatement {
            token: Token {
                token_type: TokenType::INT,
                literal: String::from("3"),
                line: 0,
            },
            value: Box::new(BinaryExpression {
                token: Token {
                    token_type: TokenType::EQ,
                    literal: String::from("=="),
                    line: 0,
                },
                operator: String::from("=="),
                left: Rc::new(Box::new(BinaryExpression {
                    token: Token {
                        token_type: TokenType::GT,
                        literal: String::from(">"),
                        line: 0,
                    },
                    operator: String::from(">"),
                    left: Rc::new(Box::new(IntegerLiteral {
                        token: Token {
                            token_type: TokenType::INT,
                            literal: String::from("3"),
                            line: 0,
                        },
                        value: 3.0,
                    })),
                    right: Box::new(IntegerLiteral {
                        token: Token {
                            token_type: TokenType::INT,
                            literal: String::from("4"),
                            line: 0,
                        },
                        value: 4.0,
                    }),
                })),
                right: Box::new(Boolean {
                    token: Token {
                        token_type: TokenType::FALSE,
                        literal: String::from("false"),
                        line: 0,
                    },
                    value: false,
                }),
            }),
        },
    ];
    let size = tests.len();
    for i in 0..size {
        let mut parser = Parser::new(tests[i]);
        let p = parser.parse_program();
        match p {
            Ok(res) => {
                let stmt: &Box<dyn Statement> = &res.stmts[0];
                assert_eq!(format!("{:?}", stmt), format!("{:?}", expected_results[i]));
            }
            Err(e) => {
                panic!("Error - {:?}", e.get_message());
            }
        }
    }
}

#[test]
fn test_grouped_expressions() {
    let tests = ["1 + (2 + 3)", "(5 + 5) * 2", "-(5 + 5)"];
    let expected_results = vec![
        ExpressionStatement {
            token: Token {
                token_type: TokenType::INT,
                literal: String::from("1"),
                line: 0,
            },
            value: Box::new(BinaryExpression {
                token: Token {
                    token_type: TokenType::PLUS,
                    literal: String::from("+"),
                    line: 0,
                },
                operator: String::from("+"),
                left: Rc::new(Box::new(IntegerLiteral {
                    token: Token {
                        token_type: TokenType::INT,
                        literal: String::from("1"),
                        line: 0,
                    },
                    value: 1.0,
                })),
                right: Box::new(BinaryExpression {
                    token: Token {
                        token_type: TokenType::PLUS,
                        literal: String::from("+"),
                        line: 0,
                    },
                    operator: String::from("+"),
                    left: Rc::new(Box::new(IntegerLiteral {
                        token: Token {
                            token_type: TokenType::INT,
                            literal: String::from("2"),
                            line: 0,
                        },
                        value: 2.0,
                    })),
                    right: Box::new(IntegerLiteral {
                        token: Token {
                            token_type: TokenType::INT,
                            literal: String::from("3"),
                            line: 0,
                        },
                        value: 3.0,
                    }),
                }),
            }),
        },
        ExpressionStatement {
            token: Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
                line: 0,
            },
            value: Box::new(BinaryExpression {
                token: Token {
                    token_type: TokenType::ASTERISK,
                    literal: String::from("*"),
                    line: 0,
                },
                operator: String::from("*"),
                left: Rc::new(Box::new(BinaryExpression {
                    token: Token {
                        token_type: TokenType::PLUS,
                        literal: String::from("+"),
                        line: 0,
                    },
                    operator: String::from("+"),
                    left: Rc::new(Box::new(IntegerLiteral {
                        token: Token {
                            token_type: TokenType::INT,
                            literal: String::from("5"),
                            line: 0,
                        },
                        value: 5.0,
                    })),
                    right: Box::new(IntegerLiteral {
                        token: Token {
                            token_type: TokenType::INT,
                            literal: String::from("5"),
                            line: 0,
                        },
                        value: 5.0,
                    }),
                })),
                right: Box::new(IntegerLiteral {
                    token: Token {
                        token_type: TokenType::INT,
                        literal: String::from("2"),
                        line: 0,
                    },
                    value: 2.0,
                }),
            }),
        },
        ExpressionStatement {
            token: Token {
                token_type: TokenType::MINUS,
                literal: String::from("-"),
                line: 0,
            },
            value: Box::new(PrefixExpression {
                token: Token {
                    token_type: TokenType::MINUS,
                    literal: String::from("-"),
                    line: 0,
                },
                operator: String::from("-"),
                right: Box::new(BinaryExpression {
                    token: Token {
                        token_type: TokenType::PLUS,
                        literal: String::from("+"),
                        line: 0,
                    },
                    operator: String::from("+"),
                    left: Rc::new(Box::new(IntegerLiteral {
                        token: Token {
                            token_type: TokenType::INT,
                            literal: String::from("5"),
                            line: 0,
                        },
                        value: 5.0,
                    })),
                    right: Box::new(IntegerLiteral {
                        token: Token {
                            token_type: TokenType::INT,
                            literal: String::from("5"),
                            line: 0,
                        },
                        value: 5.0,
                    }),
                }),
            }),
        },
    ];
    let size = tests.len();
    for i in 0..size {
        let mut parser = Parser::new(tests[i]);
        let p = parser.parse_program();
        match p {
            Ok(res) => {
                let stmt: &Box<dyn Statement> = &res.stmts[0];
                assert_eq!(format!("{:?}", stmt), format!("{:?}", expected_results[i]));
            }
            Err(e) => {
                panic!("Error - {:?}", e.get_message());
            }
        }
    }
}

#[test]
fn test_if_expression() {
    let tests = [
        "if (x > y) { x };",
        "if (x > y) { x } else { y }",
        "if (x > y) { let a = 1;\n let b = 2; }",
        "if (x > y) { x }\nelse { let a = 1;\n let b = 2; }",
    ];
    let expected_results = vec![
        ExpressionStatement {
            token: Token {
                token_type: TokenType::IF,
                literal: String::from("if"),
                line: 0,
            },
            value: Box::new(IfExpression {
                token: Token {
                    token_type: TokenType::IF,
                    literal: String::from("if"),
                    line: 0,
                },
                condition: Box::new(BinaryExpression {
                    token: Token {
                        token_type: TokenType::GT,
                        literal: String::from(">"),
                        line: 0,
                    },
                    operator: String::from(">"),
                    left: Rc::new(Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("x"),
                            line: 0,
                        },
                        value: String::from("x"),
                    })),
                    right: Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("y"),
                            line: 0,
                        },
                        value: String::from("y"),
                    }),
                }),
                consequence: Box::new(BlockStatement {
                    token: Token {
                        token_type: TokenType::IDENTIFIER,
                        literal: String::from("x"),
                        line: 0,
                    },
                    statements: vec![Box::new(ExpressionStatement {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("x"),
                            line: 0,
                        },
                        value: Box::new(Identifier {
                            token: Token {
                                token_type: TokenType::IDENTIFIER,
                                literal: String::from("x"),
                                line: 0,
                            },
                            value: String::from("x"),
                        }),
                    })],
                }),
                alternate: None,
            }),
        },
        ExpressionStatement {
            token: Token {
                token_type: TokenType::IF,
                literal: String::from("if"),
                line: 0,
            },
            value: Box::new(IfExpression {
                token: Token {
                    token_type: TokenType::IF,
                    literal: String::from("if"),
                    line: 0,
                },
                condition: Box::new(BinaryExpression {
                    token: Token {
                        token_type: TokenType::GT,
                        literal: String::from(">"),
                        line: 0,
                    },
                    operator: String::from(">"),
                    left: Rc::new(Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("x"),
                            line: 0,
                        },
                        value: String::from("x"),
                    })),
                    right: Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("y"),
                            line: 0,
                        },
                        value: String::from("y"),
                    }),
                }),
                consequence: Box::new(BlockStatement {
                    token: Token {
                        token_type: TokenType::IDENTIFIER,
                        literal: String::from("x"),
                        line: 0,
                    },
                    statements: vec![Box::new(ExpressionStatement {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("x"),
                            line: 0,
                        },
                        value: Box::new(Identifier {
                            token: Token {
                                token_type: TokenType::IDENTIFIER,
                                literal: String::from("x"),
                                line: 0,
                            },
                            value: String::from("x"),
                        }),
                    })],
                }),
                alternate: Some(Box::new(BlockStatement {
                    token: Token {
                        token_type: TokenType::IDENTIFIER,
                        literal: String::from("y"),
                        line: 0,
                    },
                    statements: vec![Box::new(ExpressionStatement {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("y"),
                            line: 0,
                        },
                        value: Box::new(Identifier {
                            token: Token {
                                token_type: TokenType::IDENTIFIER,
                                literal: String::from("y"),
                                line: 0,
                            },
                            value: String::from("y"),
                        }),
                    })],
                })),
            }),
        },
        ExpressionStatement {
            token: Token {
                token_type: TokenType::IF,
                literal: String::from("if"),
                line: 0,
            },
            value: Box::new(IfExpression {
                token: Token {
                    token_type: TokenType::IF,
                    literal: String::from("if"),
                    line: 0,
                },
                condition: Box::new(BinaryExpression {
                    token: Token {
                        token_type: TokenType::GT,
                        literal: String::from(">"),
                        line: 0,
                    },
                    operator: String::from(">"),
                    left: Rc::new(Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("x"),
                            line: 0,
                        },
                        value: String::from("x"),
                    })),
                    right: Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("y"),
                            line: 0,
                        },
                        value: String::from("y"),
                    }),
                }),
                consequence: Box::new(BlockStatement {
                    token: Token {
                        token_type: TokenType::LET,
                        literal: String::from("let"),
                        line: 0,
                    },
                    statements: vec![
                        Box::new(LetStatement {
                            token: Token {
                                token_type: TokenType::LET,
                                literal: String::from("let"),
                                line: 0,
                            },
                            identifier: Identifier {
                                token: Token {
                                    token_type: TokenType::IDENTIFIER,
                                    literal: String::from("a"),
                                    line: 0,
                                },
                                value: String::from("a"),
                            },
                            value: Box::new(IntegerLiteral {
                                token: Token {
                                    token_type: TokenType::INT,
                                    literal: String::from("1"),
                                    line: 0,
                                },
                                value: 1.0,
                            }),
                        }),
                        Box::new(LetStatement {
                            token: Token {
                                token_type: TokenType::LET,
                                literal: String::from("let"),
                                line: 1,
                            },
                            identifier: Identifier {
                                token: Token {
                                    token_type: TokenType::IDENTIFIER,
                                    literal: String::from("b"),
                                    line: 1,
                                },
                                value: String::from("b"),
                            },
                            value: Box::new(IntegerLiteral {
                                token: Token {
                                    token_type: TokenType::INT,
                                    literal: String::from("2"),
                                    line: 1,
                                },
                                value: 2.0,
                            }),
                        }),
                    ],
                }),
                alternate: None,
            }),
        },
        ExpressionStatement {
            token: Token {
                token_type: TokenType::IF,
                literal: String::from("if"),
                line: 0,
            },
            value: Box::new(IfExpression {
                token: Token {
                    token_type: TokenType::IF,
                    literal: String::from("if"),
                    line: 0,
                },
                condition: Box::new(BinaryExpression {
                    token: Token {
                        token_type: TokenType::GT,
                        literal: String::from(">"),
                        line: 0,
                    },
                    operator: String::from(">"),
                    left: Rc::new(Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("x"),
                            line: 0,
                        },
                        value: String::from("x"),
                    })),
                    right: Box::new(Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("y"),
                            line: 0,
                        },
                        value: String::from("y"),
                    }),
                }),
                consequence: Box::new(BlockStatement {
                    token: Token {
                        token_type: TokenType::IDENTIFIER,
                        literal: String::from("x"),
                        line: 0,
                    },
                    statements: vec![Box::new(ExpressionStatement {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("x"),
                            line: 0,
                        },
                        value: Box::new(Identifier {
                            token: Token {
                                token_type: TokenType::IDENTIFIER,
                                literal: String::from("x"),
                                line: 0,
                            },
                            value: String::from("x"),
                        }),
                    })],
                }),
                alternate: Some(Box::new(BlockStatement {
                    token: Token {
                        token_type: TokenType::LET,
                        literal: String::from("let"),
                        line: 1,
                    },
                    statements: vec![
                        Box::new(LetStatement {
                            token: Token {
                                token_type: TokenType::LET,
                                literal: String::from("let"),
                                line: 1,
                            },
                            identifier: Identifier {
                                token: Token {
                                    token_type: TokenType::IDENTIFIER,
                                    literal: String::from("a"),
                                    line: 1,
                                },
                                value: String::from("a"),
                            },
                            value: Box::new(IntegerLiteral {
                                token: Token {
                                    token_type: TokenType::INT,
                                    literal: String::from("1"),
                                    line: 1,
                                },
                                value: 1.0,
                            }),
                        }),
                        Box::new(LetStatement {
                            token: Token {
                                token_type: TokenType::LET,
                                literal: String::from("let"),
                                line: 2,
                            },
                            identifier: Identifier {
                                token: Token {
                                    token_type: TokenType::IDENTIFIER,
                                    literal: String::from("b"),
                                    line: 2,
                                },
                                value: String::from("b"),
                            },
                            value: Box::new(IntegerLiteral {
                                token: Token {
                                    token_type: TokenType::INT,
                                    literal: String::from("2"),
                                    line: 2,
                                },
                                value: 2.0,
                            }),
                        }),
                    ],
                })),
            }),
        },
    ];
    let size = tests.len();
    for i in 0..size {
        let mut parser = Parser::new(tests[i]);
        let p = parser.parse_program();
        match p {
            Ok(res) => {
                let stmt: &Box<dyn Statement> = &res.stmts[0];
                assert_eq!(format!("{:?}", stmt), format!("{:?}", expected_results[i]));
            }
            Err(e) => {
                panic!("Error - {:?}", e.get_message());
            }
        }
    }
}

#[test]
fn test_function_parameter_parsing() {
    let tests = ["fn(){}", "fn(x, y){return x + y;}"];
    let expected_results = vec![
        Box::new(ExpressionStatement {
            token: Token {
                token_type: TokenType::FUNCTION,
                literal: String::from("fn"),
                line: 0,
            },
            value: Box::new(FunctionLiteral {
                token: Token {
                    token_type: TokenType::FUNCTION,
                    literal: String::from("fn"),
                    line: 0,
                },
                parameters: Rc::new(vec![]),
                body: Rc::new(Box::new(BlockStatement {
                    token: Token {
                        token_type: TokenType::RBRACE,
                        literal: String::from("}"),
                        line: 0,
                    },
                    statements: vec![],
                })),
            }),
        }),
        Box::new(ExpressionStatement {
            token: Token {
                token_type: TokenType::FUNCTION,
                literal: String::from("fn"),
                line: 0,
            },
            value: Box::new(FunctionLiteral {
                token: Token {
                    token_type: TokenType::FUNCTION,
                    literal: String::from("fn"),
                    line: 0,
                },
                parameters: Rc::new(vec![
                    Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("x"),
                            line: 0,
                        },
                        value: String::from("x"),
                    },
                    Identifier {
                        token: Token {
                            token_type: TokenType::IDENTIFIER,
                            literal: String::from("y"),
                            line: 0,
                        },
                        value: String::from("y"),
                    },
                ]),
                body: Rc::new(Box::new(BlockStatement {
                    token: Token {
                        token_type: TokenType::RETURN,
                        literal: String::from("return"),
                        line: 0,
                    },
                    statements: vec![Box::new(ReturnStatement {
                        token: Token {
                            token_type: TokenType::RETURN,
                            literal: String::from("return"),
                            line: 0,
                        },
                        value: Box::new(BinaryExpression {
                            token: Token {
                                token_type: TokenType::PLUS,
                                literal: String::from("+"),
                                line: 0,
                            },
                            operator: String::from("+"),
                            left: Rc::new(Box::new(Identifier {
                                token: Token {
                                    token_type: TokenType::IDENTIFIER,
                                    literal: String::from("x"),
                                    line: 0,
                                },
                                value: String::from("x"),
                            })),
                            right: Box::new(Identifier {
                                token: Token {
                                    token_type: TokenType::IDENTIFIER,
                                    literal: String::from("y"),
                                    line: 0,
                                },
                                value: String::from("y"),
                            }),
                        }),
                    })],
                })),
            }),
        }),
    ];
    let size = tests.len();
    for i in 0..size {
        let mut parser = Parser::new(tests[i]);
        let p = parser.parse_program();
        match p {
            Ok(res) => {
                let stmt: &Box<dyn Statement> = &res.stmts[0];
                assert_eq!(format!("{:?}", stmt), format!("{:?}", expected_results[i]));
            }
            Err(e) => {
                panic!("Error - {:?}", e.get_message());
            }
        }
    }
}

#[test]
fn test_call_expression_parsing() {
    let input = "add(1, 2 * 3, 4 + 5);";
    let mut parser = Parser::new(&input);
    let p = parser.parse_program();
    match p {
        Ok(res) => {
            let stmt: &Box<dyn Statement> = &res.stmts[0];
            let expected_expression = Box::new(ast::CallExpression {
                token: Token {
                    token_type: TokenType::LPAREN,
                    literal: String::from("("),
                    line: 0,
                },
                funtion: Rc::new(Box::new(Identifier {
                    token: Token {
                        token_type: TokenType::IDENTIFIER,
                        literal: String::from("add"),
                        line: 0,
                    },
                    value: String::from("add"),
                })),
                parameters: Rc::new(vec![
                    Box::new(IntegerLiteral {
                        token: Token {
                            token_type: TokenType::INT,
                            literal: String::from("1"),
                            line: 0,
                        },
                        value: 1.0,
                    }),
                    Box::new(BinaryExpression {
                        token: Token {
                            token_type: TokenType::ASTERISK,
                            literal: String::from("*"),
                            line: 0,
                        },
                        operator: String::from("*"),
                        left: Rc::new(Box::new(IntegerLiteral {
                            token: Token {
                                token_type: TokenType::INT,
                                literal: String::from("2"),
                                line: 0,
                            },
                            value: 2.0,
                        })),
                        right: Box::new(IntegerLiteral {
                            token: Token {
                                token_type: TokenType::INT,
                                literal: String::from("3"),
                                line: 0,
                            },
                            value: 3.0,
                        }),
                    }),
                    Box::new(BinaryExpression {
                        token: Token {
                            token_type: TokenType::PLUS,
                            literal: String::from("+"),
                            line: 0,
                        },
                        operator: String::from("+"),
                        left: Rc::new(Box::new(IntegerLiteral {
                            token: Token {
                                token_type: TokenType::INT,
                                literal: String::from("4"),
                                line: 0,
                            },
                            value: 4.0,
                        })),
                        right: Box::new(IntegerLiteral {
                            token: Token {
                                token_type: TokenType::INT,
                                literal: String::from("5"),
                                line: 0,
                            },
                            value: 5.0,
                        }),
                    }),
                ]),
            });
            let actual_stmt = ExpressionStatement {
                token: Token {
                    token_type: TokenType::IDENTIFIER,
                    literal: String::from("add"),
                    line: 0,
                },
                value: expected_expression,
            };
            assert_eq!(format!("{:?}", stmt), format!("{:?}", actual_stmt));
        }
        Err(e) => {
            panic!("Error - {:?}", e.get_message());
        }
    }
}
