use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{
    builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd, ir_const_kind::IrConstKind,
    ir_op_kind::IrOpKind,
  },
  functions::vm_const_op::vm_const_op,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
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
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
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
    CODEGEN_ASSERT!(build.function.const_op(args).kind == IrConstKind::Double);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, setnum, args);
    let tag = build.const_tag(LuaType::Number as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, setnum, tag);
  } else {
    let va = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, args);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, setnum, va);

    CODEGEN_ASSERT!(!build.function.proto.is_null());
    let argstag = if args.kind() == IrOpKind::VmConst {
      let tag = unsafe {
        (*build.function.proto)
          .k
          .add(vm_const_op(args) as usize)
          .read()
          .tt
      };
      build.const_tag(tag as u8)
    } else {
      build.undef()
    };
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BarrierTableForward, table, args, argstag);
  }

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 0,
  }
}
