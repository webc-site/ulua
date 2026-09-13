use crate::{
  enums::ir_value_kind::{IrValueKind, K_VALUE_DWORD_SIZE},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{emit_common_x_64::K_SPILL_SLOTS, ir_reg_alloc_x_64::IrRegAllocX64},
};

impl IrRegAllocX64 {
  pub fn find_spill_stack_slot(&mut self, value_kind: IrValueKind) -> u32 {
    if value_kind == IrValueKind::Float || value_kind == IrValueKind::Int {
      for i in 0..(self.used_spill_slot_halfs.len() as u32 * 64) {
        let bit_index = i as usize;
        let word_index = bit_index / 64;
        let bit_offset = bit_index % 64;
        if self.used_spill_slot_halfs[word_index] & (1u64 << bit_offset) != 0 {
          continue;
        }

        return i;
      }
    } else {
      let _num_halves = K_VALUE_DWORD_SIZE[value_kind as usize];
      let _boundary = K_SPILL_SLOTS * 2;

      let max_start = self.used_spill_slot_halfs.len() as u32 * 64 - 3;

      let mut i = 0u32;
      while i < max_start {
        let bit_index = i as usize;
        let word_index = bit_index / 64;
        let bit_offset = bit_index % 64;

        if self.used_spill_slot_halfs[word_index] & (1u64 << bit_offset) != 0 {
          i += 2;
          continue;
        }

        let next_bit_offset = (bit_index + 1) % 64;
        let next_word_index = bit_index / 64 + (if bit_offset == 63 { 1 } else { 0 });
        if next_word_index < self.used_spill_slot_halfs.len()
          && self.used_spill_slot_halfs[next_word_index] & (1u64 << next_bit_offset) != 0
        {
          i += 2;
          continue;
        }

        if value_kind == IrValueKind::Tvalue {
          let bit_offset2 = (bit_index + 2) % 64;
          let word_index2 = bit_index / 64
            + (if bit_offset == 62 { 1 } else { 0 })
            + (if bit_offset == 63 { 1 } else { 0 });
          if word_index2 < self.used_spill_slot_halfs.len()
            && self.used_spill_slot_halfs[word_index2] & (1u64 << bit_offset2) != 0
          {
            i += 2;
            continue;
          }

          let bit_offset3 = (bit_index + 3) % 64;
          let word_index3 = bit_index / 64
            + (if bit_offset == 61 { 1 } else { 0 })
            + (if bit_offset == 62 { 1 } else { 0 })
            + (if bit_offset == 63 { 1 } else { 0 });
          if word_index3 < self.used_spill_slot_halfs.len()
            && self.used_spill_slot_halfs[word_index3] & (1u64 << bit_offset3) != 0
          {
            i += 2;
            continue;
          }
        }

        return i;
      }
    }

    CODEGEN_ASSERT!(false, "Nowhere to spill");
    !0u32
  }
}
