use alloc::vec::Vec;
use core::mem::offset_of;

use ulua_vm::{
  records::{call_info::CallInfo, lua_state::LuaState, proto::Proto},
  type_aliases::{t_value::TValue, value::Value},
};

use crate::{
  enums::{abix_64::ABIX64, alignment_data_x_64::AlignmentDataX64, size_x_64::SizeX64},
  functions::{
    get_full_stack_size::{K_STACK_ALIGN, get_full_stack_size},
    get_non_vol_xmm_storage_size::get_non_vol_xmm_storage_size,
    get_xmm_register_count::get_xmm_register_count,
    mem_x_64::mem,
    s_closure::s_closure,
    s_code::s_code,
    unwind_header_ops::{as_impl_mut, unwind_finish_function, unwind_start_function},
  },
  records::{
    assembly_builder_x_64::AssemblyBuilderX64,
    emit_common_x_64::{K_FUNCTION_ALIGNMENT, R_BASE, R_CONSTANTS, R_NATIVE_CONTEXT, R_STATE},
    entry_locations_code_gen_x_64::EntryLocations,
    ir_call_wrapper_x_64::IrCallWrapperX64,
    label::Label,
    operand_x_64::OperandX64,
    register_x_64::RegisterX64,
    unwind_builder::UnwindBuilder,
  },
};

const K_WINDOWS_FIRST_NON_VOL_XMM_REG: u8 = 6;
const K_FULL_BLOCK_FUNCTION: u32 = 0xffff_ffff;

fn set_fresh_label(build: &mut AssemblyBuilderX64) -> Label {
  let mut label = Label::default();
  build.set_label(&mut label);
  label
}

// `UnwindBuilder` 与平台实现共享 repr(C) 前缀布局，向下转型统一走 `as_impl_mut`
// （cpp 基类指针转换的移植形态）。
fn unwind_prologue_x_64(
  unwind: &mut UnwindBuilder,
  prologue_size: u32,
  full_stack_size: u32,
  setup_frame: bool,
  gpr: &[RegisterX64],
  simd: &[RegisterX64],
) {
  // Safety: 同 `unwind_start_function`——`unwind` 由上下文构造点保证指向本平台具体 unwind 实现，
  // `repr(C)` 前缀布局基址重合，下转类型正确且该 `&mut` 借用为此刻唯一持有者，无别名冲突。
  unsafe { as_impl_mut(unwind) }.prologue_x_64(
    prologue_size,
    full_stack_size,
    setup_frame,
    gpr,
    simd,
  );
}

pub fn build_entry_function(
  build: &mut AssemblyBuilderX64,
  unwind: &mut UnwindBuilder,
) -> EntryLocations {
  let mut locations = EntryLocations::default();

  build.align(K_FUNCTION_ALIGNMENT, AlignmentDataX64::Ud2);

  locations.start = set_fresh_label(build);
  unwind_start_function(unwind);

  // cpp CodeGenX64.cpp:75-78 无条件使用 suggestArgumentRegister（Rust 移植期引入的
  // LuauCodegenSuggestArgumentRegisterX64 开关在 cpp 中不存在，且两分支结果恒等，移除）。
  let (r_arg1, r_arg2, r_arg3, r_arg4) = (
    IrCallWrapperX64::suggest_argument_register::<0>(SizeX64::Qword, build),
    IrCallWrapperX64::suggest_argument_register::<1>(SizeX64::Qword, build),
    IrCallWrapperX64::suggest_argument_register::<2>(SizeX64::Qword, build),
    IrCallWrapperX64::suggest_argument_register::<3>(SizeX64::Qword, build),
  );

  if build.abi == ABIX64::SYSTEM_V {
    build.push(OperandX64::reg(RegisterX64::RBP));
    build.mov(
      OperandX64::reg(RegisterX64::RBP),
      OperandX64::reg(RegisterX64::RSP),
    );
  }

  build.push(OperandX64::reg(RegisterX64::RBX));
  build.push(OperandX64::reg(RegisterX64::R12));
  build.push(OperandX64::reg(RegisterX64::R13));
  build.push(OperandX64::reg(RegisterX64::R14));
  build.push(OperandX64::reg(RegisterX64::R15));

  if build.abi == ABIX64::WINDOWS {
    build.push(OperandX64::reg(RegisterX64::RDI));
    build.push(OperandX64::reg(RegisterX64::RSI));
    build.push(OperandX64::reg(RegisterX64::RBP));
  }

  let usable_xmm_reg_count = get_xmm_register_count(build.abi);
  let xmm_storage_size = get_non_vol_xmm_storage_size(build.abi, usable_xmm_reg_count);
  let full_stack_size = get_full_stack_size(build.abi, usable_xmm_reg_count);

  build.sub(
    OperandX64::reg(RegisterX64::RSP),
    OperandX64::imm(full_stack_size as i32),
  );

  let xmm_storage_offset = full_stack_size as i32 - (K_STACK_ALIGN + xmm_storage_size) as i32;
  let mut saved_xmm_regs: Vec<RegisterX64> = Vec::new();

  if build.abi == ABIX64::WINDOWS {
    if usable_xmm_reg_count > K_WINDOWS_FIRST_NON_VOL_XMM_REG {
      saved_xmm_regs.reserve((usable_xmm_reg_count - K_WINDOWS_FIRST_NON_VOL_XMM_REG) as usize);
    }

    let mut offset = 0;
    for i in K_WINDOWS_FIRST_NON_VOL_XMM_REG..usable_xmm_reg_count {
      let xmm_reg = RegisterX64::make(SizeX64::Xmmword, i);
      build.vmovaps(
        mem(
          SizeX64::Xmmword,
          RegisterX64::RSP,
          xmm_storage_offset + offset,
        ),
        OperandX64::reg(xmm_reg),
      );
      saved_xmm_regs.push(xmm_reg);
      offset += 16;
    }
  }

  locations.prologue_end = set_fresh_label(build);

  let prologue_size =
    build.get_label_offset(&locations.prologue_end) - build.get_label_offset(&locations.start);

  if build.abi == ABIX64::SYSTEM_V {
    unwind_prologue_x_64(
      unwind,
      prologue_size,
      full_stack_size,
      true,
      &[
        RegisterX64::RBX,
        RegisterX64::R12,
        RegisterX64::R13,
        RegisterX64::R14,
        RegisterX64::R15,
      ],
      &[],
    );
  } else if build.abi == ABIX64::WINDOWS {
    unwind_prologue_x_64(
      unwind,
      prologue_size,
      full_stack_size,
      false,
      &[
        RegisterX64::RBX,
        RegisterX64::R12,
        RegisterX64::R13,
        RegisterX64::R14,
        RegisterX64::R15,
        RegisterX64::RDI,
        RegisterX64::RSI,
        RegisterX64::RBP,
      ],
      &saved_xmm_regs,
    );
  }

  build.mov(OperandX64::reg(R_STATE), OperandX64::reg(r_arg1));
  build.mov(OperandX64::reg(R_NATIVE_CONTEXT), OperandX64::reg(r_arg4));
  build.mov(
    OperandX64::reg(R_BASE),
    mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, base) as i32),
  );
  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    mem(SizeX64::Qword, R_STATE, offset_of!(LuaState, ci) as i32),
  );
  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    mem(
      SizeX64::Qword,
      RegisterX64::RAX,
      offset_of!(CallInfo, func) as i32,
    ),
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
    OperandX64::reg(R_CONSTANTS),
    mem(SizeX64::Qword, r_arg2, offset_of!(Proto, k) as i32),
  );
  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    mem(SizeX64::Qword, r_arg2, offset_of!(Proto, code) as i32),
  );
  build.mov(s_code(), OperandX64::reg(RegisterX64::RAX));

  build.jmp_operand_x_64(OperandX64::reg(r_arg3));

  locations.epilogue_start = set_fresh_label(build);

  if build.abi == ABIX64::WINDOWS {
    let mut offset = 0;
    for i in K_WINDOWS_FIRST_NON_VOL_XMM_REG..usable_xmm_reg_count {
      build.vmovaps(
        OperandX64::reg(RegisterX64::make(SizeX64::Xmmword, i)),
        mem(
          SizeX64::Xmmword,
          RegisterX64::RSP,
          xmm_storage_offset + offset,
        ),
      );
      offset += 16;
    }
  }

  build.add(
    OperandX64::reg(RegisterX64::RSP),
    OperandX64::imm(full_stack_size as i32),
  );

  if build.abi == ABIX64::WINDOWS {
    build.pop(OperandX64::reg(RegisterX64::RBP));
    build.pop(OperandX64::reg(RegisterX64::RSI));
    build.pop(OperandX64::reg(RegisterX64::RDI));
  }

  build.pop(OperandX64::reg(RegisterX64::R15));
  build.pop(OperandX64::reg(RegisterX64::R14));
  build.pop(OperandX64::reg(RegisterX64::R13));
  build.pop(OperandX64::reg(RegisterX64::R12));
  build.pop(OperandX64::reg(RegisterX64::RBX));

  if build.abi == ABIX64::SYSTEM_V {
    build.pop(OperandX64::reg(RegisterX64::RBP));
  }

  build.ret();

  unwind_finish_function(
    unwind,
    build.get_label_offset(&locations.start),
    K_FULL_BLOCK_FUNCTION,
  );

  locations
}
