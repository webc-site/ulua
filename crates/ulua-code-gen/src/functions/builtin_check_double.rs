use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_const_kind::IrConstKind, ir_op_kind::IrOpKind},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn builtin_check_double(build: &mut IrBuilder, arg: IrOp, pcpos: i32) {
  if arg.kind() == IrOpKind::Constant {
    CODEGEN_ASSERT!(build.function.const_op(arg).kind == IrConstKind::Double);
  } else {
    let exit = build.vm_exit(pcpos as u32);
    build.load_and_check_tag(arg, LuaType::Number as u8, exit);
  }
}
