use crate::ast;
use crate::lexer::{Lexer, Token, TokenWithMeta};

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    cur_token: TokenWithMeta,
    peek_token: TokenWithMeta,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            cur_token,
            peek_token,
        }
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, Vec<String>> {
        let mut statements: Vec<ast::Statement> = Vec::new();
        let mut errors: Vec<String> = Vec::new();
        while self.cur_token.token != Token::EOF {
            match self.parse_statement() {
                Ok(stmt) => {
                    statements.push(stmt);
                    self.advance_tokens();
                }
                Err(err) => {
                    errors.push(err);
                    self.skip_statement();
                }
            };
        }
        if errors.len() == 0 {
            Ok(ast::Program { statements })
        } else {
            Err(errors)
        }
    }

    fn skip_statement(&mut self) {
        while self.cur_token.token != Token::SEMICOLON && self.cur_token.token != Token::EOF {
            self.advance_tokens();
        }
        self.advance_tokens();
    }

    fn advance_tokens(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn check_cur_token(&self, token: Token) -> Result<bool, String> {
        if self.cur_token.token == token {
            Ok(true)
        } else {
            Err(format_error(&format!("{}", token), &self.cur_token))
        }
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, String> {
        let stmt = match self.cur_token.token {
            Token::LET => self.parse_let(),
            Token::RETURN => self.parse_statement_return(),
            Token::IF => self.parse_statement_if(),
            _ => self.parse_statement_expression(),
        }?;
        self.advance_tokens();
        self.check_cur_token(Token::SEMICOLON)?;
        Ok(stmt)
    }

    fn parse_let(&mut self) -> Result<ast::Statement, String> {
        self.check_cur_token(Token::LET)?;

        self.advance_tokens();
        let ident = self.parse_identifier()?;

        self.advance_tokens();
        self.check_cur_token(Token::ASSIGN)?;

        self.advance_tokens();
        let expr = self.parse_expression()?;

        Ok(ast::Statement::Let(ident, expr))
    }

    fn parse_statement_return(&mut self) -> Result<ast::Statement, String> {
        self.check_cur_token(Token::RETURN)?;
        self.advance_tokens();
        let expr = self.parse_expression()?;
        Ok(ast::Statement::Return(expr))
    }

    fn parse_statement_expression(&mut self) -> Result<ast::Statement, String> {
        let expr = self.parse_expression()?;
        Ok(ast::Statement::Expression(expr))
    }

    fn parse_statement_if(&mut self) -> Result<ast::Statement, String> {
        self.check_cur_token(Token::IF)?;

        self.advance_tokens();
        self.check_cur_token(Token::LPAREN)?;
        self.advance_tokens();
        let expr = self.parse_expression()?;
        self.advance_tokens();
        self.check_cur_token(Token::RPAREN)?;

        self.advance_tokens();
        self.check_cur_token(Token::LBRACE)?;
        self.advance_tokens();
        let stmt_1 = self.parse_statement()?;
        self.advance_tokens();
        self.check_cur_token(Token::RBRACE)?;

        self.advance_tokens();
        self.check_cur_token(Token::ELSE)?;

        self.advance_tokens();
        self.check_cur_token(Token::LBRACE)?;
        self.advance_tokens();
        let stmt_2 = self.parse_statement()?;
        self.advance_tokens();
        self.check_cur_token(Token::RBRACE)?;

        Ok(ast::Statement::If(expr, Box::new(stmt_1), Box::new(stmt_2)))
    }

    fn parse_identifier(&mut self) -> Result<ast::Identifier, String> {
        match self.cur_token.token.clone() {
            Token::IDENT(name) => Ok(ast::Identifier { name }),
            _ => Err(format_error("IDENT", &self.cur_token)),
        }
    }

    fn parse_expression(&mut self) -> Result<ast::Expression, String> {
        let maybe_expr = match self.cur_token.token {
            Token::LPAREN => self.parse_expression_grouped(),
            _ => match self.peek_token.token {
                Token::PLUS | Token::MINUS => self.parse_expression_binary(),
                Token::SEMICOLON | Token::RPAREN => self.parse_expression_unary(),
                _ => Err(format_error("binary or unary expression", &self.cur_token)),
            },
        };
        maybe_expr
    }

    fn parse_expression_binary(&mut self) -> Result<ast::Expression, String> {
        let expr_1 = self.parse_expression_unary()?;

        self.advance_tokens();
        let operator = self.parse_operator()?;

        self.advance_tokens();
        let expr_2 = self.parse_expression()?;

        Ok(ast::Expression::Binary(
            operator,
            Box::new(expr_1),
            Box::new(expr_2),
        ))
    }

    fn parse_expression_unary(&mut self) -> Result<ast::Expression, String> {
        match self.cur_token.token {
            Token::INT(_) => self.parse_expression_int_literal(),
            Token::IDENT(_) => self.parse_expression_variable(),
            _ => Err(format_error("unary expression", &self.cur_token)),
        }
    }

    fn parse_expression_int_literal(&mut self) -> Result<ast::Expression, String> {
        match self.cur_token.token {
            Token::INT(number) => Ok(ast::Expression::IntLiteral(number)),
            _ => Err(format_error("INT", &self.cur_token)),
        }
    }

    fn parse_expression_variable(&mut self) -> Result<ast::Expression, String> {
        match self.cur_token.token.clone() {
            Token::IDENT(name) => Ok(ast::Expression::Variable(name)),
            _ => Err(format_error("IDENT", &self.cur_token)),
        }
    }

    fn parse_expression_grouped(&mut self) -> Result<ast::Expression, String> {
        self.check_cur_token(Token::LPAREN)?;

        self.advance_tokens();
        let expr = self.parse_expression()?;

        self.advance_tokens();
        self.check_cur_token(Token::RPAREN)?;

        Ok(ast::Expression::Grouped(Box::new(expr)))
    }

    fn parse_operator(&mut self) -> Result<ast::BinaryOperator, String> {
        match self.cur_token.token {
            Token::PLUS => Ok(ast::BinaryOperator::Add),
            Token::MINUS => Ok(ast::BinaryOperator::Subtract),
            _ => Err(format_error("operator", &self.cur_token)),
        }
    }
}

fn format_error(expected: &str, token: &TokenWithMeta) -> String {
    format!(
        "[{}:{}] expected {}, got {}",
        token.row_pos, token.col_pos, expected, token.token,
    )
}
