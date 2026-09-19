//! Source: `Analysis/include/Luau/Module.h`

// Module.h:36 — hand-ported; field set matches the C++ struct.
use alloc::{string::String, sync::Arc, vec::Vec};

use ulua_ast::{
  enums::mode,
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, ast_stat_block::AstStatBlock,
    comment::Comment, hot_comment::HotComment, parse_error::ParseError,
  },
};

use crate::{enums::type_file_resolver::Type, type_aliases::module_name_type::ModuleName};
#[derive(Debug, Clone)]
pub struct SourceModule {
  pub name: ModuleName, // Module identifier or a filename
  pub human_readable_name: String,
  pub r#type: Type,
  pub environment_name: Option<String>,
  pub cyclic: bool,
  pub allocator: Arc<Allocator>,
  pub names: Arc<AstNameTable>,
  pub parse_errors: Vec<ParseError>,
  pub root: *mut AstStatBlock,
  pub mode: Option<mode::Mode>,
  pub hotcomments: Vec<HotComment>,
  pub comment_locations: Vec<Comment>,
}

unsafe impl Send for SourceModule {}
unsafe impl Sync for SourceModule {}
