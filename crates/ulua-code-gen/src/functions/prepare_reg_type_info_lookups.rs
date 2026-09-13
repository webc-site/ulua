//! Node: `cxx:Function:Luau.CodeGen:CodeGen/src/BytecodeAnalysis.cpp:119:prepare_reg_type_info_lookups`
//! Source: `CodeGen/src/BytecodeAnalysis.cpp`
//! Graph edges:
//! - declared_by: source_file CodeGen/src/BytecodeAnalysis.cpp
//! - source_includes:
//!   - includes -> source_file CodeGen/include/Luau/BytecodeAnalysis.h
//!   - includes -> source_file CodeGen/include/Luau/CodeGenOptions.h
//!   - includes -> source_file CodeGen/include/Luau/IrData.h
//!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
//!   - includes -> source_file VM/src/lobject.h
//!   - includes -> source_file VM/src/lstate.h
//! - incoming:
//!   - declares <- source_file CodeGen/src/BytecodeAnalysis.cpp
//!   - calls <- function analyzeBytecodeTypes (CodeGen/src/BytecodeAnalysis.cpp)
//! - outgoing:
//!   - type_ref -> record BytecodeTypeInfo (CodeGen/include/Luau/IrData.h)
//!   - type_ref -> record BytecodeRegTypeInfo (CodeGen/include/Luau/IrData.h)
//!   - translates_to -> rust_item prepareRegTypeInfoLookups

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
