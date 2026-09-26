use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition},
  functions::{
    builtin_check_cmp::{K_UINT32_WIDTH, builtin_check_int_const},
    builtin_linearop::{builtin_check_double, builtin_load_double},
    builtin_store_number_result::builtin_store_number_result,
    vm_reg_op::vm_reg_op,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_bit_32_replace(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 3 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let arg_reg = build.vm_reg(arg as u8);
  builtin_check_double(build, arg_reg, pcpos);
  builtin_check_double(build, args, pcpos);
  builtin_check_double(build, arg3, pcpos);

  let va = builtin_load_double(build, arg_reg);
  let vb = builtin_load_double(build, args);
  let vc = builtin_load_double(build, arg3);

  let n = build.inst_ir_cmd_ir_op(IrCmd::NumToUint, va);
  let v = build.inst_ir_cmd_ir_op(IrCmd::NumToUint, vb);
  let f = build.inst_ir_cmd_ir_op(IrCmd::NumToInt, vc);

  let value = if nparams == 3 {
    builtin_check_int_const(build, f, K_UINT32_WIDTH, IrCondition::UnsignedLess, pcpos);

    let m = build.const_int(1);
    let shift = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitlshiftUint, m, f);
    let not_ = build.inst_ir_cmd_ir_op(IrCmd::BitnotUint, shift);
    let lhs = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitandUint, n, not_);

    let vm = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitandUint, v, m);
    let rhs = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitlshiftUint, vm, f);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitorUint, lhs, rhs)
  } else {
    let reg = build.vm_reg((vm_reg_op(args) + 2) as u8);
    builtin_check_double(build, reg, pcpos);
    let vd = builtin_load_double(build, reg);

    let w = build.inst_ir_cmd_ir_op(IrCmd::NumToInt, vd);
    let fw = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddInt, f, w);

    builtin_check_int_const(build, f, 0, IrCondition::GreaterEqual, pcpos);
    builtin_check_int_const(build, w, 0, IrCondition::Greater, pcpos);
    builtin_check_int_const(build, fw, K_UINT32_WIDTH, IrCondition::LessEqual, pcpos);

    let one = build.const_int(1);
    let w_minus_1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubInt, w, one);
    let base = build.const_int(0xfffffffeu32 as i32);
    let shift1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitlshiftUint, base, w_minus_1);
    let m = build.inst_ir_cmd_ir_op(IrCmd::BitnotUint, shift1);

    let shift2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitlshiftUint, m, f);
    let not_ = build.inst_ir_cmd_ir_op(IrCmd::BitnotUint, shift2);
    let lhs = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitandUint, n, not_);

    let vm = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitandUint, v, m);
    let rhs = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitlshiftUint, vm, f);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitorUint, lhs, rhs)
  };

  let num = build.inst_ir_cmd_ir_op(IrCmd::UintToNum, value);
  builtin_store_number_result(build, ra, arg, num);

  BuiltinImplResult::FULL_ONE_RESULT
}
