use core::mem::{offset_of, size_of};

use ulua_common::dfflag;
use ulua_vm::{
  enums::lua_type::LuaType,
  macros::{lua_callinfo_native::LUA_CALLINFO_NATIVE, lua_callinfo_return::LUA_CALLINFO_RETURN},
  records::{
    call_info::CallInfo,
    closure::{Closure, LClosure},
    lua_state::LuaState,
    proto::Proto,
  },
  type_aliases::{t_value::TValue, value::Value},
};

use crate::{
  enums::{condition_x_64::ConditionX64, size_x_64::SizeX64},
  functions::{dword_reg::dword_reg, s_closure::s_closure, s_code::s_code},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64,
    emit_common_x_64::{R_CONSTANTS, R_STATE},
    label::Label,
    module_helpers::ModuleHelpers,
    operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};
// C++ EmitCommonX64.h: `constexpr RegisterX64 rConstants = r12;`
fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
  OperandX64::mem(size, RegisterX64::NOREG, 1, base, disp)
}

pub fn emit_return(build: &mut AssemblyBuilderX64, helpers: &mut ModuleHelpers) {
  // 输入：结果在 rdi，写入值个数在 ecx
  let res = RegisterX64::RDI;
  let written = dword_reg(RegisterX64::RCX);

  let ci = RegisterX64::R8;
  let cip = RegisterX64::R9;
  let nresults = dword_reg(RegisterX64::RSI);

  let tvalue_size = size_of::<TValue>() as i32;
  let callinfo_size = size_of::<CallInfo>() as i32;

  build.mov(
    OperandX64::reg(ci),
    mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, ci) as i32),
  );
  build.lea_operand_x_64_operand_x_64(OperandX64::reg(cip), mem(SizeX64::None, ci, -callinfo_size));

  // nresults = ci->nresults
  build.mov(
    OperandX64::reg(nresults),
    mem(SizeX64::Dword, ci, offset_of!(CallInfo, nresults) as i32),
  );

  let mut skip_result_copy = Label { id: 0, location: 0 };

  // 用 'nil' 填满期望结果的剩余部分（nresults - written）
  let counter = written;
  build.sub(OperandX64::reg(counter), OperandX64::reg(nresults)); // counter = -(nresults - written)
  build.jcc(ConditionX64::GreaterEqual, &mut skip_result_copy);

  let mut repeat_nil_loop = Label { id: 0, location: 0 };
  build.set_label(&mut repeat_nil_loop);
  build.mov(
    mem(SizeX64::Dword, res, offset_of!(TValue, tt) as i32),
    OperandX64::imm(LuaType::Nil as i32),
  );
  build.add(OperandX64::reg(res), OperandX64::imm(tvalue_size));
  build.inc(OperandX64::reg(counter));
  build.jcc(ConditionX64::NotZero, &mut repeat_nil_loop);

  build.set_label_label(&mut skip_result_copy);

  // l->ci = cip
  build.mov(
    mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, ci) as i32),
    OperandX64::reg(cip),
  );
  // 趁还有机会时同步 base = l->base。cpp EmitCommonX64.h: `inline constexpr RegisterX64 rBase = r14;`
  build.mov(
    OperandX64::reg(RegisterX64::R14),
    mem(SizeX64::Qword, cip, offset_of!(CallInfo, base) as i32),
  );
  // l->base = cip->base
  build.mov(
    mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, base) as i32),
    OperandX64::reg(RegisterX64::R14),
  );

  let mut skip_fixed_ret_top = Label { id: 0, location: 0 };
  build.test(OperandX64::reg(nresults), OperandX64::reg(nresults));
  build.jcc(ConditionX64::Less, &mut skip_fixed_ret_top);
  build.mov(
    OperandX64::reg(res),
    mem(SizeX64::Qword, cip, offset_of!(CallInfo, top) as i32),
  );
  build.set_label_label(&mut skip_fixed_ret_top);

  // l->top = res
  build.mov(
    mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, top) as i32),
    OperandX64::reg(res),
  );

  // 不太可能，但这可能是最后一次从 VM 返回
  build.test(
    mem(SizeX64::Byte, ci, offset_of!(CallInfo, flags) as i32),
    OperandX64::imm(LUA_CALLINFO_RETURN),
  );
  build.jcc(ConditionX64::NotZero, &mut helpers.exit_no_continue_vm);

  // 返回上一个函数有点棘手
  // 存活寄存器：r9 (cip)
  let proto = RegisterX64::RCX;
  let execdata = RegisterX64::RBX;
  let exectarget = RegisterX64::R10;

  // 更换 closure
  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    mem(SizeX64::Qword, cip, offset_of!(CallInfo, func) as i32),
  );
  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    mem(
      SizeX64::Qword,
      RegisterX64::RAX,
      (offset_of!(TValue, value) + offset_of!(Value, gc)) as i32,
    ),
  );
  build.mov(s_closure(), OperandX64::reg(RegisterX64::RAX));

  build.mov(
    OperandX64::reg(proto),
    mem(
      SizeX64::Qword,
      RegisterX64::RAX,
      (offset_of!(Closure, inner) + offset_of!(LClosure, p)) as i32,
    ),
  );

  build.mov(
    OperandX64::reg(execdata),
    mem(SizeX64::Qword, proto, offset_of!(Proto, execdata) as i32),
  );

  build.test(
    mem(SizeX64::Byte, cip, offset_of!(CallInfo, flags) as i32),
    OperandX64::imm(LUA_CALLINFO_NATIVE),
  );
  build.jcc(ConditionX64::Zero, &mut helpers.exit_continue_vm); // Continue in interpreter if function has no native data

  if dfflag::AddReturnExectargetCheck.get() {
    build.mov(
      OperandX64::reg(exectarget),
      mem(SizeX64::Qword, proto, offset_of!(Proto, exectarget) as i32),
    );
    build.test(OperandX64::reg(exectarget), OperandX64::reg(exectarget));
    build.jcc(
      ConditionX64::Zero,
      &mut helpers.exit_continue_vm_clear_native_flag,
    );
  }

  // 更换 constants
  build.mov(
    OperandX64::reg(R_CONSTANTS),
    mem(SizeX64::Qword, proto, offset_of!(Proto, k) as i32),
  );

  // 更换 code
  build.mov(
    OperandX64::reg(RegisterX64::RDX),
    mem(SizeX64::Qword, proto, offset_of!(Proto, code) as i32),
  );
  build.mov(s_code(), OperandX64::reg(RegisterX64::RDX));

  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    mem(SizeX64::Qword, cip, offset_of!(CallInfo, savedpc) as i32),
  );

  // 由指令指针求指令下标需把字节偏移除以 4
  // 但之后还要把指令下标乘 4 还原为字节偏移，正好抵消
  build.sub(
    OperandX64::reg(RegisterX64::RAX),
    OperandX64::reg(RegisterX64::RDX),
  );

  // 取新指令位置并跳转过去
  build.mov(
    OperandX64::reg(dword_reg(RegisterX64::RDX)),
    OperandX64::mem(SizeX64::Dword, RegisterX64::RAX, 1, execdata, 0),
  );

  if dfflag::AddReturnExectargetCheck.get() {
    build.add(
      OperandX64::reg(RegisterX64::RDX),
      OperandX64::reg(exectarget),
    );
  } else {
    build.add(
      OperandX64::reg(RegisterX64::RDX),
      mem(SizeX64::Qword, proto, offset_of!(Proto, exectarget) as i32),
    );
  }
  build.jmp_operand_x_64(OperandX64::reg(RegisterX64::RDX));
}
