use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    builtin_check_double::builtin_check_double, builtin_check_int_64::builtin_check_int_64,
    builtin_load_double::builtin_load_double,
  },
  records::{builtin_args::BuiltinArgs, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_buffer_args_and_check_bounds(
  build: &mut IrBuilder,
  args: BuiltinArgs,
  size: i32,
  load_int_64: bool,
) -> (IrOp, IrOp) {
  let loc = build.vm_reg(args.arg as u8);
  let fallback = build.vm_exit(args.pcpos as u32);
  build.load_and_check_tag(loc, LuaType::Buffer as u8, fallback);

  builtin_check_double(build, args.args, args.pcpos);

  if args.nparams == 3 && load_int_64 {
    builtin_check_int_64(build, args.arg3, args.pcpos);
  } else if args.nparams == 3 {
    builtin_check_double(build, args.arg3, args.pcpos);
  }

  let buf = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, loc);

  let num_index = builtin_load_double(build, args.args);
  let int_index = build.inst_ir_cmd_ir_op(IrCmd::NumToInt, num_index);

  let zero = build.const_int(0);
  let size_op = build.const_int(size);
  let undef_op = build.undef();
  let exit_op = build.vm_exit(args.pcpos as u32);

  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
    IrCmd::CheckBufferLen,
    buf,
    int_index,
    zero,
    size_op,
    undef_op,
    exit_op,
  );

  (buf, int_index)
}
