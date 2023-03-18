// #[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    ParenLeft,
    ParenRight,
    BraceLeft,
    BraceRight,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    LiteralNumber(f64),
    LiteralString(String),
    LiteralIdentifier(String),

    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        println!("Scanner::new()");
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        println!("Scanner::scan_tokens()");
        let mut tokens = Vec::new();

        let chars = self.source.chars();

        for (i, c) in chars.enumerate() {
            println!("{}: {}", i, c);
        }
        tokens.push(Token::new(TokenType::EOF, String::from(""), None, 1));

        tokens
    }
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
