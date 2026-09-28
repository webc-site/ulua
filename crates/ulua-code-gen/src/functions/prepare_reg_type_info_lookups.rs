//! Source: `CodeGen/src/BytecodeAnalysis.cpp`

use crate::records::bytecode_analysis::BytecodeTypeInfo;

/// `reg_type_offsets` 表容量：u8 寄存器号（0..=255）共 `K_MAX_REG` 个，
/// 外加一个尾哨位（`reg + 1` 索引），故表长为 `K_MAX_REG + 1`。
const K_MAX_REG: usize = u8::MAX as usize + 1;

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
  type_info.reg_type_offsets.resize(K_MAX_REG + 1, 0);

  for (i, el) in type_info.reg_types.iter().enumerate() {
    // 数据按寄存器序排列，因此最后一次访问寄存器 Rn
    // 意味着 Rn+1 从 Rn 结束槽位的下一槽开始
    type_info.reg_type_offsets[el.reg as usize + 1] = (i + 1) as u32;
  }

  // 用前一个寄存器的偏移填补空洞：carry 即上一槽已填好的值，
  // 空槽（==0）继承 carry，非空槽刷新 carry（与原按序下标改写逐字节等价）。
  let offsets = &mut type_info.reg_type_offsets;
  let mut carry = offsets[0];
  for slot in offsets.iter_mut().skip(1) {
    if *slot == 0 {
      *slot = carry;
    } else {
      carry = *slot;
    }
  }
}
