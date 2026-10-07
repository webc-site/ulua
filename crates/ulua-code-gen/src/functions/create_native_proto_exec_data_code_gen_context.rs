use core::slice::from_raw_parts_mut;

use ulua_vm::records::proto::Proto;

use crate::{
  functions::create_native_proto_exec_data_native_proto_exec_data::create_native_proto_exec_data_u32_u32,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_builder::IrBuilder,
  type_aliases::native_proto_exec_data_ptr::{
    NativeProtoExecDataHeaderExt, NativeProtoExecDataPtr,
  },
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn create_native_proto_exec_data(
  proto: *mut Proto,
  ir: &IrBuilder,
  call_obs_sealed: bool,
) -> NativeProtoExecDataPtr {
  // Safety: 契约保证 proto 为存活 Proto，sizecode/bytecodeid 为同址只读快照，取一次后
  // 后续流程不再解引用该裸指针。
  let (sizecode, bytecode_id) = unsafe { ((*proto).sizecode as u32, (*proto).bytecodeid as u32) };

  // J1 Phase 2a：热点站点 TSFB 侧表（布局见 ulua-vm type_feedback::tsfb_bump 的读侧）：
  // [TSFB_MAGIC, nslots, (pc, state)×nslots——pc 升序]。观测写在 VM 侧 fallback helper。
  let tsfb = unsafe { build_tsfb_table(proto, sizecode) };

  // JIT call inlining 第 2 阶段：CALL 站点观测表（布局见 ulua-vm call_obs::locate_cobs）。
  // 暖重编译产物直接 sealed——观测已兑现为一版内联代码，不再重复触发。
  let cobs = unsafe { build_call_obs_table(proto, sizecode, call_obs_sealed) };

  // extra 总数含 TSFB 侧表与自描述尾字（表长）
  let extra_data_count =
    ir.function.extra_native_data.len() as u32 + tsfb.len() as u32 + cobs.len() as u32 + 2;
  let mut native_exec_data = create_native_proto_exec_data_u32_u32(sizecode, extra_data_count);

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

  // J1 Phase 2a：TSFB 侧表追加在既有 extra 之后，末字为表长（自描述尾，
  // VM 读侧经 header.extra_data_count 回溯定位，见 type_feedback::tsfb_bump）。
  // COBS 表紧随其后写入 extra 区最尾（观测期定位走前向扫，见 call_obs 模块注）。
  let tsfb_base = (sizecode + ir.function.extra_native_data.len() as u32) as usize;
  for (i, item) in tsfb.iter().enumerate() {
    data[tsfb_base + i] = *item;
  }
  data[tsfb_base + tsfb.len()] = tsfb.len() as u32;

  let cobs_base = tsfb_base + tsfb.len() + 1;
  for (i, item) in cobs.iter().enumerate() {
    data[cobs_base + i] = *item;
  }
  data[cobs_base + cobs.len()] = cobs.len() as u32;

  if sizecode > 0 {
    data[0] = 0;
  }

  // header 字段写入经 `header_mut` 门面（unsafe 收口见该 trait 契约）；写回结束后
  // 仅剩 native_exec_data 所有权移交。
  let header = native_exec_data.header_mut();
  header.entry_offset_or_address = inst_target as usize as *const u8;
  header.bytecode_id = bytecode_id;
  header.bytecode_instruction_count = sizecode;
  header.extra_data_count = extra_data_count; // 含 TSFB 侧表

  native_exec_data
}

/// J1 Phase 2a：扫描 proto 字节码，收集热点类别站点（升序 pc），产出 TSFB 侧表。
/// 布局（u32 单位）：`[TSFB_MAGIC, nslots, (pc, state)×nslots]`，state = hits<<8 | last_tag。
///
/// # Safety
/// `proto` 须为存活 Proto，`code[..sizecode]` 界内可读（与调用方既有前置一致）。
unsafe fn build_tsfb_table(proto: *mut Proto, sizecode: u32) -> Vec<u32> {
  use ulua_common::enums::luau_opcode::LuauOpcode;

  fn hot(op: u8) -> bool {
    use LuauOpcode::*;
    matches!(
      LuauOpcode::from(op),
      LopGettableks | LopSettableks | LopAdd | LopSub | LopMul | LopNamecall
    )
  }

  let mut sites: Vec<u32> = Vec::new();
  unsafe {
    let code = (*proto).code;
    for pc in 0..sizecode {
      let op = (*code.add(pc as usize) & 0xff) as u8;
      if hot(op) {
        sites.push(pc);
      }
    }
  }

  let mut out = Vec::with_capacity(2 + sites.len() * 2);
  out.push(0x5453_4642); // 'TSFB'
  out.push(sites.len() as u32);
  for pc in sites {
    out.push(pc);
    out.push(0); // state: hits<<8 | last_tag
  }
  out
}

/// JIT call inlining 第 2 阶段：CALL 站点观测表（`build_call_obs_table`）。
/// 布局（u32 单位）：`[COBS_MAGIC, ncalls, (pc, state, funid, proto_lo, proto_hi)
/// ×ncalls]`，槽位由 VM 侧 call_obs::call_obs_record 就地更新。
///
/// `warm`（暖重编译产物）时 state 初值置 sealed：观测已兑现为一版内联代码，
/// 重编译后重放观测只会产出同一版代码，不再触发。
///
/// # Safety
/// `proto` 须为存活 Proto，`code[..sizecode]` 界内可读（与调用方既有前置一致）。
unsafe fn build_call_obs_table(proto: *mut Proto, sizecode: u32, warm: bool) -> Vec<u32> {
  use ulua_common::enums::luau_opcode::LuauOpcode;

  // sealed 位与 ulua-vm call_obs 的 K_FLAG_SEALED 同值（布局单源在该模块注释）
  const K_FLAG_SEALED: u32 = 2;

  let mut sites: Vec<u32> = Vec::new();
  unsafe {
    let code = (*proto).code;
    for pc in 0..sizecode {
      let op = LuauOpcode::from((*code.add(pc as usize) & 0xff) as u8);
      if matches!(op, LuauOpcode::LopCall | LuauOpcode::LopCallfb) {
        sites.push(pc);
      }
    }
  }

  let mut out = Vec::with_capacity(2 + sites.len() * 5);
  out.push(0x434F_4253); // 'COBS'
  out.push(sites.len() as u32);
  for pc in sites {
    out.push(pc);
    out.push(if warm { K_FLAG_SEALED } else { 0 }); // state: hits<<8 | flags
    out.push(0); // funid
    out.push(0); // proto_lo
    out.push(0); // proto_hi
  }
  out
}
