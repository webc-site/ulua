use alloc::string::String;

use crate::enums::type_file_resolver::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceCode {
  pub source: String,
  pub r#type: Type,
}

impl SourceCode {
  pub const NONE: Type = Type::None;
  pub const MODULE: Type = Type::Module;
  pub const SCRIPT: Type = Type::Script;
}
