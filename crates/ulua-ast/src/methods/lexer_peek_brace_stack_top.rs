use crate::{enums::brace_type::BraceType, records::lexer::Lexer};

impl Lexer {
  pub fn peek_brace_stack_top(&self) -> Option<BraceType> {
    if self.brace_stack.is_empty() {
      None
    } else {
      self.brace_stack.last().copied()
    }
  }
}
