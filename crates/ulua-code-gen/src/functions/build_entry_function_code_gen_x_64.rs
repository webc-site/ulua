use alloc::vec::Vec;

use ulua_common::FFlag::LuauCodegenSuggestArgumentRegisterX64;
use ulua_vm::{
  records::{call_info::CallInfo, lua_state::lua_State, proto::Proto},
  type_aliases::{t_value::TValue, value::Value},
};

use crate::{
  enums::{abix_64::ABIX64, alignment_data_x_64::AlignmentDataX64, size_x_64::SizeX64},
  functions::{
    get_full_stack_size::{K_STACK_ALIGN, K_STACK_OFFSET_TO_LOCALS, get_full_stack_size},
    get_non_vol_xmm_storage_size::get_non_vol_xmm_storage_size,
    get_xmm_register_count::get_xmm_register_count,
  },
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, entry_locations_code_gen_x_64::EntryLocations,
    ir_call_wrapper_x_64::IrCallWrapperX64, label::Label, operand_x_64::OperandX64,
    register_x_64::RegisterX64, unwind_builder::UnwindBuilder,
    unwind_builder_dwarf_2::UnwindBuilderDwarf2,
  },
};

const K_FUNCTION_ALIGNMENT: u32 = 32;
const K_WINDOWS_FIRST_NON_VOL_XMM_REG: u8 = 6;
const K_FULL_BLOCK_FUNCTION: u32 = 0xffff_ffff;

const fn reg(index: u8, size: SizeX64) -> RegisterX64 {
  RegisterX64 {
    bits: (index << RegisterX64::INDEX_SHIFT) | size as u8,
  }
}

const R12: RegisterX64 = reg(12, SizeX64::Qword);
const R13: RegisterX64 = reg(13, SizeX64::Qword);
const R14: RegisterX64 = reg(14, SizeX64::Qword);
const R15: RegisterX64 = reg(15, SizeX64::Qword);

const R_CONSTANTS: RegisterX64 = R12;
const R_NATIVE_CONTEXT: RegisterX64 = R13;
const R_BASE: RegisterX64 = R14;
const R_STATE: RegisterX64 = R15;

fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
  OperandX64::mem(size, RegisterX64::NOREG, 1, base, disp)
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

fn set_fresh_label(build: &mut AssemblyBuilderX64) -> Label {
  let mut label = Label::default();
  build.set_label(&mut label);
  label
}

#[cfg(target_os = "windows")]
fn unwind_start_function(unwind: &mut UnwindBuilder) {
  unsafe {
    (&mut *(unwind as *mut UnwindBuilder)
      .cast::<crate::records::unwind_builder_win::UnwindBuilderWin>())
      .start_function();
  }
}

#[cfg(not(target_os = "windows"))]
fn unwind_start_function(unwind: &mut UnwindBuilder) {
  unsafe {
    (&mut *(unwind as *mut UnwindBuilder).cast::<UnwindBuilderDwarf2>()).start_function();
  }
}

#[cfg(target_os = "windows")]
fn unwind_prologue_x_64(
  unwind: &mut UnwindBuilder,
  prologue_size: u32,
  full_stack_size: u32,
  setup_frame: bool,
  gpr: &[RegisterX64],
  simd: &[RegisterX64],
) {
  unsafe {
    (&mut *(unwind as *mut UnwindBuilder)
      .cast::<crate::records::unwind_builder_win::UnwindBuilderWin>())
      .prologue_x_64(prologue_size, full_stack_size, setup_frame, gpr, simd);
  }
}

#[cfg(not(target_os = "windows"))]
fn unwind_prologue_x_64(
  unwind: &mut UnwindBuilder,
  prologue_size: u32,
  full_stack_size: u32,
  setup_frame: bool,
  gpr: &[RegisterX64],
  simd: &[RegisterX64],
) {
  unsafe {
    (&mut *(unwind as *mut UnwindBuilder).cast::<UnwindBuilderDwarf2>()).prologue_x_64(
      prologue_size,
      full_stack_size,
      setup_frame,
      gpr,
      simd,
    );
  }
}

#[cfg(target_os = "windows")]
fn unwind_finish_function(unwind: &mut UnwindBuilder, begin_offset: u32, end_offset: u32) {
  unsafe {
    (&mut *(unwind as *mut UnwindBuilder)
      .cast::<crate::records::unwind_builder_win::UnwindBuilderWin>())
      .finish_function(begin_offset, end_offset);
  }
}

#[cfg(not(target_os = "windows"))]
fn unwind_finish_function(unwind: &mut UnwindBuilder, begin_offset: u32, end_offset: u32) {
  unsafe {
    (&mut *(unwind as *mut UnwindBuilder).cast::<UnwindBuilderDwarf2>())
      .finish_function(begin_offset, end_offset);
  }
}

pub fn build_entry_function(
  build: &mut AssemblyBuilderX64,
  unwind: &mut UnwindBuilder,
) -> EntryLocations {
  let mut locations = EntryLocations::default();

  build.align(K_FUNCTION_ALIGNMENT, AlignmentDataX64::Ud2);

  locations.start = set_fresh_label(build);
  unwind_start_function(unwind);

  let (r_arg1, r_arg2, r_arg3, r_arg4) = if LuauCodegenSuggestArgumentRegisterX64.get() {
    (
      IrCallWrapperX64::suggest_argument_register::<0>(SizeX64::Qword, build),
      IrCallWrapperX64::suggest_argument_register::<1>(SizeX64::Qword, build),
      IrCallWrapperX64::suggest_argument_register::<2>(SizeX64::Qword, build),
      IrCallWrapperX64::suggest_argument_register::<3>(SizeX64::Qword, build),
    )
  } else if build.abi == ABIX64::WINDOWS {
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
  };

  if build.abi == ABIX64::SYSTEM_V {
    build.push(OperandX64::reg(RegisterX64::RBP));
    build.mov(
      OperandX64::reg(RegisterX64::RBP),
      OperandX64::reg(RegisterX64::RSP),
    );
  }

  build.push(OperandX64::reg(RegisterX64::RBX));
  build.push(OperandX64::reg(R12));
  build.push(OperandX64::reg(R13));
  build.push(OperandX64::reg(R14));
  build.push(OperandX64::reg(R15));

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
      let xmm_reg = reg(i, SizeX64::Xmmword);
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
      &[RegisterX64::RBX, R12, R13, R14, R15],
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
        R12,
        R13,
        R14,
        R15,
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
    mem(
      SizeX64::Qword,
      R_STATE,
      core::mem::offset_of!(lua_State, base) as i32,
    ),
  );
  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    mem(
      SizeX64::Qword,
      R_STATE,
      core::mem::offset_of!(lua_State, ci) as i32,
    ),
  );
  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    mem(
      SizeX64::Qword,
      RegisterX64::RAX,
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
    OperandX64::reg(R_CONSTANTS),
    mem(
      SizeX64::Qword,
      r_arg2,
      core::mem::offset_of!(Proto, k) as i32,
    ),
  );
  build.mov(
    OperandX64::reg(RegisterX64::RAX),
    mem(
      SizeX64::Qword,
      r_arg2,
      core::mem::offset_of!(Proto, code) as i32,
    ),
  );
  build.mov(s_code(), OperandX64::reg(RegisterX64::RAX));

  build.jmp_operand_x_64(OperandX64::reg(r_arg3));

  locations.epilogue_start = set_fresh_label(build);

  if build.abi == ABIX64::WINDOWS {
    let mut offset = 0;
    for i in K_WINDOWS_FIRST_NON_VOL_XMM_REG..usable_xmm_reg_count {
      build.vmovaps(
        OperandX64::reg(reg(i, SizeX64::Xmmword)),
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

  build.pop(OperandX64::reg(R15));
  build.pop(OperandX64::reg(R14));
  build.pop(OperandX64::reg(R13));
  build.pop(OperandX64::reg(R12));
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
