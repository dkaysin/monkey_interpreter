#[cfg(test)]
mod tests {
    use monkey_interpreter::lexer::*;

    #[test]
    fn test_is_letter() {
        assert!(is_letter(b'a'));
        assert!(is_letter(b's'));
        assert!(is_letter(b'z'));
        assert!(is_letter(b'A'));
        assert!(is_letter(b'T'));
        assert!(is_letter(b'Z'));
        assert!(is_letter(b'_'));
        assert!(!is_letter(b'1'));
        assert!(!is_letter(b'+'));
        assert!(!is_letter(b'='));
    }

    #[test]
    fn test_lexer_onechar() {
        let input_string = "=+(){},;";
        let mut lexer = Lexer::new(input_string.as_bytes());

        let tokens_truth = [
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
            Token::EOF,
        ];
        for token in tokens_truth.iter() {
            assert_eq!(lexer.next_token().token, *token)
        }
    }

    #[test]
    fn test_lexer_ident() {
        let input_string = "
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
            x + y;
            };
            let result = add(five, ten);
        ";
        let mut lexer = Lexer::new(input_string.as_bytes());

        let tokens_truth = [
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            //
            Token::LET,
            Token::IDENT("ten".to_string()),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,
            //
            Token::LET,
            Token::IDENT("add".to_string()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            //
            Token::LBRACE,
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            //
            Token::LET,
            Token::IDENT("result".to_string()),
            Token::ASSIGN,
            Token::IDENT("add".to_string()),
            Token::LPAREN,
            Token::IDENT("five".to_string()),
            Token::COMMA,
            Token::IDENT("ten".to_string()),
            Token::RPAREN,
            Token::SEMICOLON,
        ];
        for token in tokens_truth.iter() {
            assert_eq!(lexer.next_token().token, *token)
        }
    }

    #[test]
    fn test_lexer_additional() {
        let input_string = "
            !-/*5;
            5 < 10 > 5;
        ";
        let mut lexer = Lexer::new(input_string.as_bytes());

        let tokens_truth = [
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(5),
            Token::SEMICOLON,
            //
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::GT,
            Token::INT(5),
            Token::SEMICOLON,
        ];
        for token in tokens_truth.iter() {
            assert_eq!(lexer.next_token().token, *token)
        }
    }

    #[test]
    fn test_lexer_if_else() {
        let input_string = "
            if (5 < 10) {
                return true;
            } else {
                return false;
            }`
        ";
        let mut lexer = Lexer::new(input_string.as_bytes());

        let tokens_truth = [
            Token::IF,
            Token::LPAREN,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::RPAREN,
            //
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            //
            Token::ELSE,
            //
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
        ];
        for token in tokens_truth.iter() {
            assert_eq!(lexer.next_token().token, *token)
        }
    }

    #[test]
    fn test_lexer_two_chars() {
        let input_string = "
            x = 5;
            x == 5;
            x != 5;
            !x;
            }`
        ";
        let mut lexer = Lexer::new(input_string.as_bytes());

        let tokens_truth = [
            Token::IDENT("x".to_string()),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            //
            Token::IDENT("x".to_string()),
            Token::EQ,
            Token::INT(5),
            Token::SEMICOLON,
            //
            Token::IDENT("x".to_string()),
            Token::NEQ,
            Token::INT(5),
            Token::SEMICOLON,
            //
            Token::BANG,
            Token::IDENT("x".to_string()),
            Token::SEMICOLON,
        ];
        for token in tokens_truth.iter() {
            assert_eq!(lexer.next_token().token, *token)
        }
    }
}
