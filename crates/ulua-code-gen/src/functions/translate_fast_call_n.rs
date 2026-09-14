use ulua_common::{
  FFlag,
  enums::luau_opcode::LuauOpcode,
  macros::{
    luau_insn_a::LUAU_INSN_A, luau_insn_b::LUAU_INSN_B, luau_insn_c::LUAU_INSN_C,
    luau_insn_op::LUAU_INSN_OP,
  },
};
use ulua_vm::{enums::lua_type::LuaType, macros::lua_multret::LUA_MULTRET};

use crate::{
  enums::{
    builtin_impl_type::BuiltinImplType, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
    ir_op_kind::IrOpKind,
  },
  functions::{
    get_op_length::get_op_length, translate_builtin::translate_builtin, vm_const_op::vm_const_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{builtin_args::BuiltinArgs, ir_builder::IrBuilder, ir_op::IrOp},
  type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_fast_call_n(
  build: &mut IrBuilder,
  pc: *const Instruction,
  pcpos: i32,
  custom_params: bool,
  custom_param_count: i32,
  custom_args: IrOp,
  custom_arg3: IrOp,
) -> IrOp {
  let opcode = unsafe { LuauOpcode::from(LUAU_INSN_OP(*pc) as u8) };
  let bfid = LUAU_INSN_A(unsafe { *pc }) as i32;
  let skip = LUAU_INSN_C(unsafe { *pc }) as i32;

  let call = unsafe { *pc.add(skip as usize + 1) };
  CODEGEN_ASSERT!(LuauOpcode::from(LUAU_INSN_OP(call) as u8) == LuauOpcode::LOP_CALL);
  let ra = LUAU_INSN_A(call) as i32;

  let nparams = if custom_params {
    custom_param_count
  } else {
    (LUAU_INSN_B(call) as i32) - 1
  };
  let nresults = (LUAU_INSN_C(call) as i32) - 1;
  let arg = if custom_params {
    LUAU_INSN_B(unsafe { *pc }) as i32
  } else {
    ra + 1
  };
  let args = if custom_params {
    custom_args
  } else {
    build.vm_reg((ra + 2) as u8)
  };

  let mut builtin_args = args;

  if args.kind() == IrOpKind::VmConst {
    CODEGEN_ASSERT!(!build.function.proto.is_null());
    let protok = unsafe {
      let idx = vm_const_op(args);
      let proto = build.function.proto;
      (*proto).k.add(idx as usize).read()
    };

    if protok.tt == LuaType::Number as i32 {
      builtin_args = build.const_double(unsafe { protok.value.n });
    } else if FFlag::LuauCodegenInteger2.get()
      && FFlag::LuauCodegenIntegerFastcall2k.get()
      && protok.tt == LuaType::Integer as i32
    {
      builtin_args = build.const_int_64(unsafe { protok.value.l });
    }
  }

  let builtin_arg3 = if custom_params {
    custom_arg3
  } else {
    build.vm_reg((ra + 3) as u8)
  };

  let fallback = build.fallback_block(pcpos as u32);

  build.check_safe_env(pcpos + get_op_length(opcode));

  let bargs = BuiltinArgs {
    ra,
    arg,
    args: builtin_args,
    arg3: builtin_arg3,
    nparams,
    nresults,
    pcpos: pcpos + get_op_length(opcode),
  };

  let br = translate_builtin(build, bfid, bargs, fallback);

  if br.r#type != BuiltinImplType::None {
    CODEGEN_ASSERT!(nparams != LUA_MULTRET);

    if nresults == LUA_MULTRET {
      let reg_ra = build.vm_reg(ra as u8);
      let actual_count = build.const_int(br.actual_result_count);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::AdjustStackToReg, reg_ra, actual_count);
    } else {
      let reg_ra_next = build.vm_reg((ra + 1) as u8);
      let dead_count = build.const_int(-1);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::MarkDead, reg_ra_next, dead_count);
    }

    if br.r#type != BuiltinImplType::UsesFallback {
      let block = build.function.block_op(fallback);

      block.kind = IrBlockKind::Dead;

      return build.undef();
    }
  } else {
    let arg3 = if custom_params {
      custom_arg3
    } else {
      build.undef()
    };

    let savedpc = build.const_uint((pcpos + get_op_length(opcode)) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc);

    let bfid_op = build.const_uint(bfid as u32);
    let reg_ra = build.vm_reg(ra as u8);
    let reg_arg = build.vm_reg(arg as u8);
    let nparams_op = build.const_int(nparams);
    let nresults_op = build.const_int(nresults);
    let res = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
      IrCmd::InvokeFastcall,
      bfid_op,
      reg_ra,
      reg_arg,
      args,
      arg3,
      nparams_op,
      nresults_op,
    );
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckFastcallRes, res, fallback);

    if nresults == LUA_MULTRET {
      let reg_ra = build.vm_reg(ra as u8);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::AdjustStackToReg, reg_ra, res);
    } else if nparams == LUA_MULTRET {
      build.inst_ir_cmd(IrCmd::AdjustStackToTop);
    }
  }

  fallback
}
