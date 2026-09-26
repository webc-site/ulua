use ulua_code_gen::records::lowering_stats::LoweringStats;

#[derive(Debug, Clone, Default)]
pub(crate) struct CompileStats {
  pub lines: usize,
  pub bytecode: usize,
  pub bytecode_instruction_count: usize,
  pub codegen: usize,

  pub read_time: f64,
  pub misc_time: f64,
  pub parse_time: f64,
  pub compile_time: f64,
  pub codegen_time: f64,

  pub lower_stats: LoweringStats,
}
