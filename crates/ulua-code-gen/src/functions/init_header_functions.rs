use core::mem::transmute;

use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{base_code_gen_context::BaseCodeGenContext, native_fn::GateFn},
  traits::HeaderEntryBuilder,
};

/// gate 入口（header functions）汇编的单一骨架实现。
///
/// 取代原 `init_header_functions_a_64/x_64` 两份结构同构的逐字重复代码；
/// 平台差异（builder 类型、入口发射、指令元素宽度、unwind 架构）全部经
/// `HeaderEntryBuilder` trait 单态化传入，见其两份实现。
///
/// 借用纪律：unwind 借用一律逐语句经 `unwind_builder_mut` 门面派生、随语句结束，
/// 不得跨 `code_allocator.allocate` 存活——allocate 内部会经
/// `create_block_unwind_info` 回调重访同一 unwind 对象并派生它自己的 `&mut`。
pub fn init_header_functions<B: HeaderEntryBuilder>(
  code_gen_context: &mut BaseCodeGenContext,
) -> bool {
  let mut build = B::new_for_header();

  // unwind 借用契约（非空/长寿/语句内即时消费）统一见
  // `BaseCodeGenContext::unwind_builder_mut` 门面注释。
  code_gen_context.unwind_builder_mut().start_info(B::ARCH);

  let entry_locations = build.build_header_entry(code_gen_context.unwind_builder_mut());

  build.finalize_header();

  code_gen_context.unwind_builder_mut().finish_info();

  CODEGEN_ASSERT!(build.data_bytes().is_empty());

  // cpp CodeGenX64.cpp:205-211 / CodeGenA64.cpp:321-328：gate 代码统一走
  // CodeAllocationData（移植期开关 LuauCodegenFreeBlocks 在 cpp 中已删除）。
  // `allocate` 以切片入参（len 即字节数，与两份旧实现传入的指针+长度逐值一致；
  // CODEGEN_ASSERT 已证 data 段为空，空段语义不变）。
  code_gen_context.gate_allocation_data = code_gen_context
    .code_allocator
    .allocate(build.data_bytes(), build.code_bytes());

  if code_gen_context.gate_allocation_data.start.is_null() {
    return false;
  }

  let code_start = code_gen_context.gate_allocation_data.code_start;

  // 契约同上：unwind_builder_mut 门面注释。
  code_gen_context
    .unwind_builder_mut()
    .set_begin_offset(build.header_label_offset(&entry_locations.prologue_end) as usize);

  // Safety: `code_start` 已在上方确认 gate_allocation_data 非空（start 非 null）故为
  // 已分配可执行页基址；`add(offset)` 的 offset 取自本 build 的 label 表，落在该 code
  // 段界内。`start` 标签由 build_header_entry 恒先发射（两份平台实现均在入口第一动作
  // set_label 取 `locations.start`），故地址非空，显式 `Some`；数据代码地址→`GateFn`
  // C ABI 函数指针的窄转型收口在此一次完成（不再借道 transmute 进 Option 的 null
  // niche），gate_exit 为生成码经 offset_of! 直读的跳转目标，保持裸地址。
  // 读取方 on_enter 直接类型化调用。
  unsafe {
    let gate_entry: GateFn = transmute::<*mut u8, GateFn>(
      code_start.add(build.header_label_offset(&entry_locations.start) as usize),
    );
    code_gen_context.context.gate_entry = Some(gate_entry);
    code_gen_context.context.gate_exit =
      code_start.add(build.header_label_offset(&entry_locations.epilogue_start) as usize);
  }

  true
}
