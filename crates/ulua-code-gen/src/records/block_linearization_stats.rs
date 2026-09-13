use core::ops::Add;
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct BlockLinearizationStats {
  pub(crate) const_prop_instruction_count: u32,
  pub(crate) time_seconds: f64,
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

impl Default for BlockLinearizationStats {
  fn default() -> Self {
    Self {
      const_prop_instruction_count: 0,
      time_seconds: 0.0,
    }
  }
}
