use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    proto_view::{constant_number, with_constant_value},
    vm_const_op::vm_const_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};
pub fn load_double_or_constant(build: &mut IrBuilder, arg: IrOp) -> IrOp {
  if arg.kind() == IrOpKind::VmConst {
    CODEGEN_ASSERT!(!build.function.proto.is_null());
    // `Proto::k[idx]` 的 TValue 与 Number 联合体臂散点解引用收口到 proto_view 门面（§2）：
    // 空 proto/空 k 短路为 None，生产不可达（入口断言 + IR 常量下标依 sizek 写定）。
    let number = with_constant_value(build.function.proto, vm_const_op(arg) as u32, |tv| {
      CODEGEN_ASSERT!(tv.tt == LuaType::Number as i32);
      constant_number(tv)
    });
    if let Some(value) = number {
      return build.const_double(value);
    }
  }

  build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, arg)
}
