use alloc::string::String;
use core::{slice::from_raw_parts, str::from_utf8_unchecked};

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
        unsafe {
          // 一次性建切片后全部用索引访问，取代 text_ptr.add(...) 裸指针算术；
          // 首字符为 '!'（非空白）保证 end >= 1，text[1..end] 恒有效。
          let text = from_raw_parts(lexeme.data.data as *const u8, lexeme.get_length() as usize);

          if text[0] == b'!' {
            let mut end = text.len();
            while end > 0 && is_space(text[end - 1] as char) {
              end -= 1;
            }

            let content = from_utf8_unchecked(&text[1..end]);

            self.hotcomments.push(HotComment {
              header: self.hotcomment_header,
              location: lexeme.location,
              content: String::from(content),
            });
          }
        }
      }

      r#type = self.lexer.next_with(false, false).r#type;
    }
  }
}
