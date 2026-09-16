use core::{cmp::max, ops::AddAssign};

use crate::records::lowering_stats::{FUNCTION_STATS_ENABLE, LoweringStats};

impl LoweringStats {
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
