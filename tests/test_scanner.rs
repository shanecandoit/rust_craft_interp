
use craft_interp;

// mod craft_interp::Scanner;
// mod craft_interp::Token;
// mod craft_interp::TokenType;

#[test]
fn test_scanner() {
    assert_eq!(true, true);
}

fn test_token() {
    let t1 = craft_interp::Token {
        token_type: craft_interp::TokenType::ParenLeft,
        lexeme: String::from("("),
        literal: None,
        line: 1,
    };
    let t2 = craft_interp::Token {
        token_type: craft_interp::TokenType::ParenRight,
        lexeme: String::from(")"),
        literal: None,
        line: 1,
    };

    assert_eq!(t1, t2);
}
