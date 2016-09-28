use std::string::String;
use lexer::Lexer;

struct Parser {
  lexer: &Lexer,
  line_num: u32,
  current_token: Token,
  previous_token: Token,
}

