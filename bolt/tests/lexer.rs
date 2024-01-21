use bolt::lexer::{
    lexer::Lexer,
    token::{Token, TokenType},
};
#[test]
fn next_token() {
    let mut lexer = Lexer::new(
        "let five = (5);
        let ten = 10;
        let add = fn(x, y) {
        x + y;
        };
        let final_result = add(five, ten); 
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        10 == 10;
        10 != 9;
        //comment
        12;",
    );
    let mut tokens: Vec<Token> = Vec::new();
    let test_tokens = vec![
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
            line: 0,
        },
        Token {
            token_type: TokenType::IDENTIFIER,
            literal: String::from("five"),
            line: 0,
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
            line: 0,
        },
        Token {
            token_type: TokenType::LPAREN,
            literal: String::from("("),
            line: 0,
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("5"),
            line: 0,
        },
        Token {
            token_type: TokenType::RPAREN,
            literal: String::from(")"),
            line: 0,
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
            line: 0,
        },
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
            line: 1,
        },
        Token {
            token_type: TokenType::IDENTIFIER,
            literal: String::from("ten"),
            line: 1,
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
            line: 1,
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("10"),
            line: 1,
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
            line: 1,
        },
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
            line: 2,
        },
        Token {
            token_type: TokenType::IDENTIFIER,
            literal: String::from("add"),
            line: 2,
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
            line: 2,
        },
        Token {
            token_type: TokenType::FUNCTION,
            literal: String::from("fn"),
            line: 2,
        },
        Token {
            token_type: TokenType::LPAREN,
            literal: String::from("("),
            line: 2,
        },
        Token {
            token_type: TokenType::IDENTIFIER,
            literal: String::from("x"),
            line: 2,
        },
        Token {
            token_type: TokenType::COMMA,
            literal: String::from(","),
            line: 2,
        },
        Token {
            token_type: TokenType::IDENTIFIER,
            literal: String::from("y"),
            line: 2,
        },
        Token {
            token_type: TokenType::RPAREN,
            literal: String::from(")"),
            line: 2,
        },
        Token {
            token_type: TokenType::LBRACE,
            literal: String::from("{"),
            line: 2,
        },
        Token {
            token_type: TokenType::IDENTIFIER,
            literal: String::from("x"),
            line: 3,
        },
        Token {
            token_type: TokenType::PLUS,
            literal: String::from("+"),
            line: 3,
        },
        Token {
            token_type: TokenType::IDENTIFIER,
            literal: String::from("y"),
            line: 3,
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
            line: 3,
        },
        Token {
            token_type: TokenType::RBRACE,
            literal: String::from("}"),
            line: 4,
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
            line: 4,
        },
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
            line: 5,
        },
        Token {
            token_type: TokenType::IDENTIFIER,
            literal: String::from("final_result"),
            line: 5,
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
            line: 5,
        },
        Token {
            token_type: TokenType::IDENTIFIER,
            literal: String::from("add"),
            line: 5,
        },
        Token {
            token_type: TokenType::LPAREN,
            literal: String::from("("),
            line: 5,
        },
        Token {
            token_type: TokenType::IDENTIFIER,
            literal: String::from("five"),
            line: 5,
        },
        Token {
            token_type: TokenType::COMMA,
            literal: String::from(","),
            line: 5,
        },
        Token {
            token_type: TokenType::IDENTIFIER,
            literal: String::from("ten"),
            line: 5,
        },
        Token {
            token_type: TokenType::RPAREN,
            literal: String::from(")"),
            line: 5,
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
            line: 5,
        },
        Token {
            token_type: TokenType::BANG,
            literal: String::from("!"),
            line: 6,
        },
        Token {
            token_type: TokenType::MINUS,
            literal: String::from("-"),
            line: 6,
        },
        Token {
            token_type: TokenType::SLASH,
            literal: String::from("/"),
            line: 6,
        },
        Token {
            token_type: TokenType::ASTERISK,
            literal: String::from("*"),
            line: 6,
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("5"),
            line: 6,
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
            line: 6,
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("5"),
            line: 7,
        },
        Token {
            token_type: TokenType::LT,
            literal: String::from("<"),
            line: 7,
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("10"),
            line: 7,
        },
        Token {
            token_type: TokenType::GT,
            literal: String::from(">"),
            line: 7,
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("5"),
            line: 7,
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
            line: 7,
        },
        Token {
            token_type: TokenType::IF,
            literal: String::from("if"),
            line: 8,
        },
        Token {
            token_type: TokenType::LPAREN,
            literal: String::from("("),
            line: 8,
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("5"),
            line: 8,
        },
        Token {
            token_type: TokenType::LT,
            literal: String::from("<"),
            line: 8,
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("10"),
            line: 8,
        },
        Token {
            token_type: TokenType::RPAREN,
            literal: String::from(")"),
            line: 8,
        },
        Token {
            token_type: TokenType::LBRACE,
            literal: String::from("{"),
            line: 8,
        },
        Token {
            token_type: TokenType::RETURN,
            literal: String::from("return"),
            line: 9,
        },
        Token {
            token_type: TokenType::TRUE,
            literal: String::from("true"),
            line: 9,
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
            line: 9,
        },
        Token {
            token_type: TokenType::RBRACE,
            literal: String::from("}"),
            line: 10,
        },
        Token {
            token_type: TokenType::ELSE,
            literal: String::from("else"),
            line: 10,
        },
        Token {
            token_type: TokenType::LBRACE,
            literal: String::from("{"),
            line: 10,
        },
        Token {
            token_type: TokenType::RETURN,
            literal: String::from("return"),
            line: 11,
        },
        Token {
            token_type: TokenType::FALSE,
            literal: String::from("false"),
            line: 11,
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
            line: 11,
        },
        Token {
            token_type: TokenType::RBRACE,
            literal: String::from("}"),
            line: 12,
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("10"),
            line: 13,
        },
        Token {
            token_type: TokenType::EQ,
            literal: String::from("=="),
            line: 13,
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("10"),
            line: 13,
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
            line: 13,
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("10"),
            line: 14,
        },
        Token {
            token_type: TokenType::NOTEQ,
            literal: String::from("!="),
            line: 14,
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("9"),
            line: 14,
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
            line: 14,
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("12"),
            line: 16,
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
            line: 16,
        },
    ];
    loop {
        let token = lexer.next_token();
        if token.token_type == TokenType::EOF {
            break;
        }
        println!("{:#?}", token);
        tokens.push(token);
    }
    assert_eq!(tokens, test_tokens);
}
