use alloc::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  ast_local::AstLocal, ast_name::AstName,
  fragment_parse_resume_settings::FragmentParseResumeSettings, position::Position,
};

impl FragmentParseResumeSettings {
  pub fn new(
    local_map: DenseHashMap<AstName, *mut AstLocal>,
    local_stack: Vec<*mut AstLocal>,
    resume_position: Position,
  ) -> Self {
    Self {
      local_map,
      local_stack,
      resume_position,
    }
  }
}
