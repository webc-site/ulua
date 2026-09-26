use core::mem::transmute;

use crate::{
  enums::arch::Arch,
  functions::{
    build_entry_function_code_gen_x_64::build_entry_function,
    unwind_header_ops::{finish_info, set_begin_offset, start_info},
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, base_code_gen_context::BaseCodeGenContext,
    native_fn::GateFn,
  },
};

pub fn init_header_functions(code_gen_context: &mut BaseCodeGenContext) -> bool {
  let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

  unsafe {
    // Safety: `unwind_builder` 由 BaseCodeGenContext 构造时 `make_unwind_builder()`（Box::into_raw
    // 具体 unwind 实现）写入，非空且比 context 长寿；`&mut *..` 派生其独占借用，满足 `start_info`
    // 内 `as_impl_mut` 的 repr(C) 前缀下转契约。单线程、无并存别名。
    start_info(&mut *code_gen_context.unwind_builder, Arch::X64);
  }

  let entry_locations =
    // Safety: 同上——`&mut *unwind_builder` 独占借用指向本平台具体 unwind 实现；`build` 为本地
    // 存活 AssemblyBuilderX64，被调为安全 fn，unsafe 仅因裸指针重借用。
    unsafe { build_entry_function(&mut build, &mut *code_gen_context.unwind_builder) };

  build.finalize();

  unsafe {
    // Safety: 同 `start_info`——`&mut *unwind_builder` 为具体 unwind 实现的独占借用，满足
    // `finish_info` 契约。
    finish_info(&mut *code_gen_context.unwind_builder);
  }

  CODEGEN_ASSERT!(build.data.is_empty());

  // cpp CodeGenX64.cpp:205-211：gate 代码统一走 CodeAllocationData
  // （移植期开关 LuauCodegenFreeBlocks 在 cpp 中已删除）。
  code_gen_context.gate_allocation_data = unsafe {
    // Safety: `allocate` 的 `code`/`code_size` 源自刚 finalize 的存活 `build.code`（len 一致）；
    // `data` 指针取自 `build.data`，CODEGEN_ASSERT 已证其为空（data.len()=0），被调内部对
    // `data_size != 0` 才解引用，故 size 为 0 的空/dangling data 指针传入合法、不触发 UB。
    code_gen_context.code_allocator.allocate(
      build.data.as_ptr(),
      build.data.len(),
      build.code.as_ptr(),
      build.code.len(),
    )
  };

  if code_gen_context.gate_allocation_data.start.is_null() {
    return false;
  }

  let code_start = code_gen_context.gate_allocation_data.code_start;

  // Safety: `&mut *unwind_builder` 独占借用指向具体 unwind 实现，满足 `set_begin_offset` 契约。
  unsafe {
    set_begin_offset(
      &mut *code_gen_context.unwind_builder,
      build.get_label_offset(&entry_locations.prologue_end) as usize,
    );
  }

  // Safety: `code_start` 在确认 gate_allocation_data 非空（start 非 null）后为已分配 code 段基址，
  // `add(offset)` 用本 build 的 label 偏移，落在该 code 段界内。
  // gate_entry 处的 `start` 标签由 build_entry_function 按 `GateFn` 的 C ABI 生成为入口点，
  // 地址非空故 transmute 得 Some（null niche 表达未注册态）；ABI 收口在此一次完成，
  // 读取方 on_enter 直接类型化调用。gate_exit 为生成码经 offset_of! 直读的跳转目标，保持裸地址。
  unsafe {
    code_gen_context.context.gate_entry = transmute::<*mut u8, Option<GateFn>>(
      code_start.add(build.get_label_offset(&entry_locations.start) as usize),
    );
    code_gen_context.context.gate_exit =
      code_start.add(build.get_label_offset(&entry_locations.epilogue_start) as usize);
  }

  true
}
