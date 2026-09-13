use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{const_prop_state::ConstPropState, numbered_instruction::NumberedInstruction},
};

impl ConstPropState {
  pub fn get_max_internal_overlap(&mut self, set: &mut [NumberedInstruction], slot: usize) -> i32 {
    // Start with one live range for the slot we want to reuse
    let mut curr = 1;

    // For any slots where lifetime began before the slot of interest, mark as live if lifetime end is still active
    // This saves us from processing slots [0; slot] in the range sweep later, which requires sorting the lifetime end points
    for i in 0..slot {
      if set[i].finish_pos >= set[slot].start_pos {
        curr += 1;
      }
    }

    let mut max = curr;

    // Collect lifetime end points and sort them
    self.range_end_temp.clear();

    for item in set.iter().skip(slot + 1) {
      self.range_end_temp.push(item.finish_pos);
    }

    self.range_end_temp.sort_unstable();

    // Go over the lifetime begin/end ranges that we store as separate array and walk based on the smallest of values
    let mut i1 = slot + 1;
    let mut i2 = 0usize;

    while i1 < set.len() && i2 < self.range_end_temp.len() {
      if self.range_end_temp[i2] == set[i1].start_pos {
        i1 += 1;
        i2 += 1;
      } else if self.range_end_temp[i2] < set[i1].start_pos {
        CODEGEN_ASSERT!(curr > 0);
        curr -= 1;
        i2 += 1;
      } else {
        curr += 1;
        i1 += 1;

        if curr > max {
          max = curr;
        }
      }
    }

    // We might have unprocessed lifetime end entries, but we will never have unprocessed lifetime start entries
    // Not that lifetime end entries can only decrease the current value and do not affect the end result (maximum)
    max
  }
}
