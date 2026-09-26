use ulua_common::{
  enums::{luau_builtin_function::LuauBuiltinFunction, luau_bytecode_type::LuauBytecodeType},
  macros::{luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b, luau_insn_c::luau_insn_c},
};
use ulua_vm::enums::{lua_type::LuaType, tms::TMS};

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    check_tag_exit::check_tag_exit,
    get_initialized_fallback::get_initialized_fallback,
    is_userdata_bytecode_type::is_userdata_bytecode_type,
    load_double_or_constant::load_double_or_constant,
    proto_view::{constant_number, with_constant_value},
    tm_to_host_metamethod::tm_to_host_metamethod,
    translate_binary_numeric_fallback_if_required::translate_binary_numeric_fallback_if_required,
    vm_const_op::vm_const_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
  type_aliases::instruction_ir_builder::Instruction,
};

/// number-tag 守卫块收口（binary/minus 五处同构）：`LoadTag(reg_idx)` 后按 bytecode 类型侧
/// 是否 NUMBER 选择 vm_exit 或 initialized fallback 作失败出口，再 `CheckTag` 校验 Number。
#[inline]
pub(crate) fn check_number_tag_guard(
  build: &mut IrBuilder,
  reg_idx: u8,
  types_side_is_number: bool,
  pcpos: i32,
  fallback: &mut IrOp,
) {
  let reg = build.vm_reg(reg_idx);
  let tag_load = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg);
  let exit_or_fallback = if types_side_is_number {
    build.vm_exit(pcpos as u32)
  } else {
    get_initialized_fallback(build, fallback, pcpos)
  };
  let number_tag = build.const_tag(LuaType::Number as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag_load, number_tag, exit_or_fallback);
}

/// vector 四则 `tm → IrCmd` 映射（三处孪生 match 收口）：三站外层 `matches!` 守卫
/// 均已限定 `tm` 落在表内集合，`_` 兜底与 IDiv 合并即守卫外的原断言臂不可达。
#[inline]
fn vec_arith_cmd(tm: TMS) -> IrCmd {
  match tm {
    TMS::TmAdd => IrCmd::AddVec,
    TMS::TmSub => IrCmd::SubVec,
    TMS::TmMul => IrCmd::MulVec,
    TMS::TmDiv => IrCmd::DivVec,
    _ => IrCmd::IdivVec,
  }
}

/// 翻译双寄存器二元运算指令 (ADD, SUB, MUL, DIV, IDIV, MOD, POW)。
pub fn translate_inst_binary(build: &mut IrBuilder, code: &[Instruction], pcpos: i32, tm: TMS) {
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let rb = luau_insn_b(insn) as u8;
  let rc = luau_insn_c(insn) as u8;

  let opb = build.vm_reg(rb);
  let opc = build.vm_reg(rc);

  translate_inst_binary_numeric(build, ra as i32, rb as i32, rc as i32, opb, opc, pcpos, tm);
}

/// 翻译寄存器-常量二元运算指令 (ADDK, SUBK, MULK, DIVK, IDIVK, MODK, POWK)。
pub fn translate_inst_binary_k(build: &mut IrBuilder, code: &[Instruction], pcpos: i32, tm: TMS) {
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as i32;
  let rb = luau_insn_b(insn) as i32;
  let rc = luau_insn_c(insn);

  let opb = build.vm_reg(rb as u8);
  let opc = build.vm_const(rc);

  translate_inst_binary_numeric(build, ra, rb, -1, opb, opc, pcpos, tm);
}

/// 翻译常量-寄存器反向二元运算指令 (SUBRK, DIVRK)。
pub fn translate_inst_binary_rk(build: &mut IrBuilder, code: &[Instruction], pcpos: i32, tm: TMS) {
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let rb = luau_insn_b(insn) as u8;
  let rc = luau_insn_c(insn) as u8;

  let opb = build.vm_const(rb as u32);
  let opc = build.vm_reg(rc);

  translate_inst_binary_numeric(build, ra as i32, -1, rc as i32, opb, opc, pcpos, tm);
}

/// 翻译二元数值运算底层共享实现（含 Vector fast-path、宿主 UserData 元方法以及 Number 回退）。
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
  let a_is_number = bc_types.a == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8;
  let b_is_number = bc_types.b == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8;

  // 针对 vector 的特殊 fast-path，与 VM 中的情形对应
  if bc_types.a == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8
    && bc_types.b == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8
    && (matches!(
      tm,
      TMS::TmAdd | TMS::TmSub | TMS::TmMul | TMS::TmDiv | TMS::TmIDiv
    ))
  {
    let reg_rb = build.vm_reg(rb as u8);
    let tag_b = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rb);
    check_tag_exit(build, tag_b, LuaType::Vector as u8, pcpos);

    let reg_rc = build.vm_reg(rc as u8);
    let tag_c = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rc);
    check_tag_exit(build, tag_c, LuaType::Vector as u8, pcpos);

    let vb = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, opb, IrOp::new());

    let vc = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, opc, IrOp::new());

    let result = build.inst_ir_cmd_ir_op_ir_op(vec_arith_cmd(tm), vb, vc);

    let result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::TagVector, result, IrOp::new());
    let reg_ra = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, result);
    return;
  } else if !is_userdata_bytecode_type(bc_types.a)
    && bc_types.b == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8
    && (matches!(tm, TMS::TmMul | TMS::TmDiv | TMS::TmIDiv))
  {
    if rb != -1 {
      check_number_tag_guard(build, rb as u8, a_is_number, pcpos, &mut fallback);
    }

    let reg_rc = build.vm_reg(rc as u8);
    let tag_rc = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rc);
    check_tag_exit(build, tag_rc, LuaType::Vector as u8, pcpos);

    let load_d = load_double_or_constant(build, opb);
    let undef = IrOp::new();
    let num_float = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NumToFloat, load_d, undef);
    let vb = build.inst_ir_cmd_ir_op_ir_op(IrCmd::FloatToVec, num_float, IrOp::new());

    let vc = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, opc, undef);
    let result = build.inst_ir_cmd_ir_op_ir_op(vec_arith_cmd(tm), vb, vc);

    let result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::TagVector, result, IrOp::new());
    let reg_ra = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, result);

    translate_binary_numeric_fallback_if_required(build, fallback, ra, opb, opc, tm, pcpos);
    return;
  } else if bc_types.a == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8
    && !is_userdata_bytecode_type(bc_types.b)
    && (matches!(tm, TMS::TmMul | TMS::TmDiv | TMS::TmIDiv))
  {
    let reg_rb = build.vm_reg(rb as u8);
    let tag_rb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rb);
    check_tag_exit(build, tag_rb, LuaType::Vector as u8, pcpos);

    if rc != -1 {
      check_number_tag_guard(build, rc as u8, b_is_number, pcpos, &mut fallback);
    }

    let vb = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, opb, IrOp::new());
    let load_d = load_double_or_constant(build, opc);
    let num_float = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NumToFloat, load_d, IrOp::new());
    let vc = build.inst_ir_cmd_ir_op_ir_op(IrCmd::FloatToVec, num_float, IrOp::new());

    let result = build.inst_ir_cmd_ir_op_ir_op(vec_arith_cmd(tm), vb, vc);

    let result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::TagVector, result, IrOp::new());
    let reg_ra = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, result);

    translate_binary_numeric_fallback_if_required(build, fallback, ra, opb, opc, tm, pcpos);
    return;
  }

  if is_userdata_bytecode_type(bc_types.a) || is_userdata_bytecode_type(bc_types.b) {
    let host_hooks = unsafe { &*build.host_hooks };
    if let Some(userdata_metamethod) = host_hooks.userdata_metamethod {
      let handled = unsafe {
        userdata_metamethod(
          build as *mut IrBuilder,
          bc_types.a,
          bc_types.b,
          ra,
          opb,
          opc,
          tm_to_host_metamethod(tm as i32),
          pcpos,
        )
      };
      if handled {
        return;
      }
    }

    let savedpc = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc);
    let reg_ra = build.vm_reg(ra as u8);
    let tm_op = build.const_int(tm as i32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::DoArith, reg_ra, opb, opc, tm_op);
    return;
  }

  // fast-path：number
  if rb != -1 {
    check_number_tag_guard(build, rb as u8, a_is_number, pcpos, &mut fallback);
  }

  if rc != -1 && rc != rb {
    check_number_tag_guard(build, rc as u8, b_is_number, pcpos, &mut fallback);
  }

  let vb = load_double_or_constant(build, opb);
  let mut vc = IrOp::new();
  let mut result = IrOp::new();

  if opc.kind() == IrOpKind::VmConst {
    let protok_index = vm_const_op(opc);
    CODEGEN_ASSERT!(!build.function.proto.is_null());
    let n = with_constant_value(build.function.proto, protok_index as u32, |tv| {
      let tt = tv.tt;
      CODEGEN_ASSERT!(tt == LuaType::Number as i32);
      constant_number(tv)
    })
    .expect("translate_inst_binary_numeric: proto/k 非空且 vm_const 界内(codegen 契约)");
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

  // 若结果为 None，需要发出通用数值运算
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
    build.store_tag(reg_ra, LuaType::Number as u8);
  }

  translate_binary_numeric_fallback_if_required(build, fallback, ra, opb, opc, tm, pcpos);
}
