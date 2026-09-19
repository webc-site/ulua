use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;

// cpp RecursionLimitFixture 无拷贝语义，本 fixture 亦无需 Clone
// （BytecodeBuilder 的 encoder 已内联化，Clone 已可用）
#[derive(Debug)]
pub struct RecursionLimitFixture {
  pub bcb: BytecodeBuilder,
  pub reps: i32,
  pub find_limit: bool,
}
