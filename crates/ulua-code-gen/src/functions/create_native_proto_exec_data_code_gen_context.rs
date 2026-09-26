use core::slice::from_raw_parts_mut;

use ulua_vm::records::proto::Proto;

use crate::{
  functions::{
    create_native_proto_exec_data_native_proto_exec_data::create_native_proto_exec_data_u32_u32,
    get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_builder::IrBuilder,
  type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn create_native_proto_exec_data(
  proto: *mut Proto,
  ir: &IrBuilder,
) -> NativeProtoExecDataPtr {
  // Safety: 契约保证 proto 为存活 Proto，sizecode/bytecodeid 为同址只读快照，取一次后
  // 后续流程不再解引用该裸指针。
  let (sizecode, bytecode_id) = unsafe { ((*proto).sizecode as u32, (*proto).bytecodeid as u32) };

  let extra_data_count = ir.function.extra_native_data.len() as u32;
  let native_exec_data = create_native_proto_exec_data_u32_u32(sizecode, extra_data_count);

  let inst_target = ir.function.entry_location;
  let unassigned_offset = ir.function.end_location - inst_target;

  // Safety: 新分配的数据区恰为 sizecode+extra_data_count 个 u32，切片覆盖分配尺寸、
  // 对齐一致；借用只存活到 header 写回之前，与 header 区域（数据区之外的另一段）不交叠。
  let data = unsafe {
    from_raw_parts_mut(
      native_exec_data.as_ptr(),
      (sizecode + extra_data_count) as usize,
    )
  };

  for (i, item) in ir
    .function
    .bc_mapping
    .iter()
    .take(sizecode as usize)
    .enumerate()
  {
    let bc_mapping = *item;

    CODEGEN_ASSERT!(bc_mapping.asm_location >= inst_target);

    data[i] = if bc_mapping.asm_location != !0u32 {
      bc_mapping.asm_location - inst_target
    } else {
      unassigned_offset
    };
  }

  for (i, item) in ir
    .function
    .extra_native_data
    .iter()
    .enumerate()
    .take(extra_data_count as usize)
  {
    data[sizecode as usize + i] = *item;
  }

  if sizecode > 0 {
    data[0] = 0;
  }

  // Safety: header 与数据区同址相邻（由分配布局反推），字段写入对齐一致；重建的 &mut
  // 随 header 写回结束，之后仅剩 native_exec_data 所有权移交。
  unsafe {
    let header = &mut *get_native_proto_exec_data_header_mut(native_exec_data.as_ptr());
    header.entry_offset_or_address = inst_target as usize as *const u8;
    header.bytecode_id = bytecode_id;
    header.bytecode_instruction_count = sizecode;
    header.extra_data_count = extra_data_count;
  }

  native_exec_data
}
