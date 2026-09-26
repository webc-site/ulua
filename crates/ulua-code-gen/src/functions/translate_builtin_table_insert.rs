use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{proto_view::with_constant_value, vm_const_op::vm_const_op},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_const::IrConst, ir_op::IrOp,
  },
};

pub fn translate_builtin_table_insert(
  build: &mut IrBuilder,
  nparams: i32,
  _ra: i32,
  arg: i32,
  args: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams != 2 || nresults > 0 {
    return BuiltinImplResult::NONE_FALLBACK;
  }

  let arg_reg = build.vm_reg(arg as u8);
  let exit = build.vm_exit(pcpos as u32);
  build.load_and_check_tag(arg_reg, LuaType::Table as u8, exit);

  let arg_reg = build.vm_reg(arg as u8);
  let table = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, arg_reg);
  let exit = build.vm_exit(pcpos as u32);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, table, exit);

  let len = build.inst_ir_cmd_ir_op(IrCmd::TableLen, table);
  let one = build.const_int(1);
  let pos = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddInt, len, one);
  let setnum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::TableSetnum, table, pos);

  if args.kind() == IrOpKind::Constant {
    CODEGEN_ASSERT!(matches!(build.function.const_op(args), IrConst::Double(_)));
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, setnum, args);
    build.store_tag(setnum, LuaType::Number as u8);
  } else {
    let va = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, args);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, setnum, va);

    CODEGEN_ASSERT!(!build.function.proto.is_null());
    let argstag = if args.kind() == IrOpKind::VmConst {
      // Proto::k 直读收口进 `with_constant_value` 门面(§2): 只读 tt, 再按原位 `const_tag` 内部化。
      let tag = with_constant_value(build.function.proto, vm_const_op(args) as u32, |tv| tv.tt)
        .expect("translate_builtin_table_insert: proto/k 非空且 vm_const 界内(codegen 契约)");
      build.const_tag(tag as u8)
    } else {
      build.undef()
    };
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BarrierTableForward, table, args, argstag);
  }

  BuiltinImplResult::new(BuiltinImplType::Full, 0)
}
