use alloc::vec::Vec;
use core::{
  ffi::c_void,
  ptr::{NonNull, write},
};

use ulua_vm::records::proto::Proto;

use crate::{
  enums::code_gen_compilation_result::CodeGenCompilationResult,
  functions::bind_native_protos::bind_native_protos,
  records::{
    base_code_gen_context::BaseCodeGenContext, module_bind_result::ModuleBindResult,
    shared_code_allocator::SharedCodeAllocator,
  },
  type_aliases::{
    allocation_callback::AllocationCallback, module_id::ModuleId,
    native_proto_exec_data_ptr::NativeProtoExecDataPtr,
  },
};

// 非 Clone：code-gen context 拥有 CodeAllocator（mmap 的可执行内存）
// 与共享 allocator——在 C++ 中同样不可拷贝。
// Default 仅用于构造前的占位（全 null/空容器），真实初始化由
// standalone_code_gen_context_standalone_code_gen_context 以 ptr::write 整体覆盖。
#[derive(Debug, Default)]
#[repr(C)]
pub struct StandaloneCodeGenContext {
  pub base: BaseCodeGenContext,
  pub(crate) shared_allocator: SharedCodeAllocator,
}

impl StandaloneCodeGenContext {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn bind_module(
    &mut self,
    _module_id: &Option<ModuleId>,
    module_protos: &[*mut Proto],
    native_protos: Vec<NativeProtoExecDataPtr>,
    data: *const u8,
    data_size: usize,
    code: *const u8,
    code_size: usize,
  ) -> ModuleBindResult {
    // cpp CodeGenContext.cpp:211-232 `StandaloneCodeGenContext::bindModule`：
    // 统一经 insertAnonymousNativeModule 走 CodeAllocationData（移植期开关
    // LuauCodegenFreeBlocks 在 cpp 中已删除，手工打补丁绑定的旧路径不复存在）。
    // Safety: insert_anonymous_native_module 要求 data/code 指针按给定长度可读且
    // native_protos 非空——正是本函数头 `/// # Safety` 约定由调用方满足的前提, 原样转发;
    // shared_allocator 是 self 的存活字段, 此处 &mut 独占借用。
    let module_ref = unsafe {
      self.shared_allocator.insert_anonymous_native_module(
        native_protos,
        data,
        data_size,
        code,
        code_size,
      )
    };

    // 分配失败返回空 ref；`as_ref_option` 统一收口判空与安全共享借用（ref 的计数保活）。
    let Some(native_module) = module_ref.as_ref_option() else {
      return ModuleBindResult {
        compilation_result: CodeGenCompilationResult::AllocationFailed,
        functions_bound: 0,
      };
    };
    let mut native_protos_for_bind = native_module.native_module_get_native_protos().to_vec();
    let protos_bound = bind_native_protos(module_protos, &mut native_protos_for_bind, false);
    native_module.native_module_add_refs(protos_bound as usize);

    ModuleBindResult {
      compilation_result: CodeGenCompilationResult::Success,
      functions_bound: protos_bound,
    }
  }

  pub fn on_close_state(&mut self) {
    // StandaloneCodeGenContext 由其宿主 VM 拥有，VM 销毁时
    // 本对象也随之销毁：
    // Safety: 本上下文此前经 Box 泄漏、将其唯一所有权裸指针交给所属 VM; on_close_state 仅在 VM 析构时
    // 被调用一次, 故 Box::from_raw(self) 重新取得这份独占所有权并一次性 drop/dealloc, 不存在二次释放,
    // 此时 VM 亦不再持有对该对象的其它引用, 无并存别名。
    unsafe {
      let _ = Box::from_raw(self);
    }
  }

  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn standalone_code_gen_context_standalone_code_gen_context(
    &mut self,
    block_size: usize,
    max_total_size: usize,
    allocation_callback: *mut AllocationCallback,
    allocation_callback_context: *mut c_void,
  ) {
    // Safety: 本函数是 unsafe fn，直接透传构造参数给基类构造函数；`allocation_callback`
    // 指向存活的回调、`allocation_callback_context` 为其合法裸指针，其有效性由本函数
    // 的 `# Safety` 契约（调用方保证）承担，与基类构造函数要求一致。
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

    // Safety: `self` 由 `&mut self` 提供，指向调用方持有、已按 Default 放置的占位实例，
    // `ptr::write` 用栈上构造好的 base/shared_allocator 整体覆盖该占位且不 drop 旧值
    //（Default 为全 null 占位，无资源需回收）。
    unsafe {
      write(
        self,
        StandaloneCodeGenContext {
          base,
          shared_allocator,
        },
      );
    }

    // cpp 构造函数 `sharedAllocator{&codeAllocator}` 指向的是最终对象内的字段；
    // Rust 端 base 先构造在栈上再 write 进来，回指针必须在搬移后重定向，
    // 否则悬空指向已失效的栈内存。此为裸指针字段赋值, 安全操作。
    self.shared_allocator.code_allocator = NonNull::new(&mut self.base.code_allocator);
  }

  pub fn try_bind_existing_module(
    &mut self,
    _module_id: &ModuleId,
    _module_protos: &[*mut Proto],
  ) -> Option<ModuleBindResult> {
    // StandaloneCodeGenContext 不支持共享 native 代码
    None
  }
}

/// cpp StandaloneCodeGenContext::onCloseState（CodeGenContext.h:85 虚覆写）的
/// 分派垫片，对齐 `getCodeGenContext(L)->onCloseState()` 的虚调用语义。
/// C 回调 shim：关 state 时销毁独立 context。
/// # Safety
/// `ctx` 必须指向 StandaloneCodeGenContext 实例（注册时保证）。
unsafe fn standalone_on_close_state_shim(ctx: *mut BaseCodeGenContext) {
  // Safety: `ctx` 由本 context 在构造时注册，实际指向 StandaloneCodeGenContext。
  // 该类型是 #[repr(C)] 且 `base: BaseCodeGenContext` 为首字段，基类地址与派生类
  // 地址相同，故 `ctx as *mut StandaloneCodeGenContext` 的向下转型及解引用有效，
  // 不产生越界或错类型读。
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
  // Safety: `ctx` 由本 context 注册，实际指向 StandaloneCodeGenContext；该类型为
  // #[repr(C)] 且 `base` 为首字段，基类地址即派生类地址，向下转型解引用有效。
  // `module_id`/`module_protos` 由调用方（转发本 shim 的虚表槽）保证在调用期间存活可读。
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
  // Safety: `ctx` 实际指向 StandaloneCodeGenContext（#[repr(C)]，`base` 首字段），
  // 向下转型解引用有效；`module_protos` 各指针与 `data`/`code` 及其长度由调用方保证
  // 在调用期间构成有效可读区间，满足 bind_module 自身的前置条件。
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
