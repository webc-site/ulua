use ulua_vm::{enums::lua_type::LuaType, type_aliases::t_value::TValue};

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::vm_const_op::vm_const_op,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};
pub fn load_double_or_constant(build: &mut IrBuilder, arg: IrOp) -> IrOp {
  if arg.kind() == IrOpKind::VmConst {
    CODEGEN_ASSERT!(!build.function.proto.is_null());
    let protok_idx = vm_const_op(arg) as usize;
    let protok: TValue = unsafe { *(*build.function.proto).k.add(protok_idx) };
    CODEGEN_ASSERT!(protok.tt == LuaType::Number as i32);
    return build.const_double(unsafe { protok.value.n });
  }

  build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, arg)
}
