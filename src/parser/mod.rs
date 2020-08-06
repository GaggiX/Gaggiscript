mod ast;
use ast::{Program, Statement, Expression};
use crate::lexer::{Token};
use std::process::exit;

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,
    Lessgreater,
    Sum,
    Product,
    Prefix,
    Call
}

fn get_precedence(token: &Token) -> Precedence {
    match token {
        Token::EQ           => Precedence::Equals,
        Token::NotEQ        => Precedence::Equals,
        Token::LT           => Precedence::Lessgreater,
        Token::GT           => Precedence::Lessgreater,
        Token::PlusSign     => Precedence::Sum,
        Token::MinusSign    => Precedence::Sum,
        Token::SlashSign    => Precedence::Product,
        Token::AsteriskSign => Precedence::Product,
        Token::Lparen       => Precedence::Call,
        _                   => Precedence::Lowest
    }
}

pub struct Parser<'a> {
    tokens:     &'a Vec<Token<'a>>,
    cur_token:  usize,
    peek_token: usize,
}

pub fn new<'a>(tokens: &'a Vec<Token<'a>>) -> Parser<'a> {
    let mut parser = Parser{
        tokens,
        cur_token:  0,
        peek_token: 0
    };

    parser.next_token();
    parser
}

fn err(err: &str) -> String {
    String::from(err)
}

impl<'a> Parser<'a> {

    fn next_token(&mut self) {
        self.cur_token = self.peek_token;
        self.peek_token += 1;
    }

    pub fn parse_program(&mut self) -> Program<'a> {
        let mut program = Program{statements: vec!()};

        while self.tokens[self.cur_token] != Token::EOF {
            program.statements.push(
                self.parse_statement()
                    .unwrap_or_else(|err| {eprintln!("Parser error: {}", err); exit(1)})
            );

            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Result<Statement<'a>, String> {
        Ok(match self.tokens[self.cur_token] {
            Token::Let    => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _             => self.parse_expression_statement(),
        }?)
    }

    fn parse_let_statement(&mut self) -> Result<Statement<'a>, String> {
        
        let name: &str; 
        if let Token::Identifier(i) = self.tokens[self.peek_token] {
            name = i;
            self.next_token();
        } else {
            return Err(err("Expected identifier"));
        }

        if !self.expect_token(Token::EqualSign) {return Err(err("Expected identifier"))}

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(Token::Semicolon) {self.next_token()}

        Ok(Statement::LetStatement(name, value))
    }

    fn parse_return_statement(&mut self) -> Result<Statement<'a>, String> {
        self.next_token();

        
        let return_value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(Token::Semicolon) {self.next_token()}

        Ok(Statement::ReturnStatement(return_value))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement<'a>, String> {

        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }

        Ok(Statement::ExpressionStatement(expression))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression<'a>, String> {
        let mut left_exp = match self.tokens[self.cur_token] {
            Token::Identifier(i)       => Ok(Expression::Ident(i)),
            Token::Number(i)           => Ok(Expression::Int(i)),
            Token::True | Token::False => Ok(Expression::Bool(self.cur_token_is(Token::True))),
            Token::BangSign 
            | Token::MinusSign         => Ok(self.parse_prefix_expression()?),
            Token::Lparen              => Ok(self.parse_grouped_expression()?),
            Token::If                  => Ok(self.parse_if_expression()?),
            Token::Function            => Ok(self.parse_function_literal()?),
            _                          => Err(err("Expected expression"))
        };

        while !self.peek_token_is(Token::Semicolon) && precedence < self.peek_precedence() {
            self.next_token();

            left_exp = match self.tokens[self.cur_token] {
                Token::PlusSign       
                | Token::MinusSign   
                | Token::SlashSign   
                | Token::AsteriskSign
                | Token::EQ          
                | Token::NotEQ       
                | Token::LT          
                | Token::GT           => self.parse_infix_expression(left_exp?),
                Token::Lparen         => self.parse_call_expression(left_exp?),
                _                     => left_exp
            }
        }

        left_exp
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression<'a>, String> {
        let token = &self.tokens[self.cur_token];

        self.next_token();

        let right = self.parse_expression(Precedence::Prefix)?;

        Ok(Expression::PrefixExpression(token, Box::new(right)))
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression<'a>, String> {
        self.next_token();

        let exp = self.parse_expression(Precedence::Lowest);

        if self.expect_token(Token::Rparen) {exp} else {Err(err("Expected right parenthesis"))}
    }

    fn parse_if_expression(&mut self) -> Result<Expression<'a>, String> {
        if !self.expect_token(Token::Lparen) {return Err(err("Expected left parenthesis"))}
        
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_token(Token::Rparen) {return Err(err("Expected right parenthesis"))} 
        if !self.expect_token(Token::Lbrace) {return Err(err("Expected left brace"))}

        let consequence = self.parse_block_statement()?;
        
        let alternative = if self.peek_token_is(Token::Else) {
            self.next_token();

            if !self.expect_token(Token::Lbrace) {return Err(err("Expected left brace"))}

            Some(self.parse_block_statement()?)
        } else {None};

        Ok(Expression::IfExpression(Box::new(condition), consequence, alternative))
    }

    fn parse_block_statement(&mut self) -> Result<Vec<Statement<'a>>, String> {
        let mut block = Vec::new();
        
        self.next_token();

        while !self.cur_token_is(Token::Rbrace) && !self.cur_token_is(Token::EOF) {
            block.push(self.parse_statement()?);
            self.next_token();
        };

        Ok(block)
    }

    fn parse_function_literal(&mut self) -> Result<Expression<'a>, String> {

        if !self.expect_token(Token::Lparen) {return Err(err("Expected left parenthesis"))}

        let parameters = self.parse_function_parameters()?;

        if !self.expect_token(Token::Lbrace) {return Err(err("Expected left brace"))}

        Ok(Expression::FunctionLiteral(parameters, self.parse_block_statement()?))
    }

    fn parse_function_parameters(&mut self) -> Result<Option<Vec<&'a str>>, String> {
        if self.peek_token_is(Token::Rparen) {self.next_token(); Ok(None)}
        else {
            self.next_token();
            let mut identifier = Vec::new();
            if let Token::Identifier(i) = self.tokens[self.cur_token] {
                identifier.push(i);
            }

            while self.peek_token_is(Token::Comma) {
                self.next_token(); self.next_token();

                if let Token::Identifier(i) = self.tokens[self.cur_token] {
                    identifier.push(i);
                }
            }

            if !self.expect_token(Token::Rparen) {return Err(err("Expected right parenthesis"))}

            Ok(Some(identifier))
        }
    }

    fn parse_infix_expression(&mut self, left: Expression<'a>) -> Result<Expression<'a>, String> {
        let token = &self.tokens[self.cur_token];
        let precedence = self.cur_precedence();

        self.next_token();

        let right = self.parse_expression(precedence)?;

        Ok(Expression::InfixExpression(Box::new(left), token, Box::new(right)))
    }

    fn parse_call_expression(&mut self, left: Expression<'a>) -> Result<Expression<'a>, String> {
        Ok(Expression::CallExpression(Box::new(left), self.parse_call_arguments()?))
    }

    fn parse_call_arguments(&mut self) -> Result<Option<Vec<Expression<'a>>>, String> {
        if self.peek_token_is(Token::Rparen) {self.next_token(); Ok(None)}
        else {
            self.next_token();
            let mut args = vec!(self.parse_expression(Precedence::Lowest)?);

            while self.peek_token_is(Token::Comma) {
                self.next_token(); self.next_token();

                args.push(self.parse_expression(Precedence::Lowest)?)
            }

            if !self.expect_token(Token::Rparen) {return Err(err("expected right parenthesis"))}

            Ok(Some(args))
        }
    }

    fn peek_precedence(&self) -> Precedence {
        get_precedence(&self.tokens[self.peek_token])
    }

    fn cur_precedence(&self) -> Precedence {
        get_precedence(&self.tokens[self.cur_token])
    }

    fn cur_token_is(&self, token: Token) -> bool {
        self.tokens[self.cur_token] == token
    }

    fn peek_token_is(&self, token: Token) -> bool {
        self.tokens[self.peek_token] == token
    }

    fn expect_token(&mut self, token: Token) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            true
        } else {
            false
        }
    }
}