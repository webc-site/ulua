use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd},
  functions::{
    builtin_load_double::builtin_load_double, builtin_load_int_64::builtin_load_int_64,
    translate_buffer_args_and_check_bounds::translate_buffer_args_and_check_bounds,
  },
  records::{
    builtin_args::BuiltinArgs, builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder,
  },
};

pub fn translate_builtin_buffer_write(
  build: &mut IrBuilder,
  args: BuiltinArgs,
  write_cmd: IrCmd,
  size: i32,
  conv_cmd: IrCmd,
  load_int_64: bool,
) -> BuiltinImplResult {
  if args.nparams < 3 || args.nresults > 0 {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let (buf, int_index) = translate_buffer_args_and_check_bounds(build, args, size, load_int_64);

  let num_value = if load_int_64 {
    builtin_load_int_64(build, args.arg3)
  } else {
    builtin_load_double(build, args.arg3)
  };

  let value_to_write = if conv_cmd == IrCmd::NOP {
    num_value
  } else {
    build.inst_ir_cmd_ir_op(conv_cmd, num_value)
  };

  let tag_buffer = build.const_tag(LuaType::Buffer as u8);

  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(write_cmd, buf, int_index, value_to_write, tag_buffer);

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 0,
  }
}
