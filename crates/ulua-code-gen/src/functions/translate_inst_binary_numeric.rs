use ulua_common::enums::{
  luau_builtin_function::LuauBuiltinFunction, luau_bytecode_type::LuauBytecodeType,
};
use ulua_vm::enums::{lua_type::LuaType, tms::TMS};

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    get_initialized_fallback::get_initialized_fallback,
    is_userdata_bytecode_type::is_userdata_bytecode_type,
    load_double_or_constant::load_double_or_constant,
    translate_binary_numeric_fallback_if_required::translate_binary_numeric_fallback_if_required,
    vm_const_op::vm_const_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn translate_inst_binary_numeric(
  build: &mut IrBuilder,
  ra: i32,
  rb: i32,
  rc: i32,
  opb: IrOp,
  opc: IrOp,
  pcpos: i32,
  tm: TMS,
) {
  let mut fallback = IrOp::new();

  let bc_types = build.function.get_bytecode_types_at(pcpos);

  // Special fast-paths for vectors, matching the cases we have in VM
  if bc_types.a == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8
    && bc_types.b == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8
    && (tm == TMS::TmAdd
      || tm == TMS::TmSub
      || tm == TMS::TmMul
      || tm == TMS::TmDiv
      || tm == TMS::TmIDiv)
  {
    let reg_rb = build.vm_reg(rb as u8);
    let tag_b = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rb);
    let vector_tag = build.const_tag(LuaType::Vector as u8);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag_b, vector_tag, exit);

    let reg_rc = build.vm_reg(rc as u8);
    let tag_c = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rc);
    let vector_tag = build.const_tag(LuaType::Vector as u8);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag_c, vector_tag, exit);

    let vb = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, opb, IrOp::new());

    let vc = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, opc, IrOp::new());

    let result = match tm {
      TMS::TmAdd => build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddVec, vb, vc),
      TMS::TmSub => build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubVec, vb, vc),
      TMS::TmMul => build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulVec, vb, vc),
      TMS::TmDiv => build.inst_ir_cmd_ir_op_ir_op(IrCmd::DivVec, vb, vc),
      TMS::TmIDiv => build.inst_ir_cmd_ir_op_ir_op(IrCmd::IdivVec, vb, vc),
      _ => {
        CODEGEN_ASSERT!(false);
        IrOp::new()
      }
    };

    let result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::TagVector, result, IrOp::new());
    let reg_ra = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, result);
    return;
  } else if !is_userdata_bytecode_type(bc_types.a)
    && bc_types.b == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8
    && (tm == TMS::TmMul || tm == TMS::TmDiv || tm == TMS::TmIDiv)
  {
    if rb != -1 {
      let fallback_exit = if bc_types.a == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8 {
        build.vm_exit(pcpos as u32)
      } else {
        get_initialized_fallback(build, &mut fallback, pcpos)
      };

      let rb_reg = build.vm_reg(rb as u8);
      let tag_load = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, rb_reg);
      let number_tag = build.const_tag(LuaType::Number as u8);
      build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag_load, number_tag, fallback_exit);
    }

    let reg_rc = build.vm_reg(rc as u8);
    let tag_rc = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rc);
    let vector_tag = build.const_tag(LuaType::Vector as u8);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag_rc, vector_tag, exit);

    let load_d = load_double_or_constant(build, opb);
    let undef = IrOp::new();
    let num_float = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NumToFloat, load_d, undef);
    let vb = build.inst_ir_cmd_ir_op_ir_op(IrCmd::FloatToVec, num_float, IrOp::new());

    let vc = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, opc);
    let result = match tm {
      TMS::TmMul => build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulVec, vb, vc),
      TMS::TmDiv => build.inst_ir_cmd_ir_op_ir_op(IrCmd::DivVec, vb, vc),
      TMS::TmIDiv => build.inst_ir_cmd_ir_op_ir_op(IrCmd::IdivVec, vb, vc),
      _ => {
        CODEGEN_ASSERT!(false);
        IrOp::new()
      }
    };

    let result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::TagVector, result, IrOp::new());
    let reg_ra = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, result);

    translate_binary_numeric_fallback_if_required(build, fallback, ra, opb, opc, tm, pcpos);
    return;
  } else if bc_types.a == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8
    && !is_userdata_bytecode_type(bc_types.b)
    && (tm == TMS::TmMul || tm == TMS::TmDiv || tm == TMS::TmIDiv)
  {
    let reg_rb = build.vm_reg(rb as u8);
    let tag_rb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rb);
    let vector_tag = build.const_tag(LuaType::Vector as u8);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag_rb, vector_tag, exit);

    if rc != -1 {
      let fallback_exit = if bc_types.b == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8 {
        build.vm_exit(pcpos as u32)
      } else {
        get_initialized_fallback(build, &mut fallback, pcpos)
      };

      let rc_reg = build.vm_reg(rc as u8);
      let tag_load = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, rc_reg);
      let number_tag = build.const_tag(LuaType::Number as u8);
      build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag_load, number_tag, fallback_exit);
    }

    let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, opb);
    let load_d = load_double_or_constant(build, opc);
    let num_float = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NumToFloat, load_d, IrOp::new());
    let vc = build.inst_ir_cmd_ir_op_ir_op(IrCmd::FloatToVec, num_float, IrOp::new());

    let result = match tm {
      TMS::TmMul => build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulVec, vb, vc),
      TMS::TmDiv => build.inst_ir_cmd_ir_op_ir_op(IrCmd::DivVec, vb, vc),
      TMS::TmIDiv => build.inst_ir_cmd_ir_op_ir_op(IrCmd::IdivVec, vb, vc),
      _ => {
        CODEGEN_ASSERT!(false);
        IrOp::new()
      }
    };

    let result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::TagVector, result, IrOp::new());
    let reg_ra = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, result);

    translate_binary_numeric_fallback_if_required(build, fallback, ra, opb, opc, tm, pcpos);
    return;
  }

  if is_userdata_bytecode_type(bc_types.a) || is_userdata_bytecode_type(bc_types.b) {
    let savedpc = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc);
    let reg_ra = build.vm_reg(ra as u8);
    let tm_op = build.const_int(tm as i32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::DoArith, reg_ra, opb, opc, tm_op);
    return;
  }

  // fast-path: number
  if rb != -1 {
    let reg_rb = build.vm_reg(rb as u8);
    let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rb);
    let exit_or_fallback = if bc_types.a == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8 {
      build.vm_exit(pcpos as u32)
    } else {
      get_initialized_fallback(build, &mut fallback, pcpos)
    };
    let number_tag = build.const_tag(LuaType::Number as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb, number_tag, exit_or_fallback);
  }

  if rc != -1 && rc != rb {
    let reg_rc = build.vm_reg(rc as u8);
    let tc = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rc);
    let exit_or_fallback = if bc_types.b == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8 {
      build.vm_exit(pcpos as u32)
    } else {
      get_initialized_fallback(build, &mut fallback, pcpos)
    };
    let number_tag = build.const_tag(LuaType::Number as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tc, number_tag, exit_or_fallback);
  }

  let vb = load_double_or_constant(build, opb);
  let mut vc = IrOp::new();
  let mut result = IrOp::new();

  if opc.kind() == IrOpKind::VmConst {
    let protok_index = vm_const_op(opc);
    CODEGEN_ASSERT!(!build.function.proto.is_null());
    let protok = unsafe { *(*build.function.proto).k.add(protok_index as usize) };
    CODEGEN_ASSERT!(protok.tt == LuaType::Number as i32);

    let n = unsafe { protok.value.n };
    if tm == TMS::TmPow && n == 0.5 {
      result = build.inst_ir_cmd_ir_op(IrCmd::SqrtNum, vb);
    } else if tm == TMS::TmPow && n == 2.0 {
      result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulNum, vb, vb);
    } else if tm == TMS::TmPow && n == 3.0 {
      let vv = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulNum, vb, vb);
      result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulNum, vb, vv);
    } else {
      vc = build.const_double(n);
    }
  } else {
    vc = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, opc);
  }

  // If result is None, we need to emit the generic numeric op
  if result.kind() == IrOpKind::None {
    CODEGEN_ASSERT!(vc.kind() != IrOpKind::None);
    result = match tm {
      TMS::TmAdd => build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, vb, vc),
      TMS::TmSub => build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubNum, vb, vc),
      TMS::TmMul => build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulNum, vb, vc),
      TMS::TmDiv => build.inst_ir_cmd_ir_op_ir_op(IrCmd::DivNum, vb, vc),
      TMS::TmIDiv => build.inst_ir_cmd_ir_op_ir_op(IrCmd::IdivNum, vb, vc),
      TMS::TmMod => build.inst_ir_cmd_ir_op_ir_op(IrCmd::ModNum, vb, vc),
      TMS::TmPow => {
        let pow = build.const_uint(LuauBuiltinFunction::LBF_MATH_POW as u32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::InvokeLibm, pow, vb, vc)
      }
      _ => {
        CODEGEN_ASSERT!(false);
        IrOp::new()
      }
    };
  }

  let reg_ra = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, reg_ra, result);

  if ra != rb && ra != rc {
    let reg_ra = build.vm_reg(ra as u8);
    let number_tag = build.const_tag(LuaType::Number as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, reg_ra, number_tag);
  }

  translate_binary_numeric_fallback_if_required(build, fallback, ra, opb, opc, tm, pcpos);
}
