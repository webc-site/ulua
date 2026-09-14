use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd},
  functions::{
    builtin_check_double::builtin_check_double, builtin_load_double::builtin_load_double,
    vm_reg_op::vm_reg_op,
  },
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_math_min_max(
  build: &mut IrBuilder,
  cmd: IrCmd,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  const K_MIN_MAX_UNROLLED_PARAMS: i32 = 16;

  if !(2..=K_MIN_MAX_UNROLLED_PARAMS).contains(&nparams) || nresults > 1 {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let arg_reg = build.vm_reg(arg as u8);
  builtin_check_double(build, arg_reg, pcpos);
  builtin_check_double(build, args, pcpos);

  if nparams >= 3 {
    builtin_check_double(build, arg3, pcpos);
  }

  let args_base = vm_reg_op(args);
  for i in 4..=nparams {
    let reg = build.vm_reg((args_base + (i - 2)) as u8);
    builtin_check_double(build, reg, pcpos);
  }

  let varg1 = builtin_load_double(build, arg_reg);
  let varg2 = builtin_load_double(build, args);

  let mut res = build.inst_ir_cmd_ir_op_ir_op(cmd, varg2, varg1);

  if nparams >= 3 {
    let arg = builtin_load_double(build, arg3);
    res = build.inst_ir_cmd_ir_op_ir_op(cmd, arg, res);
  }

  for i in 4..=nparams {
    let reg = build.vm_reg((args_base + (i - 2)) as u8);
    let arg = builtin_load_double(build, reg);
    res = build.inst_ir_cmd_ir_op_ir_op(cmd, arg, res);
  }

  let ra_reg = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, ra_reg, res);

  if ra != arg {
    let tag_number = build.const_tag(3);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, tag_number);
  }

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
