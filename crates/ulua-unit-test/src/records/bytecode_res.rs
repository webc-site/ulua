use alloc::{string::String, vec::Vec};
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
pub struct BytecodeRes {
  pub inlinee_bytecode: String,
  pub caller_bytecode: String,
  pub string_table: Vec<String>,
}
