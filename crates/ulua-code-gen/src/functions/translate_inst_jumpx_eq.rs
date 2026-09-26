use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{
    luau_insn_a::luau_insn_a, luau_insn_aux_kb::luau_insn_aux_kb,
    luau_insn_aux_kv::luau_insn_aux_kv, luau_insn_aux_not::luau_insn_aux_not,
    luau_insn_d::luau_insn_d,
  },
};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::proto_view::{constant_number, with_constant_value},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

/// 统一翻译 JUMPXEQ 族系指令 (KNIL, KB, KN, KS) 的常规分支路径
pub fn translate_inst_jumpx_eq(
  build: &mut IrBuilder,
  code: &[Instruction],
  pcpos: i32,
  op: LuauOpcode,
) {
  let pc_val = code[pcpos as usize];
  let ra = luau_insn_a(pc_val) as u8;
  let aux = code[pcpos as usize + 1];
  let not_ = luau_insn_aux_not(aux) != 0;

  let target = build.block_at_inst((pcpos + 1 + luau_insn_d(pc_val)) as u32);
  let next = build.block_at_inst((pcpos + 2) as u32);

  let vm_reg_ra = build.vm_reg(ra);
  let ta = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, vm_reg_ra);

  match op {
    LuauOpcode::LOP_JUMPXEQKNIL => {
      let const_nil_tag = build.const_tag(LuaType::Nil as u8);
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpEqTag,
        ta,
        const_nil_tag,
        if not_ { next } else { target },
        if not_ { target } else { next },
      );
    }
    LuauOpcode::LOP_JUMPXEQKB => {
      let check_value = build.block(IrBlockKind::Internal);
      let const_tag_boolean = build.const_tag(LuaType::Boolean as u8);
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpEqTag,
        ta,
        const_tag_boolean,
        check_value,
        if not_ { target } else { next },
      );

      build.begin_block(check_value);
      let vm_reg_ra = build.vm_reg(ra);
      let va = build.inst_ir_cmd_ir_op(IrCmd::LoadInt, vm_reg_ra);

      let const_int_kb = build.const_int(luau_insn_aux_kb(aux) as i32);
      let cond_equal = build.cond(IrCondition::Equal);
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpCmpInt,
        va,
        const_int_kb,
        cond_equal,
        if not_ { next } else { target },
        if not_ { target } else { next },
      );
    }
    LuauOpcode::LOP_JUMPXEQKN => {
      let check_value = build.block(IrBlockKind::Internal);
      let const_tag_number = build.const_tag(LuaType::Number as u8);
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpEqTag,
        ta,
        const_tag_number,
        check_value,
        if not_ { target } else { next },
      );

      build.begin_block(check_value);
      let vm_reg_ra = build.vm_reg(ra);
      let va = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, vm_reg_ra);

      CODEGEN_ASSERT!(!build.function.proto.is_null());
      let vb = with_constant_value(build.function.proto, luau_insn_aux_kv(aux) as u32, |tv| {
        let tt = tv.tt;
        CODEGEN_ASSERT!(tt == LuaType::Number as i32);
        build.const_double(constant_number(tv))
      })
      .expect("translate_inst_jumpx_eq_n: proto/k 非空且 aux_kv 界内(codegen 契约)");

      let cond = build.cond(IrCondition::NotEqual);
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpCmpNum,
        va,
        vb,
        cond,
        if not_ { target } else { next },
        if not_ { next } else { target },
      );
    }
    _ => {
      let check_value = build.block(IrBlockKind::Internal);
      let const_tag_string = build.const_tag(LuaType::String as u8);
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpEqTag,
        ta,
        const_tag_string,
        check_value,
        if not_ { target } else { next },
      );

      build.begin_block(check_value);
      let vm_reg_ra = build.vm_reg(ra);
      let va = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, vm_reg_ra);
      let vm_const_kv = build.vm_const(luau_insn_aux_kv(aux));
      let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, vm_const_kv);

      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpEqPointer,
        va,
        vb,
        if not_ { next } else { target },
        if not_ { target } else { next },
      );
    }
  }

  if build.is_internal_block(next) {
    build.begin_block(next);
  }
}

/// 统一翻译 JUMPXEQ 族系指令 (KNIL, KB, KN, KS) 的 direct_compare shortcut 快速路径
pub fn translate_inst_jumpx_eq_shortcut(
  build: &mut IrBuilder,
  code: &[Instruction],
  pcpos: i32,
  op: LuauOpcode,
) {
  let rr = luau_insn_a(code[pcpos as usize + 2]);
  let ra = luau_insn_a(code[pcpos as usize]);
  let aux = code[pcpos as usize + 1];
  let not_ = luau_insn_aux_not(aux) != 0;

  let next = build.block_at_inst((pcpos + 4) as u32);

  let vm_reg_ra = build.vm_reg(ra as u8);
  let ta = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, vm_reg_ra);

  let cond = if not_ {
    IrCondition::NotEqual
  } else {
    IrCondition::Equal
  };
  let cond_op = build.cond(cond);

  let result = match op {
    LuauOpcode::LOP_JUMPXEQKNIL => {
      let const_tag_nil = build.const_tag(LuaType::Nil as u8);
      build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CmpTag, ta, const_tag_nil, cond_op)
    }
    LuauOpcode::LOP_JUMPXEQKB => {
      let va = build.inst_ir_cmd_ir_op(IrCmd::LoadInt, vm_reg_ra);
      let vb = build.const_int(luau_insn_aux_kb(aux) as i32);
      let const_tag_boolean = build.const_tag(LuaType::Boolean as u8);
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CmpSplitTvalue,
        ta,
        const_tag_boolean,
        va,
        vb,
        cond_op,
      )
    }
    LuauOpcode::LOP_JUMPXEQKN => {
      let va = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, vm_reg_ra);
      CODEGEN_ASSERT!(!build.function.proto.is_null());
      let vb = with_constant_value(build.function.proto, luau_insn_aux_kv(aux) as u32, |tv| {
        let tt = tv.tt;
        let protok_value_n = constant_number(tv);
        CODEGEN_ASSERT!(tt == LuaType::Number as i32);
        build.const_double(protok_value_n)
      })
      .expect("translate_inst_jumpx_eq_n_shortcut: proto/k 非空且 aux_kv 界内(codegen 契约)");
      let const_tag_number = build.const_tag(LuaType::Number as u8);
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CmpSplitTvalue,
        ta,
        const_tag_number,
        va,
        vb,
        cond_op,
      )
    }
    _ => {
      let va = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, vm_reg_ra);
      let vm_const_kv = build.vm_const(luau_insn_aux_kv(aux) as u32);
      let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, vm_const_kv);
      let const_tag_string = build.const_tag(LuaType::String as u8);
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CmpSplitTvalue,
        ta,
        const_tag_string,
        va,
        vb,
        cond_op,
      )
    }
  };

  let vm_reg_rr = build.vm_reg(rr as u8);
  build.store_tag(vm_reg_rr, LuaType::Boolean as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt, vm_reg_rr, result);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

  if build.is_internal_block(next) {
    build.begin_block(next);
  }
}
