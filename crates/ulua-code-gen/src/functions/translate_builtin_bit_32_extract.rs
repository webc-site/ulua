use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{
    builtin_impl_type::BuiltinImplType, ir_cmd::IrCmd, ir_condition::IrCondition,
    ir_op_kind::IrOpKind,
  },
  functions::{
    builtin_check_double::builtin_check_double, builtin_load_double::builtin_load_double,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{builtin_impl_result::BuiltinImplResult, ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_builtin_bit_32_extract(
  build: &mut IrBuilder,
  nparams: i32,
  ra: i32,
  arg: i32,
  args: IrOp,
  arg3: IrOp,
  nresults: i32,
  pcpos: i32,
) -> BuiltinImplResult {
  if nparams < 2 || nresults > 1 {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  if nparams == 2
    && args.kind() == IrOpKind::Constant
    && (build.function.double_op(args) as i32 as u32) >= 32
  {
    return BuiltinImplResult {
      r#type: BuiltinImplType::None,
      actual_result_count: -1,
    };
  }

  let arg_reg = build.vm_reg(arg as u8);
  builtin_check_double(build, arg_reg, pcpos);
  builtin_check_double(build, args, pcpos);

  let arg_reg = build.vm_reg(arg as u8);
  let va = builtin_load_double(build, arg_reg);
  let vb = builtin_load_double(build, args);

  let n = build.inst_ir_cmd_ir_op(IrCmd::NumToUint, va);
  let value = if nparams == 2 {
    if vb.kind() == IrOpKind::Constant {
      let f = build.function.double_op(vb) as i32;
      CODEGEN_ASSERT!((f as u32) < 32);

      let mut value = n;
      if f != 0 {
        let shift = build.const_int(f);
        value = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitrshiftUint, value, shift);
      }

      if f + 1 < 32 {
        let one = build.const_int(1);
        value = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitandUint, value, one);
      }
      value
    } else {
      let f = build.inst_ir_cmd_ir_op(IrCmd::NumToInt, vb);
      let c32 = build.const_int(32);
      let cond = build.cond(IrCondition::UnsignedLess);
      let exit = build.vm_exit(pcpos as u32);
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CheckCmpInt, f, c32, cond, exit);

      let shift = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitrshiftUint, n, f);
      let one = build.const_int(1);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitandUint, shift, one)
    }
  } else {
    let f = build.inst_ir_cmd_ir_op(IrCmd::NumToInt, vb);

    builtin_check_double(build, arg3, pcpos);
    let vc = builtin_load_double(build, arg3);

    let w = build.inst_ir_cmd_ir_op(IrCmd::NumToInt, vc);
    let fw = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddInt, f, w);

    let zero = build.const_int(0);
    let ge = build.cond(IrCondition::GreaterEqual);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CheckCmpInt, f, zero, ge, exit);

    let zero = build.const_int(0);
    let gt = build.cond(IrCondition::Greater);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CheckCmpInt, w, zero, gt, exit);

    let c32 = build.const_int(32);
    let le = build.cond(IrCondition::LessEqual);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CheckCmpInt, fw, c32, le, exit);

    let one = build.const_int(1);
    let w_minus_1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubInt, w, one);
    let base = build.const_int(0xfffffffeu32 as i32);
    let shift = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitlshiftUint, base, w_minus_1);
    let m = build.inst_ir_cmd_ir_op(IrCmd::BitnotUint, shift);

    let nf = build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitrshiftUint, n, f);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::BitandUint, nf, m)
  };

  let value = build.inst_ir_cmd_ir_op(IrCmd::UintToNum, value);
  let ra_reg = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, ra_reg, value);

  if ra != arg {
    let ra_reg = build.vm_reg(ra as u8);
    let tag = build.const_tag(LuaType::Number as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, ra_reg, tag);
  }

  BuiltinImplResult {
    r#type: BuiltinImplType::Full,
    actual_result_count: 1,
  }
}
