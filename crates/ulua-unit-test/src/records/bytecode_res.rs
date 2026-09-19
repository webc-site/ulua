use alloc::vec::Vec;
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
pub struct BytecodeRes {
  pub inlinee_bytecode: Vec<u8>,
  pub caller_bytecode: Vec<u8>,
  pub string_table: Vec<Vec<u8>>,
}
