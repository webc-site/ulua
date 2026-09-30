use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType,
  macros::luau_insn_ops::{luau_insn_a, luau_insn_b},
};
use ulua_vm::enums::{lua_type::LuaType, tms::TMS};

use crate::{
  enums::{host_metamethod::HostMetamethod, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    bytecode_types::is_userdata_bytecode_type, emit::x_64::check_tag_exit,
    translate_inst_binary::check_number_tag_guard,
  },
  records::{fallback_stream_scope::FallbackStreamScope, ir_builder::IrBuilder, ir_op::IrOp},
  type_aliases::ir::Instruction,
};

/// DO_ARITH 取负展开（userdata 直发与 fallback 流两处共用）：SetSavedpc 后发 `ra = -rb` 的 DoArith。
fn emit_do_arith_unm(build: &mut IrBuilder, ra: u8, rb: u8, pcpos: i32) {
  let savedpc_arg = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
  let reg_ra = build.vm_reg(ra);
  let reg_rb = build.vm_reg(rb);
  let const_int_unm = build.const_int(TMS::TmUnm as i32);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::DoArith, reg_ra, reg_rb, reg_rb, const_int_unm);
}
pub fn translate_inst_minus(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  let bc_types = build.function.get_bytecode_types_at(pcpos);

  // Safety: pc 指向 proto.code 内一条 MINUS 指令主字(译码器保证在 sizecode 界内、Instruction=u32 对齐);
  // 只读取出该字供 A/B 域提取, 纯读无别名冲突, 取代原先对 *pc 的两次重复读取。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let rb = luau_insn_b(insn) as u8;

  if bc_types.a == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8 {
    let reg_rb = build.vm_reg(rb);
    let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rb);
    check_tag_exit(build, tb, LuaType::Vector as u8, pcpos);

    let reg_rb = build.vm_reg(rb);
    let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, reg_rb);
    let va = build.inst_ir_cmd_ir_op(IrCmd::UnmVec, vb);
    let va = build.inst_ir_cmd_ir_op(IrCmd::TagVector, va);
    let reg_ra = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, va);
    return;
  }

  if is_userdata_bytecode_type(bc_types.a) {
    // cpp IrTranslation.cpp:1037-1041：先尝试宿主 userdata 元方法展开，未接管再走 DO_ARITH
    // 视图重建与安全论证单源于 `IrBuilder::host_hooks_ref`。
    let host_hooks = build.host_hooks_ref();
    if let Some(userdata_metamethod) = host_hooks.userdata_metamethod {
      let lhs = build.vm_reg(rb) as IrOp;
      if userdata_metamethod(
        build,
        bc_types.a,
        bc_types.b,
        ra as i32,
        lhs,
        IrOp::default(),
        HostMetamethod::Minus,
        pcpos,
      ) {
        return;
      }
    }

    emit_do_arith_unm(build, ra, rb, pcpos);
    return;
  }

  let mut fallback = IrOp::new();
  let a_is_number = bc_types.a == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8;

  check_number_tag_guard(build, rb, a_is_number, pcpos, &mut fallback);

  let reg_rb = build.vm_reg(rb);
  let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_rb);
  let va = build.inst_ir_cmd_ir_op(IrCmd::UnmNum, vb);

  let reg_ra = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, reg_ra, va);

  if ra != rb {
    let reg_ra = build.vm_reg(ra);
    build.store_tag(reg_ra, LuaType::Number as u8);
  }

  if fallback.kind() != IrOpKind::None {
    let next = build.block_at_inst((pcpos + 1) as u32);
    let scope = FallbackStreamScope::new(build, fallback, next);
    let build = &mut *scope.build;

    emit_do_arith_unm(build, ra, rb, pcpos);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
  }
}
