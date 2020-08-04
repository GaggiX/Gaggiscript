#[derive(PartialEq, Debug)]
pub enum Token {
    //special
    Illegal,
    EOF,

    //identifier + literals
    Number(u64),
    Identifier(String),

    //operators
    EqualSign,
    PlusSign,
    
    //delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    //keyword
    Function,
    Let
}

#[derive(Clone, Copy)]
pub struct Lexer<'b> {
    code:          &'b str,
    position:      usize,
    read_position: usize,
    ch:            u8
}

pub fn new<'b>(code: &'b str) -> Lexer {
    let mut lexer = Lexer{code, 
        position: 0, 
        read_position: 0, 
        ch: '\0' as u8};

    lexer.read_char();
    lexer
}

fn is_letter(chr: char) -> bool {
    'a' <= chr && chr <= 'z' || 
    'A' <= chr && chr <= 'z' || 
    chr == '_'
}

fn is_digit(chr: char) -> bool {
    '0' <= chr && chr <= '9' 
}

impl<'b> Lexer<'b> {

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec!(self.next_token());
        
        while *tokens.last().unwrap() != Token::EOF {
            tokens.push(self.next_token());
        }

        tokens
    }

    fn read_char(&mut self) {
        if self.read_position >= self.code.len() {
            self.ch = '\0' as u8;
        } else {
            self.ch = self.code.as_bytes()[self.read_position];
        }
        self.position = self.read_position;

        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let token: Token;

        self.skip_whitespace();

        match self.ch as char {
            '='  => token = Token::EqualSign,
            '+'  => token = Token::PlusSign,
            ','  => token = Token::Comma,
            ';'  => token = Token::Semicolon,
            '('  => token = Token::Lparen,
            ')'  => token = Token::Rparen,
            '{'  => token = Token::Lbrace,
            '}'  => token = Token::Rbrace,
            '\0' => token = Token::EOF,
            _    => {
                    if is_letter(self.ch as char) {
                        return self.read_identifier();
                    } else if is_digit(self.ch as char) {
                        return self.read_number();
                    } else {
                        token = Token::Illegal;
                    }
                }
        }

        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.ch as char == ' ' || self.ch as char == '\n' || self.ch as char == '\r' {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> Token {
        let position = self.position;
        while is_letter(self.ch as char) {
            self.read_char();
        }

        match &self.code[position..self.position] {
            "fn"  => Token::Function,
            "let" => Token::Let,
            _     => Token::Identifier(
                        String::from(&self.code[position..self.position])
                     )
        }
    }

    fn read_number(&mut self) -> Token {
        let position = self.position;

        while is_digit(self.ch as char) {
            self.read_char();
        }

        Token::Number(
            self.code[position..self.position]
                .parse()
                .unwrap()
        )
    }
}