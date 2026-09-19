use alloc::vec::Vec;
use core::{ffi::c_void, ptr::write};

use ulua_vm::records::proto::Proto;

use crate::{
  records::{
    base_code_gen_context::BaseCodeGenContext, module_bind_result::ModuleBindResult,
    shared_code_allocator::SharedCodeAllocator, shared_code_gen_context::SharedCodeGenContext,
  },
  type_aliases::{
    allocation_callback::AllocationCallback, module_id::ModuleId,
    native_proto_exec_data_ptr::NativeProtoExecDataPtr,
  },
};

/// C 回调 shim：转发到 SharedCodeGenContext::try_bind_existing_module。
/// # Safety
/// `ctx` 必须指向 SharedCodeGenContext 实例（注册时保证）。
unsafe fn shared_try_bind_existing_module_shim(
  ctx: *mut BaseCodeGenContext,
  module_id: &ModuleId,
  module_protos: &[*mut Proto],
) -> Option<ModuleBindResult> {
  unsafe {
    (*(ctx as *mut SharedCodeGenContext)).try_bind_existing_module(module_id, module_protos)
  }
}

/// C 回调 shim：转发到 SharedCodeGenContext::bind_module。
/// # Safety
/// `ctx` 必须指向 SharedCodeGenContext 实例，指针参数均须满足 C++ 前置条件。
unsafe fn shared_bind_module_shim(
  ctx: *mut BaseCodeGenContext,
  module_id: &Option<ModuleId>,
  module_protos: &[*mut Proto],
  native_protos: Vec<NativeProtoExecDataPtr>,
  data: *const u8,
  data_size: usize,
  code: *const u8,
  code_size: usize,
) -> ModuleBindResult {
  unsafe {
    (*(ctx as *mut SharedCodeGenContext)).bind_module(
      module_id,
      module_protos,
      native_protos,
      data,
      data_size,
      code,
      code_size,
    )
  }
}

/// cpp SharedCodeGenContext::onCloseState（CodeGenContext.h:109 虚覆写，no-op）
/// 的分派垫片。
/// C 回调 shim：关 state 时销毁共享 context。
/// # Safety
/// `ctx` 必须指向 SharedCodeGenContext 实例（注册时保证）。
unsafe fn shared_on_close_state_shim(ctx: *mut BaseCodeGenContext) {
  unsafe {
    (*(ctx as *mut SharedCodeGenContext)).on_close_state();
  }
}

impl SharedCodeGenContext {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn shared_code_gen_context_shared_code_gen_context(
    &mut self,
    block_size: usize,
    max_total_size: usize,
    allocation_callback: *mut AllocationCallback,
    allocation_callback_context: *mut c_void,
  ) {
    let mut base = unsafe {
      BaseCodeGenContext::base_code_gen_context_base_code_gen_context(
        block_size,
        max_total_size,
        allocation_callback,
        allocation_callback_context,
      )
    };
    base.try_bind_existing_module_fn = Some(shared_try_bind_existing_module_shim);
    base.bind_module_fn = Some(shared_bind_module_shim);
    base.on_close_state_fn = Some(shared_on_close_state_shim);

    let mut shared_allocator = SharedCodeAllocator::default();
    shared_allocator.shared_code_allocator_code_allocator(&mut base.code_allocator);

    unsafe {
      write(
        self,
        SharedCodeGenContext {
          base,
          shared_allocator,
        },
      );

      // cpp 构造函数 `sharedAllocator{&codeAllocator}`（CodeGenContext.cpp:255）
      // 指向的是最终对象内的字段；Rust 端 base 先构造在栈上再 write 进来，
      // 回指针必须在搬移后重定向，否则悬空指向已失效的栈内存
      self.shared_allocator.code_allocator = &mut self.base.code_allocator;
    }
  }
}
