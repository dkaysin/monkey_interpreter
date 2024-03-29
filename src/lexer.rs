use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Special
    ILLEGAL,
    EOF,
    // Identifiers
    IDENT(String),
    INT(i32),
    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    // Comparison operators
    LT,
    GT,
    EQ,
    NEQ,
    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Token::ILLEGAL => "ILLEGAL".to_string(),
            Token::EOF => "EOF".to_string(),
            Token::IDENT(string) => format!("IDENT({})", string),
            Token::INT(number) => format!("INT({})", number),
            Token::ASSIGN => "ASSIGN".to_string(),
            Token::PLUS => "PLUS".to_string(),
            Token::MINUS => "MINUS".to_string(),
            Token::BANG => "BANG".to_string(),
            Token::ASTERISK => "ASTERISK".to_string(),
            Token::SLASH => "SLASH".to_string(),
            Token::LT => "LT".to_string(),
            Token::GT => "GT".to_string(),
            Token::EQ => "EQ".to_string(),
            Token::NEQ => "NEQ".to_string(),
            Token::COMMA => "COMMA".to_string(),
            Token::SEMICOLON => "SEMICOLON".to_string(),
            Token::LPAREN => "LPAREN".to_string(),
            Token::RPAREN => "RPAREN".to_string(),
            Token::LBRACE => "LBRACE".to_string(),
            Token::RBRACE => "RBRACE".to_string(),
            Token::FUNCTION => "FUNCTION".to_string(),
            Token::LET => "LET".to_string(),
            Token::TRUE => "TRUE".to_string(),
            Token::FALSE => "FALSE".to_string(),
            Token::IF => "IF".to_string(),
            Token::ELSE => "ELSE".to_string(),
            Token::RETURN => "RETURN".to_string(),
        };
        write!(f, "{string}")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TokenWithMeta {
    pub token: Token,
    pub pos: usize,
    pub row_pos: u32,
    pub col_pos: u32,
}

pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    read_position: usize,
    ch: u8,

    token_pos: usize,
    token_row_pos: u32,
    token_col_pos: u32,
    token_row_pos_delta: u32,
    token_col_pos_delta: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            ch: 0,

            token_pos: 0,
            token_row_pos: 0,
            token_col_pos: 0,
            token_row_pos_delta: 0,
            token_col_pos_delta: 0,
        }
    }

    pub fn read_char(&mut self) {
        let size = self.input.len();
        self.ch = if self.read_position >= size {
            0
        } else {
            self.input[self.read_position]
        };
        self.position = self.read_position;
        self.read_position += 1;
        self.token_col_pos_delta += 1;

        if is_newline(self.ch) {
            self.token_col_pos_delta = 0;
            self.token_row_pos_delta += 1;
        }
    }

    pub fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> TokenWithMeta {
        if self.position == 0 && self.read_position == 0 {
            self.read_char();
        }
        self.eat_whitespace();
        let token = match self.ch {
            ch if is_letter(ch) => match self.read_string() {
                "let" => Token::LET,
                "fn" => Token::FUNCTION,
                "true" => Token::TRUE,
                "false" => Token::FALSE,
                "if" => Token::IF,
                "else" => Token::ELSE,
                "return" => Token::RETURN,
                string => Token::IDENT(string.to_string()),
            },
            ch if is_digit(ch) => Token::INT(self.read_int()),
            _ => self.match_char(),
        };

        let token_pos = self.token_pos;
        let token_row_pos = self.token_row_pos;
        let token_col_pos = self.token_col_pos;
        self.token_pos = self.position;
        self.token_row_pos += self.token_row_pos_delta;
        self.token_col_pos += self.token_col_pos_delta;
        self.token_row_pos_delta = 0;
        self.token_col_pos_delta = 0;

        TokenWithMeta {
            token,
            pos: token_pos,
            row_pos: token_row_pos,
            col_pos: token_col_pos,
        }
    }

    fn eat_whitespace(&mut self) {
        while is_whitespace(self.ch) {
            self.read_char()
        }
    }

    fn read_string(&mut self) -> &'a str {
        let start = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        std::str::from_utf8(&self.input[start..self.position])
            .expect("failed to convert letter bytes to &str {err}")
    }

    fn read_int(&mut self) -> i32 {
        let start = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        let string = std::str::from_utf8(&self.input[start..self.position])
            .expect("failed to convert slice of digit bytes to string");
        string.parse().expect("failed to convert &str to i32")
    }

    fn match_char(&mut self) -> Token {
        let token = match self.ch {
            b'=' => match self.peek_char() {
                b'=' => {
                    self.read_char();
                    Token::EQ
                }
                _ => Token::ASSIGN,
            },
            b'!' => match self.peek_char() {
                b'=' => {
                    self.read_char();
                    Token::NEQ
                }
                _ => Token::BANG,
            },
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b',' => Token::COMMA,
            b'+' => Token::PLUS,
            b'-' => Token::MINUS,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b'*' => Token::ASTERISK,
            b'/' => Token::SLASH,
            b'<' => Token::LT,
            b'>' => Token::GT,
            0 => Token::EOF,
            _ => Token::ILLEGAL,
        };
        self.read_char();
        token
    }
}

pub fn is_letter(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_'
}

pub fn is_digit(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9'
}

pub fn is_whitespace(ch: u8) -> bool {
    ch == b' ' || ch == b'\t' || ch == b'\n' || ch == b'\r'
}

pub fn is_newline(ch: u8) -> bool {
    ch == b'\n'
}
