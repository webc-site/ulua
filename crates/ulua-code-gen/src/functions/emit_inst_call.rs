use core::mem::{offset_of, size_of};

use ulua_vm::{
  enums::lua_type::LuaType,
  macros::{lua_callinfo_native::LUA_CALLINFO_NATIVE, lua_multret::LUA_MULTRET},
  records::{
    call_info::CallInfo,
    closure::{CClosure, Closure, LClosure},
    lua_state::LuaState,
    proto::Proto,
  },
  type_aliases::t_value::TValue,
};

use crate::{
  enums::{condition_x_64::ConditionX64, size_x_64::SizeX64},
  functions::{
    emit_update_base_emit_common_x_64::emit_update_base, luau_reg::luau_reg,
    luau_reg_address::luau_reg_address, luau_reg_tag::luau_reg_tag, mem_x_64::mem,
    s_closure::s_closure, s_code::s_code,
  },
  records::{
    assembly_builder_x_64::AssemblyBuilderX64,
    emit_common_x_64::{R_BASE, R_CONSTANTS, R_NATIVE_CONTEXT, R_STATE},
    ir_call_wrapper_x_64::IrCallWrapperX64,
    ir_data::K_INVALID_INST_IDX,
    ir_op::IrOp,
    ir_reg_alloc_x_64::IrRegAllocX64,
    label::Label,
    module_helpers::ModuleHelpers,
    native_context::NativeContext,
    operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};

pub fn emit_inst_call(
  regs: &mut IrRegAllocX64,
  build: &mut AssemblyBuilderX64,
  helpers: &mut ModuleHelpers,
  ra: i32,
  nparams: i32,
  nresults: i32,
) {
  // cpp EmitInstructionX64.cpp:26-40：emitInstCall 无条件经 IrCallWrapperX64 发起
  // callProlog 调用（移植期开关 LuauCodeGenCallWrapperEmitInst 在 cpp 中已删除）。
  let mut call_wrapper =
    IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(regs, build, K_INVALID_INST_IDX);

  call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    OperandX64::reg(R_STATE),
    IrOp::new(),
  );
  call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Qword,
    luau_reg_address(ra),
    IrOp::new(),
  );
  if nparams == LUA_MULTRET {
    call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Qword,
      mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, top) as i32),
      IrOp::new(),
    );
  } else {
    call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Qword,
      luau_reg_address(ra + 1 + nparams),
      IrOp::new(),
    );
  }
  call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
    SizeX64::Dword,
    OperandX64::imm(nresults),
    IrOp::new(),
  );
  call_wrapper.call(&native_context_slot(
    offset_of!(NativeContext, call_prolog) as i32
  ));

  let ccl = RegisterX64::RAX;
  emit_update_base(build);

  let mut c_func_call = Label::default();

  build.test(
    mem(SizeX64::Byte, ccl, offset_of!(Closure, is_c) as i32),
    OperandX64::imm(1),
  );
  build.jcc(ConditionX64::NotZero, &mut c_func_call);

  {
    let proto = RegisterX64::RCX;
    let ci = RegisterX64::RDX;
    let argi = RegisterX64::RSI;
    let argend = RegisterX64::RDI;

    build.mov(
      OperandX64::reg(proto),
      mem(
        SizeX64::Qword,
        ccl,
        (offset_of!(Closure, inner) + offset_of!(LClosure, p)) as i32,
      ),
    );

    build.mov(s_closure(), OperandX64::reg(ccl));
    build.mov(
      OperandX64::reg(ci),
      mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, ci) as i32),
    );

    let mut fillnil = Label::default();
    let mut exitfillnil = Label::default();

    build.mov(
      OperandX64::reg(argi),
      mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, top) as i32),
    );

    build.movzx(
      RegisterX64::RAX.sized(SizeX64::Dword),
      mem(SizeX64::Byte, proto, offset_of!(Proto, numparams) as i32),
    );
    build.shl(
      OperandX64::reg(RegisterX64::RAX.sized(SizeX64::Dword)),
      OperandX64::imm(K_TVALUE_SIZE_LOG2),
    );
    build.lea_operand_x_64_operand_x_64(
      OperandX64::reg(argend),
      OperandX64::mem(
        SizeX64::None,
        RegisterX64::RAX.sized(SizeX64::Qword),
        1,
        R_BASE,
        0,
      ),
    );

    build.set_label(&mut fillnil);
    build.cmp(OperandX64::reg(argi), OperandX64::reg(argend));
    build.jcc(ConditionX64::NotBelow, &mut exitfillnil);

    build.mov(
      mem(SizeX64::Dword, argi, offset_of!(TValue, tt) as i32),
      OperandX64::imm(LuaType::Nil as i32),
    );
    build.add(
      OperandX64::reg(argi),
      OperandX64::imm(size_of::<TValue>() as i32),
    );
    build.jmp_label(&mut fillnil);

    build.set_label_label(&mut exitfillnil);

    build.mov(
      OperandX64::reg(RegisterX64::RAX),
      mem(SizeX64::Qword, ci, offset_of!(CallInfo, top) as i32),
    );

    let mut skip_vararg = Label::default();
    build.test(
      mem(SizeX64::Byte, proto, offset_of!(Proto, is_vararg) as i32),
      OperandX64::imm(1),
    );
    build.jcc(ConditionX64::Zero, &mut skip_vararg);
    build.mov(OperandX64::reg(RegisterX64::RAX), OperandX64::reg(argi));

    build.set_label_label(&mut skip_vararg);

    build.mov(
      mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, top) as i32),
      OperandX64::reg(RegisterX64::RAX),
    );

    build.mov(
      OperandX64::reg(RegisterX64::RAX),
      mem(SizeX64::Qword, proto, offset_of!(Proto, code) as i32),
    );
    build.mov(s_code(), OperandX64::reg(RegisterX64::RAX));
    build.mov(
      mem(SizeX64::Qword, ci, offset_of!(CallInfo, savedpc) as i32),
      OperandX64::reg(RegisterX64::RAX),
    );

    build.mov(
      OperandX64::reg(R_CONSTANTS),
      mem(SizeX64::Qword, proto, offset_of!(Proto, k) as i32),
    );

    build.mov(
      OperandX64::reg(RegisterX64::RAX),
      mem(SizeX64::Qword, proto, offset_of!(Proto, exectarget) as i32),
    );
    build.test(
      OperandX64::reg(RegisterX64::RAX),
      OperandX64::reg(RegisterX64::RAX),
    );
    build.jcc(ConditionX64::Zero, &mut helpers.exit_continue_vm);

    build.mov(
      mem(SizeX64::Dword, ci, offset_of!(CallInfo, flags) as i32),
      OperandX64::imm(LUA_CALLINFO_NATIVE),
    );

    build.jmp_operand_x_64(OperandX64::reg(RegisterX64::RAX));
  }

  build.set_label_label(&mut c_func_call);

  {
    // cpp EmitInstructionX64.cpp:120-125：c.f(L) 经 wrapper 调用
    regs.take_reg(ccl, K_INVALID_INST_IDX);
    let mut call_wrapper =
      IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(regs, build, K_INVALID_INST_IDX);
    call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Qword,
      OperandX64::reg(R_STATE),
      IrOp::new(),
    );
    call_wrapper.call(&mem(
      SizeX64::Qword,
      ccl,
      (offset_of!(Closure, inner) + offset_of!(CClosure, f)) as i32,
    ));

    let results = RegisterX64::RAX.sized(SizeX64::Dword);

    build.test(OperandX64::reg(results), OperandX64::reg(results));
    build.jcc(ConditionX64::Less, &mut helpers.exit_no_continue_vm);

    if nresults != 0 && nresults != 1 {
      // cpp EmitInstructionX64.cpp:133-141：callEpilogC 经 wrapper 调用
      regs.take_reg(results, K_INVALID_INST_IDX);
      let mut call_wrapper = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
        regs,
        build,
        K_INVALID_INST_IDX,
      );
      call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::Qword,
        OperandX64::reg(R_STATE),
        IrOp::new(),
      );
      call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::Dword,
        OperandX64::imm(nresults),
        IrOp::new(),
      );
      call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::Dword,
        OperandX64::reg(results),
        IrOp::new(),
      );
      call_wrapper.call(&native_context_slot(
        offset_of!(NativeContext, call_epilog_c) as i32,
      ));

      emit_update_base(build);
      return;
    }

    let ci = RegisterX64::RDX;
    let cip = RegisterX64::RCX;
    let vali = RegisterX64::RSI;

    build.mov(
      OperandX64::reg(ci),
      mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, ci) as i32),
    );
    build.lea_operand_x_64_operand_x_64(
      OperandX64::reg(cip),
      mem(SizeX64::None, ci, -(size_of::<CallInfo>() as i32)),
    );

    build.mov(
      OperandX64::reg(R_BASE),
      mem(SizeX64::Qword, cip, offset_of!(CallInfo, base) as i32),
    );
    build.mov(
      mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, base) as i32),
      OperandX64::reg(R_BASE),
    );

    if nresults == 1 {
      build.mov(
        OperandX64::reg(vali),
        mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, top) as i32),
      );
      build.shl(
        OperandX64::reg(results),
        OperandX64::imm(K_TVALUE_SIZE_LOG2),
      );
      build.sub(
        OperandX64::reg(vali),
        OperandX64::reg(results.sized(SizeX64::Qword)),
      );
      build.vmovups(
        OperandX64::reg(RegisterX64::XMM0),
        OperandX64::mem(SizeX64::Xmmword, RegisterX64::NOREG, 1, vali, 0),
      );
      build.vmovups(luau_reg(ra), OperandX64::reg(RegisterX64::XMM0));

      let mut skipnil = Label::default();
      build.test(OperandX64::reg(results), OperandX64::reg(results));
      build.jcc(ConditionX64::NotZero, &mut skipnil);
      build.mov(luau_reg_tag(ra), OperandX64::imm(LuaType::Nil as i32));
      build.set_label_label(&mut skipnil);
    }

    build.mov(
      mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, ci) as i32),
      OperandX64::reg(cip),
    );
    build.mov(
      OperandX64::reg(RegisterX64::RAX),
      mem(SizeX64::Qword, cip, offset_of!(CallInfo, top) as i32),
    );
    build.mov(
      mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, top) as i32),
      OperandX64::reg(RegisterX64::RAX),
    );
  }
}

const K_TVALUE_SIZE_LOG2: i32 = 4;

fn native_context_slot(disp: i32) -> OperandX64 {
  mem(SizeX64::Qword, R_NATIVE_CONTEXT, disp)
}
