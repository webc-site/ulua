use core::mem::size_of;

use ulua_common::FFlag::LuauCodeGenCallWrapperEmitInst;
use ulua_vm::{
  enums::lua_type::LuaType,
  macros::{lua_callinfo_native::LUA_CALLINFO_NATIVE, lua_multret::LUA_MULTRET},
  records::{
    call_info::CallInfo,
    closure::{CClosure, Closure, LClosure},
    lua_state::lua_State,
    proto::Proto,
  },
  type_aliases::t_value::TValue,
};

use crate::{
  enums::{abix_64::ABIX64, condition_x_64::ConditionX64, size_x_64::SizeX64},
  functions::{
    emit_update_base_emit_common_x_64::emit_update_base,
    get_full_stack_size::K_STACK_OFFSET_TO_LOCALS, luau_reg::luau_reg,
    luau_reg_address::luau_reg_address, luau_reg_tag::luau_reg_tag,
  },
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, ir_call_wrapper_x_64::IrCallWrapperX64,
    ir_data::K_INVALID_INST_IDX, ir_op::IrOp, ir_reg_alloc_x_64::IrRegAllocX64, label::Label,
    module_helpers::ModuleHelpers, native_context::NativeContext, operand_x_64::OperandX64,
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
  if LuauCodeGenCallWrapperEmitInst.get() {
    let mut call_wrapper =
      IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(regs, build, K_INVALID_INST_IDX);

    call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
      SizeX64::Qword,
      OperandX64::reg(r_state()),
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
        mem(
          SizeX64::Qword,
          r_state(),
          core::mem::offset_of!(lua_State, top) as i32,
        ),
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
      core::mem::offset_of!(NativeContext, call_prolog) as i32,
    ));
  } else {
    let (r_arg1, r_arg2, r_arg3, r_arg4) = abi_arg_regs(build);

    build.mov(OperandX64::reg(r_arg1), OperandX64::reg(r_state()));
    build.lea_operand_x_64_operand_x_64(OperandX64::reg(r_arg2), luau_reg_address(ra));

    if nparams == LUA_MULTRET {
      build.mov(
        OperandX64::reg(r_arg3),
        mem(
          SizeX64::Qword,
          r_state(),
          core::mem::offset_of!(lua_State, top) as i32,
        ),
      );
    } else {
      build
        .lea_operand_x_64_operand_x_64(OperandX64::reg(r_arg3), luau_reg_address(ra + 1 + nparams));
    }

    build.mov(
      OperandX64::reg(sized(r_arg4, SizeX64::Dword)),
      OperandX64::imm(nresults),
    );
    build.call_operand_x_64(native_context_slot(
      core::mem::offset_of!(NativeContext, call_prolog) as i32,
    ));
  }

  let ccl = RegisterX64::RAX;
  emit_update_base(build);

  let mut c_func_call = Label::default();

  build.test(
    mem(
      SizeX64::Byte,
      ccl,
      core::mem::offset_of!(Closure, is_c) as i32,
    ),
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
        (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, p)) as i32,
      ),
    );

    build.mov(s_closure(), OperandX64::reg(ccl));
    build.mov(
      OperandX64::reg(ci),
      mem(
        SizeX64::Qword,
        r_state(),
        core::mem::offset_of!(lua_State, ci) as i32,
      ),
    );

    let mut fillnil = Label::default();
    let mut exitfillnil = Label::default();

    build.mov(
      OperandX64::reg(argi),
      mem(
        SizeX64::Qword,
        r_state(),
        core::mem::offset_of!(lua_State, top) as i32,
      ),
    );

    build.movzx(
      sized(RegisterX64::RAX, SizeX64::Dword),
      mem(
        SizeX64::Byte,
        proto,
        core::mem::offset_of!(Proto, numparams) as i32,
      ),
    );
    build.shl(
      OperandX64::reg(sized(RegisterX64::RAX, SizeX64::Dword)),
      OperandX64::imm(K_TVALUE_SIZE_LOG2),
    );
    build.lea_operand_x_64_operand_x_64(
      OperandX64::reg(argend),
      OperandX64::mem(
        SizeX64::None,
        sized(RegisterX64::RAX, SizeX64::Qword),
        1,
        r_base(),
        0,
      ),
    );

    build.set_label(&mut fillnil);
    build.cmp(OperandX64::reg(argi), OperandX64::reg(argend));
    build.jcc(ConditionX64::NotBelow, &mut exitfillnil);

    build.mov(
      mem(
        SizeX64::Dword,
        argi,
        core::mem::offset_of!(TValue, tt) as i32,
      ),
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
      mem(
        SizeX64::Qword,
        ci,
        core::mem::offset_of!(CallInfo, top) as i32,
      ),
    );

    let mut skip_vararg = Label::default();
    build.test(
      mem(
        SizeX64::Byte,
        proto,
        core::mem::offset_of!(Proto, is_vararg) as i32,
      ),
      OperandX64::imm(1),
    );
    build.jcc(ConditionX64::Zero, &mut skip_vararg);
    build.mov(OperandX64::reg(RegisterX64::RAX), OperandX64::reg(argi));

    build.set_label_label(&mut skip_vararg);

    build.mov(
      mem(
        SizeX64::Qword,
        r_state(),
        core::mem::offset_of!(lua_State, top) as i32,
      ),
      OperandX64::reg(RegisterX64::RAX),
    );

    build.mov(
      OperandX64::reg(RegisterX64::RAX),
      mem(
        SizeX64::Qword,
        proto,
        core::mem::offset_of!(Proto, code) as i32,
      ),
    );
    build.mov(s_code(), OperandX64::reg(RegisterX64::RAX));
    build.mov(
      mem(
        SizeX64::Qword,
        ci,
        core::mem::offset_of!(CallInfo, savedpc) as i32,
      ),
      OperandX64::reg(RegisterX64::RAX),
    );

    build.mov(
      OperandX64::reg(r_constants()),
      mem(
        SizeX64::Qword,
        proto,
        core::mem::offset_of!(Proto, k) as i32,
      ),
    );

    build.mov(
      OperandX64::reg(RegisterX64::RAX),
      mem(
        SizeX64::Qword,
        proto,
        core::mem::offset_of!(Proto, exectarget) as i32,
      ),
    );
    build.test(
      OperandX64::reg(RegisterX64::RAX),
      OperandX64::reg(RegisterX64::RAX),
    );
    build.jcc(ConditionX64::Zero, &mut helpers.exit_continue_vm);

    build.mov(
      mem(
        SizeX64::Dword,
        ci,
        core::mem::offset_of!(CallInfo, flags) as i32,
      ),
      OperandX64::imm(LUA_CALLINFO_NATIVE),
    );

    build.jmp_operand_x_64(OperandX64::reg(RegisterX64::RAX));
  }

  build.set_label_label(&mut c_func_call);

  {
    if LuauCodeGenCallWrapperEmitInst.get() {
      regs.take_reg(ccl, K_INVALID_INST_IDX);
      let mut call_wrapper = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
        regs,
        build,
        K_INVALID_INST_IDX,
      );
      call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::Qword,
        OperandX64::reg(r_state()),
        IrOp::new(),
      );
      call_wrapper.call(&mem(
        SizeX64::Qword,
        ccl,
        (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(CClosure, f)) as i32,
      ));
    } else {
      let (r_arg1, ..) = abi_arg_regs(build);
      build.mov(OperandX64::reg(r_arg1), OperandX64::reg(r_state()));
      build.call_operand_x_64(mem(
        SizeX64::Qword,
        ccl,
        (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(CClosure, f)) as i32,
      ));
    }

    let results = sized(RegisterX64::RAX, SizeX64::Dword);

    build.test(OperandX64::reg(results), OperandX64::reg(results));
    build.jcc(ConditionX64::Less, &mut helpers.exit_no_continue_vm);

    if nresults != 0 && nresults != 1 {
      if LuauCodeGenCallWrapperEmitInst.get() {
        regs.take_reg(results, K_INVALID_INST_IDX);
        let mut call_wrapper = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
          regs,
          build,
          K_INVALID_INST_IDX,
        );
        call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
          SizeX64::Qword,
          OperandX64::reg(r_state()),
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
          core::mem::offset_of!(NativeContext, call_epilog_c) as i32,
        ));
      } else {
        let (r_arg1, r_arg2, r_arg3, _) = abi_arg_regs(build);

        build.mov(OperandX64::reg(r_arg1), OperandX64::reg(r_state()));
        build.mov(
          OperandX64::reg(sized(r_arg2, SizeX64::Dword)),
          OperandX64::imm(nresults),
        );
        build.mov(
          OperandX64::reg(sized(r_arg3, SizeX64::Dword)),
          OperandX64::reg(results),
        );
        build
          .call_operand_x_64(native_context_slot(
            core::mem::offset_of!(NativeContext, call_epilog_c) as i32,
          ));
      }

      emit_update_base(build);
      return;
    }

    let ci = RegisterX64::RDX;
    let cip = RegisterX64::RCX;
    let vali = RegisterX64::RSI;

    build.mov(
      OperandX64::reg(ci),
      mem(
        SizeX64::Qword,
        r_state(),
        core::mem::offset_of!(lua_State, ci) as i32,
      ),
    );
    build.lea_operand_x_64_operand_x_64(
      OperandX64::reg(cip),
      mem(SizeX64::None, ci, -(size_of::<CallInfo>() as i32)),
    );

    build.mov(
      OperandX64::reg(r_base()),
      mem(
        SizeX64::Qword,
        cip,
        core::mem::offset_of!(CallInfo, base) as i32,
      ),
    );
    build.mov(
      mem(
        SizeX64::Qword,
        r_state(),
        core::mem::offset_of!(lua_State, base) as i32,
      ),
      OperandX64::reg(r_base()),
    );

    if nresults == 1 {
      build.mov(
        OperandX64::reg(vali),
        mem(
          SizeX64::Qword,
          r_state(),
          core::mem::offset_of!(lua_State, top) as i32,
        ),
      );
      build.shl(
        OperandX64::reg(results),
        OperandX64::imm(K_TVALUE_SIZE_LOG2),
      );
      build.sub(
        OperandX64::reg(vali),
        OperandX64::reg(sized(results, SizeX64::Qword)),
      );
      build.vmovups(
        OperandX64::reg(xmm(0)),
        OperandX64::mem(SizeX64::Xmmword, RegisterX64::NOREG, 1, vali, 0),
      );
      build.vmovups(luau_reg(ra), OperandX64::reg(xmm(0)));

      let mut skipnil = Label::default();
      build.test(OperandX64::reg(results), OperandX64::reg(results));
      build.jcc(ConditionX64::NotZero, &mut skipnil);
      build.mov(luau_reg_tag(ra), OperandX64::imm(LuaType::Nil as i32));
      build.set_label_label(&mut skipnil);
    }

    build.mov(
      mem(
        SizeX64::Qword,
        r_state(),
        core::mem::offset_of!(lua_State, ci) as i32,
      ),
      OperandX64::reg(cip),
    );
    build.mov(
      OperandX64::reg(RegisterX64::RAX),
      mem(
        SizeX64::Qword,
        cip,
        core::mem::offset_of!(CallInfo, top) as i32,
      ),
    );
    build.mov(
      mem(
        SizeX64::Qword,
        r_state(),
        core::mem::offset_of!(lua_State, top) as i32,
      ),
      OperandX64::reg(RegisterX64::RAX),
    );
  }
}

const K_TVALUE_SIZE_LOG2: i32 = 4;

const fn reg(index: u8, size: SizeX64) -> RegisterX64 {
  RegisterX64 {
    bits: (index << RegisterX64::INDEX_SHIFT) | size as u8,
  }
}

const fn sized(reg: RegisterX64, size: SizeX64) -> RegisterX64 {
  RegisterX64 {
    bits: (reg.index() << RegisterX64::INDEX_SHIFT) | size as u8,
  }
}

const fn xmm(index: u8) -> RegisterX64 {
  reg(index, SizeX64::Xmmword)
}

const fn r_state() -> RegisterX64 {
  reg(15, SizeX64::Qword)
}

const fn r_native_context() -> RegisterX64 {
  reg(13, SizeX64::Qword)
}

const fn r_constants() -> RegisterX64 {
  reg(12, SizeX64::Qword)
}

const fn r_base() -> RegisterX64 {
  RegisterX64::RBP
}

fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
  OperandX64::mem(size, RegisterX64::NOREG, 1, base, disp)
}

fn native_context_slot(disp: i32) -> OperandX64 {
  mem(SizeX64::Qword, r_native_context(), disp)
}

fn s_closure() -> OperandX64 {
  mem(
    SizeX64::Qword,
    RegisterX64::RSP,
    K_STACK_OFFSET_TO_LOCALS as i32,
  )
}

fn s_code() -> OperandX64 {
  mem(
    SizeX64::Qword,
    RegisterX64::RSP,
    K_STACK_OFFSET_TO_LOCALS as i32 + 8,
  )
}

fn abi_arg_regs(
  build: &AssemblyBuilderX64,
) -> (RegisterX64, RegisterX64, RegisterX64, RegisterX64) {
  if build.abi == ABIX64::WINDOWS {
    (
      RegisterX64::RCX,
      RegisterX64::RDX,
      RegisterX64::R8,
      RegisterX64::R9,
    )
  } else {
    (
      RegisterX64::RDI,
      RegisterX64::RSI,
      RegisterX64::RDX,
      RegisterX64::RCX,
    )
  }
}
