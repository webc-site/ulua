use alloc::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{ast_local::AstLocal, ast_name::AstName, position::Position};

#[derive(Debug, Clone)]
pub struct FragmentParseResumeSettings {
  pub(crate) local_map: DenseHashMap<AstName, *mut AstLocal>,
  pub(crate) local_stack: Vec<*mut AstLocal>,
  pub(crate) resume_position: Position,
}
