use core::ops::AddAssign;

use crate::records::block_linearization_stats::BlockLinearizationStats;

impl BlockLinearizationStats {
  pub fn block_linearization_stats_operator_add_assign(
    &mut self,
    that: &BlockLinearizationStats,
  ) -> &mut Self {
    self.const_prop_instruction_count += that.const_prop_instruction_count;
    self.time_seconds += that.time_seconds;

    self
  }
}

impl AddAssign<BlockLinearizationStats> for BlockLinearizationStats {
  #[inline]
  fn add_assign(&mut self, rhs: BlockLinearizationStats) {
    self.block_linearization_stats_operator_add_assign(&rhs);
  }
}

impl AddAssign<&BlockLinearizationStats> for BlockLinearizationStats {
  #[inline]
  fn add_assign(&mut self, rhs: &BlockLinearizationStats) {
    self.block_linearization_stats_operator_add_assign(rhs);
  }
}
