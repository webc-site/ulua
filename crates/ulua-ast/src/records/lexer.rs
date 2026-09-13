use alloc::vec::Vec;
use core::ffi::c_char;

use crate::{
  enums::brace_type::BraceType,
  records::{ast_name_table::AstNameTable, lexeme::Lexeme, location::Location},
};

#[derive(Debug, Clone)]
pub struct Lexer {
  pub(crate) buffer: *const c_char,
  pub(crate) buffer_size: usize,

  pub(crate) offset: u32,

  pub(crate) line: u32,
  pub(crate) line_offset: u32,

  pub(crate) lexeme: Lexeme,

  pub(crate) prev_location: Location,

  pub(crate) names: *mut AstNameTable,

  pub(crate) skip_comments: bool,
  pub(crate) read_names: bool,

  pub(crate) brace_stack: Vec<BraceType>,
}

impl Lexer {
  pub const BRACE_TYPE_INTERPOLATED_STRING: BraceType = BraceType::InterpolatedString;
  pub const BRACE_TYPE_NORMAL: BraceType = BraceType::Normal;
}

unsafe impl Send for Lexer {}
unsafe impl Sync for Lexer {}
