//! Source: `CodeGen/src/BytecodeAnalysis.cpp`

use crate::records::bytecode_type_info::BytecodeTypeInfo;

pub fn prepare_reg_type_info_lookups(type_info: &mut BytecodeTypeInfo) {
  // 先按寄存器、再按 end PC 排序
  type_info.reg_types.sort_by(|a, b| {
    if a.reg != b.reg {
      a.reg.cmp(&b.reg)
    } else {
      a.endpc.cmp(&b.endpc)
    }
  });

  // 为所有寄存器准备数据，因为 'reg_types' 可能缺少临时寄存器
  type_info.reg_type_offsets.resize(256 + 1, 0);

  for (i, el) in type_info.reg_types.iter().enumerate() {
    // 数据按寄存器序排列，因此最后一次访问寄存器 Rn
    // 意味着 Rn+1 从 Rn 结束槽位的下一槽开始
    type_info.reg_type_offsets[el.reg as usize + 1] = (i + 1) as u32;
  }

  // 用前一个寄存器的偏移填补空洞
  for i in 1..type_info.reg_type_offsets.len() {
    if type_info.reg_type_offsets[i] == 0 {
      type_info.reg_type_offsets[i] = type_info.reg_type_offsets[i - 1];
    }
  }
}
