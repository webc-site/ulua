use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;

#[derive(Debug, Clone)]
pub struct RecursionLimitFixture {
  pub bcb: BytecodeBuilder,
  pub reps: i32,
  pub find_limit: bool,
}
