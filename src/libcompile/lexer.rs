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

pub enum DelimToken {
  Paren,
  Bracket,
  Brace,
  NoDelim,
}

pub enum Token {
  Eq,
  Lt,
  Le, 
  EqEq,
  Ne,
  Ge,
  Gt,
  Ident,
  Literal,
  BinOp(BinOpToken),
  OpenDelim(DelimToken),
  CloseDelim(DelimToken),
  Let,
}

pub struct Lexer {
  source: String,
  current_char: u32,
}

impl Lexer {
  fn peek_char(&self) -> u8 {
    self.source[self.current_char + 1]
  }

  fn next_char(&mut self) -> u8 {
    let c = self.source[self.current_char];
    self.current_char += 1;

    c
  }

  pub fn next_token(&mut self, current: Token) -> Token {
    let c = self.next_char();
    match c {
      '[' => OpenDelim(Bracket),
      ']' => CloseDelim(Bracket),
      '{' => OpenDelim(Brace),
      '}' => CloseDelim(Brace),
      '+' => BinOp(Plus),
      '-' => BinOp(Minus),
      '*' => BinOp(Star),
      '/' => BinOp(Slash),
    }
  }
}