use core::ops::AddAssign;

use crate::records::compile_stats::CompileStats;

impl CompileStats {
  pub fn compile_stats_operator_add_assign(&mut self, other: &CompileStats) -> &mut Self {
    self.lines += other.lines;
    self.bytecode += other.bytecode;
    self.bytecode_instruction_count += other.bytecode_instruction_count;
    self.codegen += other.codegen;
    self.read_time += other.read_time;
    self.misc_time += other.misc_time;
    self.parse_time += other.parse_time;
    self.compile_time += other.compile_time;
    self.codegen_time += other.codegen_time;
    self.lower_stats += other.lower_stats.clone();
    self
  }
}

impl AddAssign<&CompileStats> for CompileStats {
  fn add_assign(&mut self, other: &CompileStats) {
    self.compile_stats_operator_add_assign(other);
  }
}
