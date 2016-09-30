use std::str::Chars;
use std::iter::Peekable;
use libcompile::ast;

#[derive(Clone, RustcEncodable, PartialEq, Eq, Hash, Debug, Copy)]
pub enum BinOpToken {
  Plus,
  Minus,
  Star,
  Slash,
  Percent,
  Caret,
  And,
  Or,
  Shl,
  Shr,
}

#[derive(Clone, RustcEncodable, PartialEq, Eq, Hash, Debug, Copy)]
pub enum DelimToken {
  Paren,
  Bracket,
  Brace,
  NoDelim,
}

#[derive(Clone, RustcEncodable, PartialEq, Eq, Hash, Debug, Copy)]
pub enum Lit {
  Integer(ast::Name),
}

#[derive(Clone, RustcEncodable, PartialEq, Eq, Hash, Debug, Copy)]
pub enum Token {
  Eq,
  Lt,
  Le, 
  EqEq,
  Ne,
  Ge,
  Gt,
  Ident(ast::Name),
  Literal(Lit),
  BinOp(BinOpToken),
  OpenDelim(DelimToken),
  CloseDelim(DelimToken),
  Semicolon,
  Let,
  Whitespace,
  EoF,
}

pub struct Lexer<'a> {
  source: Peekable<Chars<'a>>,
}

#[inline]
fn is_name(digit: char) -> bool {
  (digit.is_alphabetic() || digit.is_digit(10)) || digit == '='
}

impl <'a> Lexer<'a> {
  pub fn new(source: &'a String) -> Lexer<'a> {
    Lexer {
      source: source.chars().peekable(),
    }
  }

  fn peek_char(&mut self) -> Option<&char> {
    self.source.peek()
  }

  fn next_char(&mut self) -> Option<char> {
    self.source.next()
  }

  pub fn next_token(&mut self, current: Token) -> Token {
    let c = self.next_char();
    match c {
      Some('[') => Token::OpenDelim(DelimToken::Bracket),
      Some(']') => Token::CloseDelim(DelimToken::Bracket),
      Some('{') => Token::OpenDelim(DelimToken::Brace),
      Some('}') => Token::CloseDelim(DelimToken::Brace),
      Some('+') => Token::BinOp(BinOpToken::Plus),
      Some('-') => Token::BinOp(BinOpToken::Minus),
      Some('*') => Token::BinOp(BinOpToken::Star),
      Some('/') => Token::BinOp(BinOpToken::Slash),
      Some('=') => {
        if *self.peek_char().unwrap() == '=' {
          Token::EqEq
        } else {
          Token::Eq
        }
      },
      Some(';') => Token::Semicolon,
      Some(' ') | Some('\t') | Some('\n') => Token::Whitespace,
      Some(x) => {
        if x.is_digit(10) {
          self.read_num_lit(x)
        } else if is_name(x) {
          self.read_ident(x)
        } else {
          Token::EoF
        }
      },
      None => Token::EoF,
    }
  }

  fn read_ident(&mut self, current: char) -> Token {
    let mut name = String::new();
    name.push(current);
    while is_name(*self.peek_char().unwrap()) {
      name.push(self.next_char().unwrap());
    }

    if name == String::from("let") {
      Token::Let
    } else {
      Token::Ident(ast::Name(1))
    }
  }

  fn read_num_lit(&mut self, current: char) -> Token {
    let mut num = String::new();
    num.push(current);
    while self.peek_char().unwrap().is_digit(10) {
      num.push(self.next_char().unwrap());
    }

    Token::Literal(Lit::Integer(ast::Name(1)))
  }
}