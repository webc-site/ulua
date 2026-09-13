//! Node: `cxx:Method:Luau.Analysis:Analysis/src/AstJsonEncoder.cpp:1519:ast_json_encoder_write_comments`
//! Source: `Analysis/src/AstJsonEncoder.cpp` (AstJsonEncoder.cpp:1519-1552, hand-ported)

use ulua_ast::records::{comment::Comment, lexeme::Type};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_comments(&mut self, comment_locations: Vec<Comment>) {
    let mut comment_comma = false;
    for comment in comment_locations {
      if comment_comma {
        self.write_raw_string_view(",");
      } else {
        comment_comma = true;
      }
      self.write_raw_string_view("{");
      let c = self.push_comma();
      match comment.r#type {
        Type::COMMENT => self.write_type_string_view("Comment"),
        Type::BLOCK_COMMENT => self.write_type_string_view("BlockComment"),
        Type::BROKEN_COMMENT => self.write_type_string_view("BrokenComment"),
        _ => {}
      }
      self.write("location", &comment.location);
      self.pop_comma(c);
      self.write_raw_string_view("}");
    }
  }
}
