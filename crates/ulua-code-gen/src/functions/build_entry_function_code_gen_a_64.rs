use core::mem::offset_of;

use ulua_vm::{
  records::{
    call_info::CallInfo as CallInfoRecord, lua_state::LuaState as lua_StateRecord,
    lua_t_value::lua_TValue as TValueRecord, proto::Proto as ProtoRecord,
  },
  type_aliases::value::Value,
};

use crate::{
  functions::unwind_header_ops::{as_impl_mut, unwind_finish_function, unwind_start_function},
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, emit_common_a_64,
    entry_locations_code_gen_a_64::EntryLocations, register_a_64::RegisterA64,
    unwind_builder::UnwindBuilder,
  },
  type_aliases::mem::mem,
};

/// cpp `UnwindBuilder::kFullBlockFunction = ~0u`（`UnwindBuilder.h:19`）：入口函数覆盖整块剩余代码。
const K_FULL_BLOCK_FUNCTION: u32 = 0xffff_ffff;

// `UnwindBuilder` 与平台具体实现（Dwarf2/Win）共享 repr(C) 前缀布局，向下转型统一走
// `as_impl_mut`（cpp 基类虚调用 `unwind.startFunction()/prologueA64()/finishFunction()` 的移植形态，
// 与 `build_entry_function_code_gen_x_64.rs` 的同名壳一致）。
//
// 此前这三个壳是空实现：A64 下每个 code block 唯一的 FDE 就由入口函数登记（cpp
// `CodeGenA64.cpp:292-294` 以 `kFullBlockFunction` 覆盖整块），空实现使 JIT 帧完全没有展开信息，
// 任何在原生帧之上抛出的 `lua_exception`（VM 的 longjmp 仿真）都会让 `_Unwind_RaiseException`
// 以 `_URC_END_OF_STACK`(5) 返回，std 随即 `failed to initiate panic, error 5` 直接 abort。
fn unwind_prologue_a_64(
  unwind: &mut UnwindBuilder,
  prologue_size: u32,
  stack_size: u32,
  regs: &[RegisterA64],
) {
  // Safety: 同 `unwind_start_function`——下转类型正确且该 `&mut` 借用为此刻唯一持有者。
  unsafe { as_impl_mut(unwind) }.prologue_a_64(prologue_size, stack_size, regs);
}

const SP: RegisterA64 = RegisterA64 { bits: (31 << 3) };
const X0: RegisterA64 = RegisterA64 { bits: 2 };
const X1: RegisterA64 = RegisterA64 { bits: (1 << 3) | 2 };
const X2: RegisterA64 = RegisterA64 { bits: (2 << 3) | 2 };
const X3: RegisterA64 = RegisterA64 { bits: (3 << 3) | 2 };
const X9: RegisterA64 = RegisterA64 { bits: (9 << 3) | 2 };
const X19: RegisterA64 = RegisterA64 {
  bits: (19 << 3) | 2,
};
const X20: RegisterA64 = RegisterA64 {
  bits: (20 << 3) | 2,
};
const X21: RegisterA64 = RegisterA64 {
  bits: (21 << 3) | 2,
};
const X22: RegisterA64 = RegisterA64 {
  bits: (22 << 3) | 2,
};
const X23: RegisterA64 = RegisterA64 {
  bits: (23 << 3) | 2,
};
const X24: RegisterA64 = RegisterA64 {
  bits: (24 << 3) | 2,
};
const X25: RegisterA64 = RegisterA64 {
  bits: (25 << 3) | 2,
};
const X29: RegisterA64 = RegisterA64 {
  bits: (29 << 3) | 2,
};
const X30: RegisterA64 = RegisterA64 {
  bits: (30 << 3) | 2,
};

/// entry 函数帧尺寸：cpp `inline constexpr unsigned kStackSize = (kStashSlots + kTempSlots + kSpillSlots) * 8;`
/// （EmitCommonA64.h:46 = (9+1+22)*8 = 256）。常量源收敛于 `records::emit_common_a_64`，
/// 必须容纳 spill 槽区 `mem(sp, S_SPILL_AREA + slot*8)`（slot < K_SPILL_SLOTS）的最高字节；
/// 曾错写为 128，导致槽 6..=21 的 spill str/ldr 越过帧底踩毁调用者栈。
const K_STACK_SIZE: u16 = emit_common_a_64::K_STACK_SIZE as u16;

// 编译期锚定：帧尺寸不得小于 (K_STASH_SLOTS + K_TEMP_SLOTS + K_SPILL_SLOTS) * 8（cpp EmitCommonA64.h:46 同式）。
const _: () = assert!(emit_common_a_64::K_STACK_SIZE >= (9 + 1 + 22) * 8);

pub fn build_entry_function_assembly_builder_a_64_unwind_builder(
  build: &mut AssemblyBuilderA64,
  unwind: &mut UnwindBuilder,
) -> EntryLocations {
  let mut locations = EntryLocations {
    start: build.set_label(),
    ..Default::default()
  };

  build.sub_register_a_64_register_a_64_u16(SP, SP, K_STACK_SIZE);
  build.stp(X29, X30, mem(SP, 0));

  build.stp(X19, X20, mem(SP, 16));
  build.stp(X21, X22, mem(SP, 32));
  build.stp(X23, X24, mem(SP, 48));
  build.str(X25, mem(SP, 64));

  build.mov_register_a_64_register_a_64(X29, SP);

  locations.prologue_end = build.set_label();

  let prologue_size =
    build.get_label_offset(&locations.prologue_end) - build.get_label_offset(&locations.start);

  let r_state = X19;
  let r_native_context = X20;
  let r_global_state = X21;
  let r_constants = X22;
  let r_closure = X23;
  let r_code = X24;
  let r_base = X25;

  build.mov_register_a_64_register_a_64(r_state, X0);
  build.mov_register_a_64_register_a_64(r_native_context, X3);
  build.ldr(
    r_global_state,
    mem(X0, offset_of!(lua_StateRecord, global) as i32),
  );
  build.ldr(r_base, mem(X0, offset_of!(lua_StateRecord, base) as i32));

  build.ldp(
    r_constants,
    r_code,
    mem(X1, offset_of!(ProtoRecord, k) as i32),
  );

  build.ldr(X9, mem(X0, offset_of!(lua_StateRecord, ci) as i32));
  build.ldr(X9, mem(X9, offset_of!(CallInfoRecord, func) as i32));
  build.ldr(
    r_closure,
    mem(
      X9,
      offset_of!(TValueRecord, value) as i32 + offset_of!(Value, gc) as i32,
    ),
  );

  build.br(X2);

  locations.epilogue_start = build.set_label();

  build.ldr(X25, mem(SP, 64));
  build.ldp(X23, X24, mem(SP, 48));
  build.ldp(X21, X22, mem(SP, 32));
  build.ldp(X19, X20, mem(SP, 16));
  build.ldp(X29, X30, mem(SP, 0));
  build.add_register_a_64_register_a_64_u16(SP, SP, K_STACK_SIZE);

  build.ret();

  unwind_start_function(unwind);
  unwind_prologue_a_64(
    unwind,
    prologue_size,
    K_STACK_SIZE as u32,
    &[X29, X30, X19, X20, X21, X22, X23, X24, X25],
  );
  unwind_finish_function(
    unwind,
    build.get_label_offset(&locations.start),
    K_FULL_BLOCK_FUNCTION,
  );

  locations
}
