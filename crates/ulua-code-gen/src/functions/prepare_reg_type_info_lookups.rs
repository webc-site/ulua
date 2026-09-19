//! Source: `CodeGen/src/BytecodeAnalysis.cpp`

use crate::records::bytecode_type_info::BytecodeTypeInfo;

pub fn prepare_reg_type_info_lookups(type_info: &mut BytecodeTypeInfo) {
  // Sort by register first, then by end PC
  type_info.reg_types.sort_by(|a, b| {
    if a.reg != b.reg {
      a.reg.cmp(&b.reg)
    } else {
      a.endpc.cmp(&b.endpc)
    }
  });

  // Prepare data for all registers as 'reg_types' might be missing temporaries
  type_info.reg_type_offsets.resize(256 + 1, 0);

  for (i, el) in type_info.reg_types.iter().enumerate() {
    // Data is sorted by register order, so when we visit register Rn last time
    // it means that register Rn+1 starts one after the slot where Rn ends
    type_info.reg_type_offsets[el.reg as usize + 1] = (i + 1) as u32;
  }

  // Fill in holes with the offset of the previous register
  for i in 1..type_info.reg_type_offsets.len() {
    if type_info.reg_type_offsets[i] == 0 {
      type_info.reg_type_offsets[i] = type_info.reg_type_offsets[i - 1];
    }
  }
}
