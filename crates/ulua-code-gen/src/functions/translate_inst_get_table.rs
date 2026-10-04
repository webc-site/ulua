//! LOP_GETTABLE / LOP_SETTABLE 共享翻译骨架。
//!
//! cpp `IrBuilder.cpp` 中 translateInstGETTABLE/SETTABLE 是两份同型展开：两条指令
//! 仅在三处不同 —— 表命令方向（`cmd`：GetTable/SetTable）、SET 侧写前追加的只读
//! 检查（CheckReadonly）、读写搬移方向（GET 取数组槽 TValue 入 RAr；SET 取 RAr
//! 写回数组槽并补 BarrierTableForward）。其余逐指令同构，收敛到
//! [`translate_table_access`] 单源，`IrCmd::SetTable` 即 SETTABLE 路径。

use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType,
  fflag,
  macros::luau_insn_ops::{luau_insn_a, luau_insn_b, luau_insn_c},
};

use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    bytecode_types::is_userdata_bytecode_type, check_table_tag_guard::check_table_tag_guard,
    translate_inst_binary::check_number_tag_guard,
  },
  records::{fallback_stream_scope::FallbackStreamScope, ir_builder::IrBuilder},
  type_aliases::ir::Instruction,
};

pub fn translate_inst_get_table(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  translate_table_access(build, code, pcpos, IrCmd::GetTable)
}

/// GETTABLE/SETTABLE 共用主体：`cmd` 取 `GetTable`（读）/`SetTable`（写）。
pub(crate) fn translate_table_access(
  build: &mut IrBuilder,
  code: &[Instruction],
  pcpos: i32,
  cmd: IrCmd,
) {
  let is_set = cmd == IrCmd::SetTable;

  // 界内约定:  契约保证 code 切片于 pcpos 指向字节码缓冲内存活、对齐(Instruction=u32)的合法指令字，code[pcpos] 只读
  // 取出整条指令供 A/B/C 域提取，纯读无别名冲突。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let rb = luau_insn_b(insn) as u8;
  let rc = luau_insn_c(insn) as u8;

  let bc_types = build.function.get_bytecode_types_at(pcpos);

  if is_userdata_bytecode_type(bc_types.a)
    || bc_types.b == LuauBytecodeType::LBC_TYPE_STRING.0 as u8
  {
    let savedpc_arg = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let reg_rc = build.vm_reg(rc);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(cmd, reg_ra, reg_rb, reg_rc);
    return;
  }

  // 前置 Table 守卫需 fallback 值供后续 TryNumToIndex/CheckArraySize 等复用，保持 eager 创建。
  let mut fallback = build.fallback_block(pcpos as u32);

  // SET 专属：CheckArraySize 越界后的哈希直插块组（本 fork 扩展，cpp 无对应路径）。
  // rt 侧 nsieve/tablegrow 形负载中整数键表的数组段长期为空（等步长整数键
  // 达不到 computesizes 的 50% 吸收阈值），全部写落哈希段——仅内联数组段直写
  // 会让守卫链全付后仍每次回落 helper（sample 归因：写侧 77% 样本在
  // settable_num_fastpath/newkey/rehash）。此处对齐 SETTABLEKS J4b 的内联插入
  // 形态：主位空直插 / 等键覆写，其余（哨兵表、碰撞、rehash）仍落 helper。
  // GET 无插入语义，不建块（空块会破坏块序计算）。
  let hash_blocks = if is_set && fflag::LuauJitSettableHashInline.get() {
    Some((
      build.fallback_block(pcpos as u32),
      build.fallback_block(pcpos as u32),
    ))
  } else {
    None
  };

  let reg_rb = build.vm_reg(rb);
  let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rb);
  let a_is_table = bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8;
  check_table_tag_guard(build, tb, a_is_table, pcpos, fallback);

  // R93 守卫并网：双臂（b 侧 NUMBER → vm_exit，否则 fallback）与原全形态守卫同形。
  let c_is_number = bc_types.b == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8;
  check_number_tag_guard(build, rc, c_is_number, pcpos, &mut fallback);

  let reg_rc = build.vm_reg(rc);
  let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_rb);
  let vc = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_rc);

  let index = build.inst_ir_cmd_ir_op_ir_op(IrCmd::TryNumToIndex, vc, fallback);

  let const_int_one = build.const_int(1);
  let index = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubInt, index, const_int_one);

  // CheckArraySize 越界去向：SET 且旗标开时走哈希直插块（数组段外落哈希段），
  // 其余仍落 helper。主链守卫次序与原版逐位一致（golden IR 依赖）；元表/只读
  // 守卫由数组路与哈希直插路各自持有（每写恰好各查一次，无重复付）。
  let oob_target = match hash_blocks {
    Some((hash_block, _)) => hash_block,
    None => fallback,
  };
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, vb, index, oob_target);

  let reg_ra = build.vm_reg(ra);

  if is_set {
    // SET：写方向先挡掉只读表，再取数组槽写回；末尾补前向 barrier，防旧值所在表逃逸
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, vb, fallback);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, vb, fallback);
    let arr_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, vb, index);
    let tva = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, reg_ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, arr_el, tva);
    let undef = build.undef();
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BarrierTableForward, vb, reg_ra, undef);
  } else {
    // GET：读方向，数组槽 TValue 直入 RAr
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, vb, fallback);
    let arr_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, vb, index);
    let arr_el_tval = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, arr_el);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, arr_el_tval);
  }

  let next = build.block_at_inst((pcpos + 1) as u32);

  // 块布局（SET 带 hash_blocks 时，镜像 SETTABLEKS J4b 的发射序）：
  //   主路数组直写 → hash_block（直插）→ occupied_block（覆写检查）→ fallback（helper）→ next
  // GET（无 hash_blocks）保持原形态：主路 → fallback → next。
  let tail_scope_block = match hash_blocks {
    Some((hash_block, _)) => hash_block,
    None => fallback,
  };

  let scope = FallbackStreamScope::new(build, tail_scope_block, next);
  let build = &mut *scope.build;

  match hash_blocks {
    None => {
      // GET 原路径：helper 块即尾块
      let savedpc_arg = build.const_uint((pcpos + 1) as u32);
      build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
      let reg_ra = build.vm_reg(ra);
      let reg_rb = build.vm_reg(rb);
      let reg_rc = build.vm_reg(rc);
      build.inst_ir_cmd_ir_op_ir_op_ir_op(cmd, reg_ra, reg_rb, reg_rc);
      build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
    }
    Some((_, occupied_block)) => {
      // 哈希直插块：元表/只读守卫自含（与主链数组路同序同数，无重复付），主位空 →
      // 直插新键（setnodekey 数字版含 tmcache 作废）+ 值落位 + 前向屏障。键为数字
      // （非 collectable），键侧屏障空操作，不再补 BarrierTableForward（对齐 cpp
      // luaC_barriert 对非收集对象的零动作语义）。
      let reg_rb = build.vm_reg(rb);
      let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_rb);
      let reg_rc = build.vm_reg(rc);
      let node = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetHashNodeAddrNum, vb, reg_rc);

      build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, vb, fallback);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, vb, fallback);

      // 主位可插判定（哨兵表 node==dummynode 或 val 非空 → 覆写检查分支）
      build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckNodeInsertable, node, vb, occupied_block);

      build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreNodeKeyNum, node, reg_rc, vb);
      let reg_ra = build.vm_reg(ra);
      let tva = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, reg_ra);
      let offset = build.const_int(0);
      build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreTvalue, node, tva, offset);
      let undef = build.undef();
      build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BarrierTableForward, vb, reg_ra, undef);
      build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

      // 覆写检查分支：node 键为等值数字 → 直接覆写值（probe 命中口径）；
      // 否则（碰撞/他型键）→ helper 全路径。
      build.begin_block(occupied_block);

      let reg_rb = build.vm_reg(rb);
      let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_rb);
      let reg_rc = build.vm_reg(rc);
      let node = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetHashNodeAddrNum, vb, reg_rc);
      build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::JumpIfNodeKeyNotNum, node, reg_rc, fallback);

      let reg_ra = build.vm_reg(ra);
      let tva = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, reg_ra);
      let offset = build.const_int(0);
      build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreTvalue, node, tva, offset);
      let undef = build.undef();
      build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BarrierTableForward, vb, reg_ra, undef);
      build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

      // helper 块：SetSavedpc + 全路径 helper 调用（快路未吞的形态一律到此）
      build.begin_block(fallback);

      let savedpc_arg = build.const_uint((pcpos + 1) as u32);
      build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
      let reg_ra = build.vm_reg(ra);
      let reg_rb = build.vm_reg(rb);
      let reg_rc = build.vm_reg(rc);
      build.inst_ir_cmd_ir_op_ir_op_ir_op(cmd, reg_ra, reg_rb, reg_rc);
      build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
    }
  }
}
