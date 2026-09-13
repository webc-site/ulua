use ulua_code_gen::records::lowering_stats::LoweringStats;

#[derive(Debug, Clone)]
pub struct CompileStats {
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

impl Default for CompileStats {
  fn default() -> Self {
    Self {
      lines: 0,
      bytecode: 0,
      bytecode_instruction_count: 0,
      codegen: 0,
      read_time: 0.0,
      misc_time: 0.0,
      parse_time: 0.0,
      compile_time: 0.0,
      codegen_time: 0.0,
      lower_stats: LoweringStats {
        total_functions: 0,
        skipped_functions: 0,
        spills_to_slot: 0,
        spills_to_restore: 0,
        max_spill_slots_used: 0,
        blocks_pre_opt: 0,
        blocks_post_opt: 0,
        max_block_instructions: 0,
        reg_alloc_errors: 0,
        lowering_errors: 0,
        block_linearization_stats: Default::default(),
        function_stats_flags: 0,
        functions: Vec::new(),
      },
    }
  }
}
