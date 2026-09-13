use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    builtin_check_int_64::builtin_check_int_64, builtin_load_int_64::builtin_load_int_64,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_int_64_extract(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 2 || nresults > 1 {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  builtin_check_int_64(build, vm_reg_arg, pcpos);
  builtin_check_int_64(build, args, pcpos);

  let n = builtin_load_int_64(build, vm_reg_arg);
  let f = builtin_load_int_64(build, args);

  let value: IrOp = if nparams == 2 {
    let const_0 = build.const_int_64(0);
    let cond_ge = build.cond(IrCondition::GreaterEqual);
    let vm_exit_pcpos = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::CheckCmpInt64,
      f,
      const_0,
      cond_ge,
      vm_exit_pcpos,
    );

    let const_63 = build.const_int_64(63);
    let cond_le = build.cond(IrCondition::LessEqual);
    let vm_exit_pcpos = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::CheckCmpInt64,
      f,
      const_63,
      cond_le,
      vm_exit_pcpos,
    );

    let shifted = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitrshiftInt64, n, f);
    let const_1 = build.const_int_64(1);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitandInt64, shifted, const_1)
  } else {
    builtin_check_int_64(build, arg3, pcpos);
    let w = builtin_load_int_64(build, arg3);
    let fw = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddInt64, f, w);

    let const_0 = build.const_int_64(0);
    let cond_ge = build.cond(IrCondition::GreaterEqual);
    let vm_exit_pcpos = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::CheckCmpInt64,
      f,
      const_0,
      cond_ge,
      vm_exit_pcpos,
    );

    let const_63 = build.const_int_64(63);
    let cond_le = build.cond(IrCondition::LessEqual);
    let vm_exit_pcpos = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::CheckCmpInt64,
      f,
      const_63,
      cond_le,
      vm_exit_pcpos,
    );

    let const_1 = build.const_int_64(1);
    let cond_ge = build.cond(IrCondition::GreaterEqual);
    let vm_exit_pcpos = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::CheckCmpInt64,
      w,
      const_1,
      cond_ge,
      vm_exit_pcpos,
    );

    let const_64 = build.const_int_64(64);
    let cond_le = build.cond(IrCondition::LessEqual);
    let vm_exit_pcpos = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::CheckCmpInt64,
      w,
      const_64,
      cond_le,
      vm_exit_pcpos,
    );

    let const_64 = build.const_int_64(64);
    let cond_le = build.cond(IrCondition::LessEqual);
    let vm_exit_pcpos = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::CheckCmpInt64,
      fw,
      const_64,
      cond_le,
      vm_exit_pcpos,
    );

    let const_64 = build.const_int_64(64);
    let shift_amount = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubInt64, const_64, w);
    let const_minus_1 = build.const_int_64(-1);
    let mask = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitrshiftInt64, const_minus_1, shift_amount);

    let shifted = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitrshiftInt64, n, f);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitandInt64, shifted, mask)
  };

  let vm_reg_ra = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreInt64, vm_reg_ra, value);
  let const_tag = build.const_tag(LuaType::Integer as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, vm_reg_ra, const_tag);

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
