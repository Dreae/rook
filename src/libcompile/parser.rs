use libcompile::lexer::{Lexer, Token};

pub struct Parser<'a> {
  lexer: Box<Lexer<'a>>,
  line_num: u32,
  current_token: Token,
  previous_token: Token,
}

impl <'a> Parser<'a> {
  pub fn new(source: &'a String) -> Parser<'a> {
    Parser {
      lexer: Box::new(Lexer::new(source)),
      line_num: 0,
      current_token: Token::EoF,
      previous_token: Token::EoF,
    }
  }

  pub fn parse(&mut self) {
    let mut token = self.lexer.next_token(self.current_token);
    while token != Token::EoF {
      println!("Got token {:?}", token);
      token = self.lexer.next_token(token);
    }
  }
}