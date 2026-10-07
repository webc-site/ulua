use core::ops::AddAssign;

use crate::records::compile_stats::CompileStats;

impl AddAssign<&CompileStats> for CompileStats {
  fn add_assign(&mut self, other: &CompileStats) {
    self.lines += other.lines;
    self.bytecode += other.bytecode;
    self.bytecode_instruction_count += other.bytecode_instruction_count;
    self.codegen += other.codegen;
    self.read_time += other.read_time;
    self.misc_time += other.misc_time;
    self.parse_time += other.parse_time;
    self.compile_time += other.compile_time;
    self.codegen_time += other.codegen_time;
    // AddAssign<&LoweringStats> 内部逐字段累加，借走免 clone
    self.lower_stats += &other.lower_stats;
  }
}
