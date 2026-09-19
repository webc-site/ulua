use alloc::string::String;

use crate::{
  functions::is_space::is_space,
  records::{comment::Comment, hot_comment::HotComment, lexeme::Type, parser::Parser},
};

impl Parser {
  pub fn next_lexeme(&mut self) {
    let mut r#type = self.lexer.next_with(false, true).r#type;

    while r#type == Type::BROKEN_COMMENT || r#type == Type::COMMENT || r#type == Type::BLOCK_COMMENT
    {
      let lexeme = *self.lexer.current();

      if self.options.capture_comments {
        self.comment_locations.push(Comment {
          r#type: lexeme.r#type,
          location: lexeme.location,
        });
      }

      // Subtlety: Broken comments are weird because we record them as comments AND pass them to the parser as a lexeme.
      // The parser will turn this into a proper syntax error.
      if lexeme.r#type == Type::BROKEN_COMMENT {
        return;
      }

      // Comments starting with ! are called "hot comments" and contain directives for type checking / linting / compiling
      if lexeme.r#type == Type::COMMENT && lexeme.get_length() > 0 {
        // cpp `std::string(text + 1, text + end)` 是字节串；源缓冲不保证
        // UTF-8（`--!\xFF` 合法），因此按字节取值后再 lossy 解码为 String，
        // 绝不用 `from_utf8_unchecked` 把非法字节伪装成 `&str`。
        if let Some(text) = unsafe { lexeme.data_bytes() }.filter(|text| text[0] == b'!') {
          let mut end = text.len();
          while end > 0 && is_space(text[end - 1] as char) {
            end -= 1;
          }

          // 首字符 '!' 非空白，故 end >= 1，`text[1..end]` 恒为有效切片。
          self.hotcomments.push(HotComment {
            header: self.hotcomment_header,
            location: lexeme.location,
            content: String::from_utf8_lossy(&text[1..end]).into_owned(),
          });
        }
      }

      r#type = self.lexer.next_with(false, false).r#type;
    }
  }
}
