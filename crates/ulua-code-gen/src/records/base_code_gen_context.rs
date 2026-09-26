use alloc::{boxed::Box, vec::Vec};
use core::{ffi::c_void, ptr::null_mut};

use ulua_vm::records::proto::Proto;

#[cfg(not(target_os = "windows"))]
use crate::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;
use crate::{
  functions::{
    create_block_unwind_info::create_block_unwind_info,
    destroy_block_unwind_info::destroy_block_unwind_info, init_functions::init_functions,
    init_header_functions_code_gen_a_64::init_header_functions as init_header_functions_a64,
    init_header_functions_code_gen_x_64::init_header_functions as init_header_functions_x64,
    is_supported::is_supported,
  },
  macros::{
    codegen_assert::CODEGEN_ASSERT, codegen_target_a_64::CODEGEN_TARGET_A64,
    codegen_target_x_64::CODEGEN_TARGET_X64,
  },
  records::{
    code_allocation_data::CodeAllocationData, code_allocator::CodeAllocator,
    module_bind_result::ModuleBindResult, native_context::NativeContext,
    unwind_builder::UnwindBuilder,
  },
  type_aliases::{
    allocation_callback::AllocationCallback, module_id::ModuleId,
    native_proto_exec_data_ptr::NativeProtoExecDataPtr,
    userdata_remapper_callback::UserdataRemapperCallback,
  },
};

extern crate alloc;

/// 绑定模块回调签名(字段过长,提炼别名)
///
/// # Safety
///
/// 经该指针调用时: 首参必须指向存活且实际类型与实现方期望一致的 `BaseCodeGenContext`;
/// `protos` 各 `*mut Proto` 与 `Vec<NativeProtoExecDataPtr>` 元素须为该上下文在调用期间
/// 持有的有效指针; 两处 `*const u8`/`usize` 必须各自构成有效可读字节区间。
pub type BindModuleFn = unsafe fn(
  *mut BaseCodeGenContext,
  &Option<ModuleId>,
  &[*mut Proto],
  Vec<NativeProtoExecDataPtr>,
  *const u8,
  usize,
  *const u8,
  usize,
) -> ModuleBindResult;

/// 尝试绑定既有模块的回调签名(字段过长,提炼别名)
///
/// # Safety
///
/// 经该指针调用时: 首参必须指向存活且实际类型与实现方期望一致的 `BaseCodeGenContext`;
/// `protos` 各 `*mut Proto` 须在调用期间有效可读。
pub(crate) type TryBindExistingModuleFn =
  unsafe fn(*mut BaseCodeGenContext, &ModuleId, &[*mut Proto]) -> Option<ModuleBindResult>;

/// cpp `virtual void onCloseState() noexcept` 的分派槽位(CodeGenContext.h:54)
///
/// # Safety
///
/// 经该指针调用时: 实参必须指向存活且实际类型与实现方期望一致的 `BaseCodeGenContext`。
pub(crate) type OnCloseStateFn = unsafe fn(*mut BaseCodeGenContext);

#[derive(Debug, Default)]
#[repr(C)]
pub struct BaseCodeGenContext {
  pub(crate) code_allocator: CodeAllocator,
  pub try_bind_existing_module_fn: Option<TryBindExistingModuleFn>,
  pub bind_module_fn: Option<BindModuleFn>,
  pub(crate) on_close_state_fn: Option<OnCloseStateFn>,
  pub(crate) unwind_builder: *mut UnwindBuilder,
  pub gate_allocation_data: CodeAllocationData,
  pub userdata_remapping_context: *mut c_void,
  // C++ 字段：`UserdataRemapperCallback* userdataRemapper`，其中 C++ 别名
  // 是*函数类型*，故该字段是可空函数指针。Rust 的
  // `UserdataRemapperCallback` 别名本身已是函数指针类型，因此
  // 忠实等价形式是 `Option<UserdataRemapperCallback>`（Some == 函数指针，
  // None == nullptr）。
  pub userdata_remapper: Option<UserdataRemapperCallback>,
  pub context: NativeContext,
}

/// cpp CodeGenContext.cpp:171-174 `~BaseCodeGenContext`：上下文析构时必须归还
/// gate 代码块，否则 CodeAllocator 的 liveAllocations 计数与分配严格配对的断言
/// （移植期开关 LuauCodegenFreeBlocks 在 cpp 中已删除后恒为真）会被打破。
/// deallocate 对空 allocationStart 为空操作，故 gate 未分配时同样安全。
impl Drop for BaseCodeGenContext {
  fn drop(&mut self) {
    self.code_allocator.deallocate(self.gate_allocation_data);
  }
}

impl BaseCodeGenContext {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn base_code_gen_context_base_code_gen_context(
    block_size: usize,
    max_total_size: usize,
    allocation_callback: *mut AllocationCallback,
    allocation_callback_context: *mut c_void,
  ) -> Self {
    CODEGEN_ASSERT!(is_supported());

    let mut code_allocator = CodeAllocator::default();
    let callback = if allocation_callback.is_null() {
      None
    } else {
      // Safety: allocation_callback 已判空非 null，且契约保证它指向一个存活的 AllocationCallback
      // （C fn 指针、Copy）；*allocation_callback 读出该指针值本身（拷贝指针、不解引用目标），对齐一致。
      Some(unsafe { *allocation_callback })
    };
    code_allocator.code_allocator_usize_usize_allocation_callback_void(
      block_size,
      max_total_size,
      callback,
      allocation_callback_context,
    );

    let unwind_builder: *mut UnwindBuilder = make_unwind_builder();

    code_allocator.context = unwind_builder.cast();
    code_allocator.create_block_unwind_info = Some(create_block_unwind_info);
    code_allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info);

    let mut context = NativeContext::default();
    init_functions(&mut context);

    Self {
      code_allocator,
      try_bind_existing_module_fn: None,
      bind_module_fn: None,
      on_close_state_fn: None,
      unwind_builder,
      gate_allocation_data: CodeAllocationData::default(),
      userdata_remapping_context: null_mut(),
      userdata_remapper: None,
      context,
    }
  }

  pub fn init_header_functions(&mut self) -> bool {
    if CODEGEN_TARGET_X64 && !init_header_functions_x64(self) {
      return false;
    }

    if CODEGEN_TARGET_A64 && !init_header_functions_a64(self) {
      return false;
    }

    true
  }
}

#[cfg(target_os = "windows")]
fn make_unwind_builder() -> *mut UnwindBuilder {
  let builder = Box::new(crate::records::unwind_builder_win::UnwindBuilderWin::default());
  Box::into_raw(builder).cast()
}

#[cfg(not(target_os = "windows"))]
fn make_unwind_builder() -> *mut UnwindBuilder {
  let builder = Box::new(UnwindBuilderDwarf2::default());
  Box::into_raw(builder).cast()
}
