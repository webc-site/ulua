use core::mem::size_of;

use ulua_vm::{
  macros::lua_multret::LUA_MULTRET,
  records::{call_info::CallInfo, lua_state::lua_State},
  type_aliases::t_value::TValue,
};

use crate::{
  enums::{condition_x_64::ConditionX64, size_x_64::SizeX64},
  functions::{dword_reg::dword_reg, luau_reg::luau_reg, luau_reg_address::luau_reg_address},
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, label::Label, module_helpers::ModuleHelpers,
    operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

pub fn emit_inst_return(
  build: &mut AssemblyBuilderX64,
  helpers: &mut ModuleHelpers,
  ra: i32,
  actual_results: i32,
  function_variadic: bool,
) {
  let res = RegisterX64::RDI;
  let written = dword_reg(RegisterX64::RCX);

  if function_variadic {
    build.mov(
      OperandX64::reg(res),
      mem(
        SizeX64::Qword,
        r_state(),
        core::mem::offset_of!(lua_State, ci) as i32,
      ),
    );
    build.mov(
      OperandX64::reg(res),
      mem(
        SizeX64::Qword,
        res,
        core::mem::offset_of!(CallInfo, func) as i32,
      ),
    );
  } else if actual_results != 1 {
    build.lea_operand_x_64_operand_x_64(
      OperandX64::reg(res),
      mem(SizeX64::None, r_base(), -(size_of::<TValue>() as i32)),
    );
  }

  if actual_results == 0 {
    build.xor_(OperandX64::reg(written), OperandX64::reg(written));
    build.jmp_label(&mut helpers.return_);
  } else if actual_results == 1 && !function_variadic {
    build.vmovups(OperandX64::reg(xmm(0)), luau_reg(ra));
    build.vmovups(
      mem(SizeX64::Xmmword, r_base(), -(size_of::<TValue>() as i32)),
      OperandX64::reg(xmm(0)),
    );
    build.mov(OperandX64::reg(res), OperandX64::reg(r_base()));
    build.mov(OperandX64::reg(written), OperandX64::imm(1));
    build.jmp_label(&mut helpers.return_);
  } else if (1..=3).contains(&actual_results) {
    for r in 0..actual_results {
      build.vmovups(OperandX64::reg(xmm(0)), luau_reg(ra + r));
      build.vmovups(
        mem(SizeX64::Xmmword, res, r * size_of::<TValue>() as i32),
        OperandX64::reg(xmm(0)),
      );
    }
    build.add(
      OperandX64::reg(res),
      OperandX64::imm(actual_results * size_of::<TValue>() as i32),
    );
    build.mov(OperandX64::reg(written), OperandX64::imm(actual_results));
    build.jmp_label(&mut helpers.return_);
  } else {
    let vali = RegisterX64::RAX;
    let valend = RegisterX64::RDX;

    build.lea_operand_x_64_operand_x_64(OperandX64::reg(vali), luau_reg_address(ra));

    if actual_results == LUA_MULTRET {
      build.mov(
        OperandX64::reg(valend),
        mem(
          SizeX64::Qword,
          r_state(),
          core::mem::offset_of!(lua_State, top) as i32,
        ),
      );
    } else {
      build.lea_operand_x_64_operand_x_64(
        OperandX64::reg(valend),
        luau_reg_address(ra + actual_results),
      );
    }

    build.xor_(OperandX64::reg(written), OperandX64::reg(written));

    let mut repeat_value_loop = Label::default();
    let mut exit_value_loop = Label::default();

    if actual_results == LUA_MULTRET {
      build.cmp(OperandX64::reg(vali), OperandX64::reg(valend));
      build.jcc(ConditionX64::NotBelow, &mut exit_value_loop);
    }

    build.set_label(&mut repeat_value_loop);
    build.vmovups(OperandX64::reg(xmm(0)), mem(SizeX64::Xmmword, vali, 0));
    build.vmovups(mem(SizeX64::Xmmword, res, 0), OperandX64::reg(xmm(0)));
    build.add(
      OperandX64::reg(vali),
      OperandX64::imm(size_of::<TValue>() as i32),
    );
    build.add(
      OperandX64::reg(res),
      OperandX64::imm(size_of::<TValue>() as i32),
    );
    build.inc(OperandX64::reg(written));
    build.cmp(OperandX64::reg(vali), OperandX64::reg(valend));
    build.jcc(ConditionX64::Below, &mut repeat_value_loop);

    build.set_label_label(&mut exit_value_loop);
    build.jmp_label(&mut helpers.return_);
  }
}

const fn reg(index: u8, size: SizeX64) -> RegisterX64 {
  RegisterX64 {
    bits: (index << RegisterX64::INDEX_SHIFT) | size as u8,
  }
}

const fn xmm(index: u8) -> RegisterX64 {
  reg(index, SizeX64::Xmmword)
}

const fn r_state() -> RegisterX64 {
  reg(15, SizeX64::Qword)
}

const fn r_base() -> RegisterX64 {
  reg(14, SizeX64::Qword)
}

fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
  OperandX64::mem(size, RegisterX64::NOREG, 1, base, disp)
}
