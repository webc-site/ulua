use alloc::vec::Vec;
use core::ops::Add;

use crate::records::{
  block_linearization_stats::BlockLinearizationStats, function_stats::FunctionStats,
};

#[derive(Debug, Clone, Default)]
pub struct LoweringStats {
  pub total_functions: u32,
  pub skipped_functions: u32,
  pub spills_to_slot: i32,
  pub spills_to_restore: i32,
  pub max_spill_slots_used: u32,
  pub blocks_pre_opt: u32,
  pub blocks_post_opt: u32,
  pub max_block_instructions: u32,
  pub reg_alloc_errors: i32,
  pub lowering_errors: i32,
  pub block_linearization_stats: BlockLinearizationStats,
  pub function_stats_flags: u32,
  pub functions: Vec<FunctionStats>,
}

impl LoweringStats {
  #[inline]
  pub fn lowering_stats_operator_add(&self, other: &LoweringStats) -> LoweringStats {
    let mut result: LoweringStats = self.clone();
    result.lowering_stats_operator_add_assign(other);
    result
  }
}

impl Add for LoweringStats {
  type Output = Self;

  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    self.lowering_stats_operator_add(&rhs)
  }
}

impl Add<&LoweringStats> for LoweringStats {
  type Output = Self;

  #[inline]
  fn add(self, rhs: &LoweringStats) -> Self::Output {
    self.lowering_stats_operator_add(rhs)
  }
}

impl Add<&LoweringStats> for &LoweringStats {
  type Output = LoweringStats;

  #[inline]
  fn add(self, rhs: &LoweringStats) -> Self::Output {
    self.lowering_stats_operator_add(rhs)
  }
}

pub const FUNCTION_STATS_ENABLE: u32 = 1 << 0;
