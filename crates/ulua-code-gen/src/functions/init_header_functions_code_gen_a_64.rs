use core::mem::{size_of, transmute};

use crate::{
  enums::arch::Arch,
  functions::{
    build_entry_function_code_gen_a_64::build_entry_function_assembly_builder_a_64_unwind_builder,
    unwind_header_ops::{finish_info, set_begin_offset, start_info},
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, base_code_gen_context::BaseCodeGenContext,
    native_fn::GateFn,
  },
};

pub fn init_header_functions(code_gen_context: &mut BaseCodeGenContext) -> bool {
  let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);

  unsafe {
    // Safety: `unwind_builder` 在 BaseCodeGenContext 构造时由 `make_unwind_builder()`（Box::into_raw
    // 一个本平台具体 unwind 实现）写入，非空且比 context 长寿；`&mut *..` 派生该对象的独占借用，
    // `start_info` 内 `as_impl_mut` 的 repr(C) 前缀下转契约由此满足。单线程、无并存别名。
    start_info(&mut *code_gen_context.unwind_builder, Arch::A64);
  }

  let entry_locations = unsafe {
    // Safety: 同上——`&mut *unwind_builder` 为独占借用指向具体 unwind 实现；`build` 为本地存活
    // AssemblyBuilderA64。被调为安全 fn，unsafe 仅因裸指针重借用。
    build_entry_function_assembly_builder_a_64_unwind_builder(
      &mut build,
      &mut *code_gen_context.unwind_builder,
    )
  };

  build.finalize();

  unsafe {
    // Safety: 同 `start_info`——`&mut *unwind_builder` 独占借用指向本平台具体 unwind 实现，满足
    // `finish_info` 的 `as_impl_mut` 契约。
    finish_info(&mut *code_gen_context.unwind_builder);
  }

  CODEGEN_ASSERT!(build.data.is_empty());

  let code_ptr = build.code.as_ptr().cast::<u8>();
  let code_size = build.code.len() * size_of::<u32>();

  // cpp CodeGenA64.cpp:321-328：gate 代码统一走 CodeAllocationData
  // （移植期开关 LuauCodegenFreeBlocks 在 cpp 中已删除）。
  code_gen_context.gate_allocation_data = unsafe {
    // Safety: `allocate` 的裸指针入参——`code_ptr/code_size` 源自刚 finalize 的 `build.code`（存活
    // 且大小一致）；`data` 取自 `build.data`，且 CODEGEN_ASSERT 已证其为空（data_size=0，被调内部
    // 以 `data_size != 0` 守卫、size 为 0 时不解引用），故空/dangling data 指针传入合法。
    code_gen_context.code_allocator.allocate(
      build.data.as_ptr(),
      build.data.len(),
      code_ptr,
      code_size,
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

  // Safety: `code_start` 已在上方确认 gate_allocation_data 非空（start 非 null）故为已分配可执行页
  // 基址，`add(offset)` 落在本次分配的 code 段界内（offset 取自本 build 的 label 偏移）。
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
