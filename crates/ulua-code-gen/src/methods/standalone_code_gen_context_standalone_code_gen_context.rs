use alloc::vec::Vec;
use core::{ffi::c_void, ptr::write};

use ulua_vm::records::proto::Proto;

use crate::{
  records::{
    base_code_gen_context::BaseCodeGenContext, module_bind_result::ModuleBindResult,
    shared_code_allocator::SharedCodeAllocator,
    standalone_code_gen_context::StandaloneCodeGenContext,
  },
  type_aliases::{
    allocation_callback::AllocationCallback, module_id::ModuleId,
    native_proto_exec_data_ptr::NativeProtoExecDataPtr,
  },
};

/// cpp StandaloneCodeGenContext::onCloseState（CodeGenContext.h:85 虚覆写）的
/// 分派垫片，对齐 `getCodeGenContext(L)->onCloseState()` 的虚调用语义。
/// C 回调 shim：关 state 时销毁独立 context。
/// # Safety
/// `ctx` 必须指向 StandaloneCodeGenContext 实例（注册时保证）。
unsafe fn standalone_on_close_state_shim(ctx: *mut BaseCodeGenContext) {
  unsafe {
    (*(ctx as *mut StandaloneCodeGenContext)).on_close_state();
  }
}

/// C 回调 shim：转发到 StandaloneCodeGenContext::try_bind_existing_module。
/// # Safety
/// `ctx` 必须指向 StandaloneCodeGenContext 实例（注册时保证）。
unsafe fn standalone_try_bind_existing_module_shim(
  ctx: *mut BaseCodeGenContext,
  module_id: &ModuleId,
  module_protos: &[*mut Proto],
) -> Option<ModuleBindResult> {
  unsafe {
    (*(ctx as *mut StandaloneCodeGenContext)).try_bind_existing_module(module_id, module_protos)
  }
}

/// C 回调 shim：转发到 StandaloneCodeGenContext::bind_module。
/// # Safety
/// `ctx` 必须指向 StandaloneCodeGenContext 实例，指针参数均须满足 C++ 前置条件。
unsafe fn standalone_bind_module_shim(
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
    (*(ctx as *mut StandaloneCodeGenContext)).bind_module(
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

impl StandaloneCodeGenContext {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn standalone_code_gen_context_standalone_code_gen_context(
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
    base.try_bind_existing_module_fn = Some(standalone_try_bind_existing_module_shim);
    base.bind_module_fn = Some(standalone_bind_module_shim);
    base.on_close_state_fn = Some(standalone_on_close_state_shim);

    let mut shared_allocator = SharedCodeAllocator::default();
    shared_allocator.shared_code_allocator_code_allocator(&mut base.code_allocator);

    unsafe {
      write(
        self,
        StandaloneCodeGenContext {
          base,
          shared_allocator,
        },
      );

      // cpp 构造函数 `sharedAllocator{&codeAllocator}` 指向的是最终对象内的字段；
      // Rust 端 base 先构造在栈上再 write 进来，回指针必须在搬移后重定向，
      // 否则悬空指向已失效的栈内存
      self.shared_allocator.code_allocator = &mut self.base.code_allocator;
    }
  }
}
