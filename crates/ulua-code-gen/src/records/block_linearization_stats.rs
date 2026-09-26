use core::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct BlockLinearizationStats {
  pub const_prop_instruction_count: u32,
  pub time_seconds: f64,
}

impl BlockLinearizationStats {
  pub fn block_linearization_stats_operator_add(
    &self,
    other: &BlockLinearizationStats,
  ) -> BlockLinearizationStats {
    let mut result: BlockLinearizationStats = *self;
    result.block_linearization_stats_operator_add_assign(other);
    result
  }

  pub fn block_linearization_stats_operator_add_assign(
    &mut self,
    that: &BlockLinearizationStats,
  ) -> &mut Self {
    self.const_prop_instruction_count += that.const_prop_instruction_count;
    self.time_seconds += that.time_seconds;

    self
  }
}

impl Add for BlockLinearizationStats {
  type Output = Self;

  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    self.block_linearization_stats_operator_add(&rhs)
  }
}

impl Add<&BlockLinearizationStats> for BlockLinearizationStats {
  type Output = Self;

  #[inline]
  fn add(self, rhs: &BlockLinearizationStats) -> Self::Output {
    self.block_linearization_stats_operator_add(rhs)
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
