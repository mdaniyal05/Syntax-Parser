#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Token {
    Int,
    Bool,
    String,
    If,
    For,
    Identifier,
    Number,
    Boolean,
    Plus,
    Minus,
    Assign,
    Greater,
    Less,
    And,
    Or,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    EOF,
}

struct Lexer {
    tokens: Vec<(Token, usize)>,
    pos: usize,
}

impl Lexer {
    fn new(tokens: Vec<(Token, usize)>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn next(&mut self) -> (Token, usize) {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            t
        } else {
            (Token::EOF, 0)
        }
    }
}

struct Parser {
    lexer: Lexer,
    current: Token,
    line: usize,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let (token, line) = lexer.next();
        Self { lexer, current: token, line }
    }

    fn advance(&mut self) {
        let (token, line) = self.lexer.next();
        self.current = token;
        self.line = line;
    }

    fn error(&self, msg: &str) {
        panic!("Syntax Error at line {}: {}", self.line, msg);
    }

    fn parse_program(&mut self) {
        while !matches!(self.current, Token::EOF) {
            if matches!(self.current, Token::Int | Token::Bool | Token::String) {
                self.parse_declaration();
            } else {
                self.parse_statement();
            }
        }
    }

    fn parse_declaration(&mut self) {
        self.advance();

        if matches!(self.current, Token::Identifier) {
            self.advance();
        } else {
            self.error("Expected identifier in declaration");
        }

        if matches!(self.current, Token::Semicolon) {
            self.advance();
        } else {
            self.error("Missing ';' in declaration");
        }
    }

    fn parse_statement(&mut self) {
        match self.current {
            Token::Identifier => self.parse_assignment(),
            Token::If => self.parse_if(),
            Token::For => self.parse_for(),
            _ => self.error("Invalid statement"),
        }
    }

    fn parse_assignment(&mut self) {
        self.advance();

        if matches!(self.current, Token::Assign) {
            self.advance();
            self.parse_expression();
        } else {
            self.error("Expected '=' in assignment");
        }

        if matches!(self.current, Token::Semicolon) {
            self.advance();
        } else {
            self.error("Missing ';' in assignment");
        }
    }

    fn parse_if(&mut self) {
        self.advance();

        if !matches!(self.current, Token::LParen) {
            self.error("Expected '(' after if");
        }
        self.advance();

        self.parse_expression();

        if !matches!(self.current, Token::RParen) {
            self.error("Expected ')'");
        }
        self.advance();

        if !matches!(self.current, Token::LBrace) {
            self.error("Expected '{'");
        }
        self.advance();

        while !matches!(self.current, Token::RBrace) {
            self.parse_statement();
        }
        self.advance();
    }

    fn parse_for(&mut self) {
        self.advance();

        if !matches!(self.current, Token::LParen) {
            self.error("Expected '(' after for");
        }
        self.advance();

        self.parse_declaration();
        self.parse_expression();

        if !matches!(self.current, Token::Semicolon) {
            self.error("Missing ';' in for");
        }
        self.advance();

        self.parse_assignment();

        if !matches!(self.current, Token::RParen) {
            self.error("Expected ')'");
        }
        self.advance();

        if !matches!(self.current, Token::LBrace) {
            self.error("Expected '{'");
        }
        self.advance();

        while !matches!(self.current, Token::RBrace) {
            self.parse_statement();
        }
        self.advance();
    }

    fn parse_expression(&mut self) {
        self.advance();

        while matches!(
            self.current,
            Token::Plus | Token::Minus | Token::Greater | Token::Less | Token::And | Token::Or
        ) {
            self.advance();
            self.advance();
        }
    }
}

//  TEST CASES

fn main() {
    std::panic::set_hook(Box::new(|_| {}));

    println!("Running SimpleLang Parser Tests...\n");

    test_valid_program();
    test_invalid_declaration();
    test_missing_brace();
    test_invalid_assignment();
    test_invalid_control();

    println!("\nAll tests executed.");
}

fn test_valid_program() {
    let tokens = vec![
        (Token::Int, 1),
        (Token::Identifier, 1),
        (Token::Semicolon, 1),
        (Token::Identifier, 2),
        (Token::Assign, 2),
        (Token::Number, 2),
        (Token::Semicolon, 2),
        (Token::If, 3),
        (Token::LParen, 3),
        (Token::Identifier, 3),
        (Token::Greater, 3),
        (Token::Number, 3),
        (Token::RParen, 3),
        (Token::LBrace, 3),
        (Token::Identifier, 4),
        (Token::Assign, 4),
        (Token::Identifier, 4),
        (Token::Minus, 4),
        (Token::Number, 4),
        (Token::Semicolon, 4),
        (Token::RBrace, 5),
        (Token::EOF, 6),
    ];

    let lexer = Lexer::new(tokens);
    let mut parser = Parser::new(lexer);
    parser.parse_program();

    println!("Test 1 Passed: Valid program");
}

fn test_invalid_declaration() {
    let tokens = vec![
        (Token::Int, 1),
        (Token::Semicolon, 1),
        (Token::EOF, 2),
    ];
    run_test(tokens, "Invalid declaration");
}

fn test_missing_brace() {
    let tokens = vec![
        (Token::If, 1),
        (Token::LParen, 1),
        (Token::Identifier, 1),
        (Token::Greater, 1),
        (Token::Number, 1),
        (Token::RParen, 1),
        (Token::LBrace, 1),
        (Token::Identifier, 2),
        (Token::Assign, 2),
        (Token::Number, 2),
        (Token::Semicolon, 2),
        (Token::EOF, 3),
    ];
    run_test(tokens, "Missing brace");
}

fn test_invalid_assignment() {
    let tokens = vec![
        (Token::Identifier, 1),
        (Token::Number, 1),
        (Token::Semicolon, 1),
        (Token::EOF, 2),
    ];
    run_test(tokens, "Invalid assignment");
}

fn test_invalid_control() {
    let tokens = vec![
        (Token::For, 1),
        (Token::Int, 1),
        (Token::Identifier, 1),
        (Token::Assign, 1),
        (Token::Number, 1),
        (Token::Semicolon, 1),
        (Token::EOF, 2),
    ];
    run_test(tokens, "Invalid control structure");
}

fn run_test(tokens: Vec<(Token, usize)>, name: &str) {
    let result = std::panic::catch_unwind(|| {
        let lexer = Lexer::new(tokens);
        let mut parser = Parser::new(lexer);
        parser.parse_program();
    });

    match result {
        Ok(_) => println!("{}: FAILED (error not detected)", name),
        Err(_) => println!("{}: Error correctly detected", name),
    }
}
