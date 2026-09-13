use core::mem::size_of;

use ulua_common::{DFFlag, FFlag::LuauClosureUsageCounter};
use ulua_vm::{
  enums::lua_type::LuaType,
  macros::{lua_callinfo_native::LUA_CALLINFO_NATIVE, lua_callinfo_return::LUA_CALLINFO_RETURN},
  records::{
    call_info::CallInfo,
    closure::{Closure, LClosure},
    lua_state::lua_State,
    proto::Proto,
  },
  type_aliases::{t_value::TValue, value::Value},
};

use crate::{
  enums::{condition_x_64::ConditionX64, size_x_64::SizeX64},
  functions::dword_reg::dword_reg,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, label::Label, module_helpers::ModuleHelpers,
    operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};
const fn r_state() -> RegisterX64 {
  RegisterX64 {
    bits: (15u8 << RegisterX64::INDEX_SHIFT) | SizeX64::Qword as u8,
  }
}

// C++ EmitCommonX64.h: `constexpr RegisterX64 rConstants = r12;`
const fn r_constants() -> RegisterX64 {
  RegisterX64 {
    bits: (12u8 << RegisterX64::INDEX_SHIFT) | SizeX64::Qword as u8,
  }
}

const K_STACK_OFFSET_TO_LOCALS: i32 = 16 + 32;

fn s_closure() -> OperandX64 {
  OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    RegisterX64::RSP,
    K_STACK_OFFSET_TO_LOCALS,
  )
}

fn s_code() -> OperandX64 {
  OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    RegisterX64::RSP,
    K_STACK_OFFSET_TO_LOCALS + 8,
  )
}

fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
  OperandX64::mem(size, RegisterX64::NOREG, 1, base, disp)
}

pub fn emit_return(build: &mut AssemblyBuilderX64, helpers: &mut ModuleHelpers) {
  // input: res in rdi, number of written values in ecx
  let res = RegisterX64::RDI;
  let written = dword_reg(RegisterX64::RCX);

  let ci = RegisterX64::R8;
  let cip = RegisterX64::R9;
  let nresults = dword_reg(RegisterX64::RSI);

  let tvalue_size = size_of::<TValue>() as i32;
  let callinfo_size = size_of::<CallInfo>() as i32;

  build.mov(
    OperandX64::reg(ci),
    mem(
      SizeX64::Qword,
      r_state(),
      core::mem::offset_of!(lua_State, ci) as i32,
    ),
  );
  build.lea_operand_x_64_operand_x_64(OperandX64::reg(cip), mem(SizeX64::None, ci, -callinfo_size));

  // nresults = ci->nresults
  build.mov(
    OperandX64::reg(nresults),
    mem(
      SizeX64::Dword,
      ci,
      core::mem::offset_of!(CallInfo, nresults) as i32,
    ),
  );

  let mut skip_result_copy = Label { id: 0, location: 0 };

  // Fill the rest of the expected results (nresults - written) with 'nil'
  let counter = written;
  build.sub(OperandX64::reg(counter), OperandX64::reg(nresults)); // counter = -(nresults - written)
  build.jcc(ConditionX64::GreaterEqual, &mut skip_result_copy);

  let mut repeat_nil_loop = Label { id: 0, location: 0 };
  build.set_label(&mut repeat_nil_loop);
  build.mov(
    mem(
      SizeX64::Dword,
      res,
      core::mem::offset_of!(TValue, tt) as i32,
    ),
    OperandX64::imm(LuaType::Nil as i32),
  );
  build.add(OperandX64::reg(res), OperandX64::imm(tvalue_size));
  build.inc(OperandX64::reg(counter));
  build.jcc(ConditionX64::NotZero, &mut repeat_nil_loop);

  build.set_label_label(&mut skip_result_copy);

  // l->ci = cip
  build.mov(
    mem(
      SizeX64::Qword,
      r_state(),
      core::mem::offset_of!(lua_State, ci) as i32,
    ),
    OperandX64::reg(cip),
  );
  // sync base = l->base while we have a chance (rBase = rbp in this crate)
  build.mov(
    OperandX64::reg(RegisterX64::RBP),
    mem(
      SizeX64::Qword,
      cip,
      core::mem::offset_of!(CallInfo, base) as i32,
    ),
  );
  // l->base = cip->base
  build.mov(
    mem(
      SizeX64::Qword,
      r_state(),
      core::mem::offset_of!(lua_State, base) as i32,
    ),
    OperandX64::reg(RegisterX64::RBP),
  );

  let mut skip_fixed_ret_top = Label { id: 0, location: 0 };
  build.test(OperandX64::reg(nresults), OperandX64::reg(nresults));
  build.jcc(ConditionX64::Less, &mut skip_fixed_ret_top);
  build.mov(
    OperandX64::reg(res),
    mem(
      SizeX64::Qword,
      cip,
      core::mem::offset_of!(CallInfo, top) as i32,
    ),
  );
  build.set_label_label(&mut skip_fixed_ret_top);

  // l->top = res
  build.mov(
    mem(
      SizeX64::Qword,
      r_state(),
      core::mem::offset_of!(lua_State, top) as i32,
    ),
    OperandX64::reg(res),
  );

  if LuauClosureUsageCounter.get() {
    build.mov(OperandX64::reg(RegisterX64::RAX), s_closure());
    build.dec(mem(
      SizeX64::Qword,
      RegisterX64::RAX,
      core::mem::offset_of!(Closure, usage) as i32,
    ));
  }

  // Unlikely, but this might be the last return from VM
  build.test(
    mem(
      SizeX64::Byte,
      ci,
      core::mem::offset_of!(CallInfo, flags) as i32,
    ),
    OperandX64::imm(LUA_CALLINFO_RETURN),
  );
  build.jcc(ConditionX64::NotZero, &mut helpers.exit_no_continue_vm);

  // Returning back to the previous function is a bit tricky
  // Registers alive: r9 (cip)
  let proto = RegisterX64::RCX;
  let execdata = RegisterX64::RBX;
  let exectarget = RegisterX64::R10;

  // Change closure
  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    mem(
      SizeX64::Qword,
      cip,
      core::mem::offset_of!(CallInfo, func) as i32,
    ),
  );
  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    mem(
      SizeX64::Qword,
      RegisterX64::RAX,
      (core::mem::offset_of!(TValue, value) + core::mem::offset_of!(Value, gc)) as i32,
    ),
  );
  build.mov(s_closure(), OperandX64::reg(RegisterX64::RAX));

  build.mov(
    OperandX64::reg(proto),
    mem(
      SizeX64::Qword,
      RegisterX64::RAX,
      (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, p)) as i32,
    ),
  );

  build.mov(
    OperandX64::reg(execdata),
    mem(
      SizeX64::Qword,
      proto,
      core::mem::offset_of!(Proto, execdata) as i32,
    ),
  );

  build.test(
    mem(
      SizeX64::Byte,
      cip,
      core::mem::offset_of!(CallInfo, flags) as i32,
    ),
    OperandX64::imm(LUA_CALLINFO_NATIVE),
  );
  build.jcc(ConditionX64::Zero, &mut helpers.exit_continue_vm); // Continue in interpreter if function has no native data

  if DFFlag::AddReturnExectargetCheck.get() {
    build.mov(
      OperandX64::reg(exectarget),
      mem(
        SizeX64::Qword,
        proto,
        core::mem::offset_of!(Proto, exectarget) as i32,
      ),
    );
    build.test(OperandX64::reg(exectarget), OperandX64::reg(exectarget));
    build.jcc(
      ConditionX64::Zero,
      &mut helpers.exit_continue_vm_clear_native_flag,
    );
  }

  // Change constants
  build.mov(
    OperandX64::reg(r_constants()),
    mem(
      SizeX64::Qword,
      proto,
      core::mem::offset_of!(Proto, k) as i32,
    ),
  );

  // Change code
  build.mov(
    OperandX64::reg(RegisterX64::RDX),
    mem(
      SizeX64::Qword,
      proto,
      core::mem::offset_of!(Proto, code) as i32,
    ),
  );
  build.mov(s_code(), OperandX64::reg(RegisterX64::RDX));

  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    mem(
      SizeX64::Qword,
      cip,
      core::mem::offset_of!(CallInfo, savedpc) as i32,
    ),
  );

  // To get instruction index from instruction pointer, we need to divide byte offset by 4
  // But we will actually need to scale instruction index by 4 back to byte offset later so it cancels out
  build.sub(
    OperandX64::reg(RegisterX64::RAX),
    OperandX64::reg(RegisterX64::RDX),
  );

  // Get new instruction location and jump to it
  build.mov(
    OperandX64::reg(dword_reg(RegisterX64::RDX)),
    OperandX64::mem(SizeX64::Dword, RegisterX64::RAX, 1, execdata, 0),
  );

  if DFFlag::AddReturnExectargetCheck.get() {
    build.add(
      OperandX64::reg(RegisterX64::RDX),
      OperandX64::reg(exectarget),
    );
  } else {
    build.add(
      OperandX64::reg(RegisterX64::RDX),
      mem(
        SizeX64::Qword,
        proto,
        core::mem::offset_of!(Proto, exectarget) as i32,
      ),
    );
  }
  build.jmp_operand_x_64(OperandX64::reg(RegisterX64::RDX));
}
