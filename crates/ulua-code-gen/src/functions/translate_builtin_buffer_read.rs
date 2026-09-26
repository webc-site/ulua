use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::translate_buffer_args_and_check_bounds::translate_buffer_args_and_check_bounds,
  records::{
    builtin_args::BuiltinArgs, builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder,
  },
};

pub fn translate_builtin_buffer_read(
  build: &mut IrBuilder,
  args: BuiltinArgs,
  read_cmd: IrCmd,
  size: i32,
  conv_cmd: IrCmd,
  store_cmd: IrCmd,
  store_tag: u8,
) -> BuiltinImplResult {
  if args.nparams < 2 || args.nresults > 1 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let (buf, int_index) = translate_buffer_args_and_check_bounds(build, args, size, false);

  let tag_buffer = build.const_tag(LuaType::Buffer as u8);
  let result = build.inst_ir_cmd_ir_op_ir_op_ir_op(read_cmd, buf, int_index, tag_buffer);

  let value_to_store = if conv_cmd == IrCmd::NOP {
    result
  } else {
    build.inst_ir_cmd_ir_op(conv_cmd, result)
  };

  let ra_vm_reg = build.vm_reg(args.ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(store_cmd, ra_vm_reg, value_to_store);
  build.store_tag(ra_vm_reg, store_tag);

  BuiltinImplResult::FULL_ONE_RESULT
}
