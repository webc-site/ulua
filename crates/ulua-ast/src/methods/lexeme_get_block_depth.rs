use core::ffi::c_char;

use ulua_common::LUAU_ASSERT;

use crate::records::lexeme::{Lexeme, Type};

impl Lexeme {
  pub fn get_block_depth(&self) -> u32 {
    LUAU_ASSERT!(self.r#type == Type::RAW_STRING || self.r#type == Type::BLOCK_COMMENT);

    unsafe {
      let data_ptr = self.data.data;
      let length = self.length as usize;

      // If we have a well-formed string, we are guaranteed to see 2 `]` characters after the end of the string contents
      LUAU_ASSERT!(*data_ptr.add(length) == b']' as c_char);

      let mut depth: u32 = 0;
      loop {
        depth += 1;
        if *data_ptr.add(length + depth as usize) == b']' as c_char {
          break;
        }
      }

      depth - 1
    }
  }
}
