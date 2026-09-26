use crate::{
  enums::{int_64_binary::Int64Binary, ir_cmd::IrCmd},
  functions::{
    builtin_check_cmp::builtin_check_non_zero_int_64,
    builtin_linearop::{builtin_check_int_64, builtin_load_int_64, builtin_store_int_64},
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_int_64_binary(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  nresults: i32,
  pcpos: i32,
  op: Int64Binary,
) -> BuiltinImplResult {
  if nparams < 2 || nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let vm_reg_arg = build.vm_reg(arg as u8);
  builtin_check_int_64(build, vm_reg_arg, pcpos);
  builtin_check_int_64(build, args, pcpos);

  let va = builtin_load_int_64(build, vm_reg_arg);
  let vb = builtin_load_int_64(build, args);

  let bin_op = match op {
    Int64Binary::Add => build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddInt64, va, vb),
    Int64Binary::Sub => build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubInt64, va, vb),
    Int64Binary::Mul => build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulInt64, va, vb),
    Int64Binary::Div => {
      let exit = build.vm_exit(pcpos as u32);
      build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckDivInt64, va, vb, exit);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::DivInt64, va, vb)
    }
    Int64Binary::Idiv => {
      let exit = build.vm_exit(pcpos as u32);
      build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckDivInt64, va, vb, exit);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::IdivInt64, va, vb)
    }
    Int64Binary::Udiv => {
      builtin_check_non_zero_int_64(build, vb, pcpos);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::UdivInt64, va, vb)
    }
    Int64Binary::Rem => {
      builtin_check_non_zero_int_64(build, vb, pcpos);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::RemInt64, va, vb)
    }
    Int64Binary::Urem => {
      builtin_check_non_zero_int_64(build, vb, pcpos);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::UremInt64, va, vb)
    }
    Int64Binary::Mod => {
      builtin_check_non_zero_int_64(build, vb, pcpos);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::ModInt64, va, vb)
    }
  };

  builtin_store_int_64(build, ra, bin_op);

  BuiltinImplResult::FULL_ONE_RESULT
}
