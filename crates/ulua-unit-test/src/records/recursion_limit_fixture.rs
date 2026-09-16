use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;

// BytecodeBuilder 因 encoder Box<dyn> 不再 Clone，本 fixture 亦无需 Clone
#[derive(Debug)]
pub struct RecursionLimitFixture {
  pub bcb: BytecodeBuilder,
  pub reps: i32,
  pub find_limit: bool,
}
