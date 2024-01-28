#![allow(dead_code, unused_imports)]

use std::rc::Rc;

use bolt::{
    lexer::{
        lexer,
        token::{self, Token, TokenType},
    },
    parser::{
        ast::{
            self, BinaryExpression, Boolean, Expression, ExpressionStatement, Identifier,
            IntegerLiteral, LetStatement, PrefixExpression, Statement,
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
            print!("Error - {:?}", e.message);
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
            print!("Error - {:?}", e.message);
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
            print!("Error - {:?}", e.message);
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
                print!("Error - {:?}", e.message);
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
                print!("Error - {:?}", e.message);
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
                print!("Error - {:?}", e.message);
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
                print!("Error - {:?}", e.message);
            }
        }
    }
}
