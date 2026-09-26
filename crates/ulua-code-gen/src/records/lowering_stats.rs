use alloc::vec::Vec;
use core::{
  cmp::max,
  ops::{Add, AddAssign},
};

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

  pub fn lowering_stats_operator_add_assign(&mut self, that: &LoweringStats) -> &mut LoweringStats {
    self.total_functions += that.total_functions;
    self.skipped_functions += that.skipped_functions;
    self.spills_to_slot += that.spills_to_slot;
    self.spills_to_restore += that.spills_to_restore;
    self.max_spill_slots_used = max(self.max_spill_slots_used, that.max_spill_slots_used);
    self.blocks_pre_opt += that.blocks_pre_opt;
    self.blocks_post_opt += that.blocks_post_opt;
    self.max_block_instructions = max(self.max_block_instructions, that.max_block_instructions);

    self.reg_alloc_errors += that.reg_alloc_errors;
    self.lowering_errors += that.lowering_errors;

    self
      .block_linearization_stats
      .block_linearization_stats_operator_add_assign(&that.block_linearization_stats);

    if (self.function_stats_flags & FUNCTION_STATS_ENABLE) != 0 {
      self.functions.extend(that.functions.iter().cloned());
    }

    self
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

impl AddAssign<&LoweringStats> for LoweringStats {
  #[inline]
  fn add_assign(&mut self, rhs: &LoweringStats) {
    self.lowering_stats_operator_add_assign(rhs);
  }
}

impl AddAssign<LoweringStats> for LoweringStats {
  #[inline]
  fn add_assign(&mut self, rhs: LoweringStats) {
    self.lowering_stats_operator_add_assign(&rhs);
  }
}
