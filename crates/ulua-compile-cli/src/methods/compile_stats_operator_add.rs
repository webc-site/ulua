use core::ops::Add;

use crate::records::compile_stats::CompileStats;

impl CompileStats {
  pub fn compile_stats_operator_add(&self, other: &CompileStats) -> CompileStats {
    let mut result = self.clone();
    result.compile_stats_operator_add_assign(other);
    result
  }
}

impl Add<CompileStats> for CompileStats {
  type Output = CompileStats;

  fn add(self, other: CompileStats) -> Self::Output {
    self.compile_stats_operator_add(&other)
  }
}

impl Add<&CompileStats> for &CompileStats {
  type Output = CompileStats;

  fn add(self, other: &CompileStats) -> Self::Output {
    self.compile_stats_operator_add(other)
  }
}
