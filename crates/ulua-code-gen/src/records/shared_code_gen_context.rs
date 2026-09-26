use alloc::vec::Vec;
use core::{
  ffi::c_void,
  ptr::{NonNull, write},
};

use ulua_vm::records::proto::Proto;

use crate::{
  enums::code_gen_compilation_result::CodeGenCompilationResult,
  functions::{
    bind_native_protos::bind_native_protos,
    get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header,
  },
  records::{
    base_code_gen_context::BaseCodeGenContext, module_bind_result::ModuleBindResult,
    native_module_ref::NativeModuleRef, shared_code_allocator::SharedCodeAllocator,
  },
  type_aliases::{
    allocation_callback::AllocationCallback, module_id::ModuleId,
    native_proto_exec_data_ptr::NativeProtoExecDataPtr,
  },
};

#[derive(Debug)]
#[repr(C)]
pub struct SharedCodeGenContext {
  pub base: BaseCodeGenContext,
  pub(crate) shared_allocator: SharedCodeAllocator,
}

impl SharedCodeGenContext {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn bind_module(
    &mut self,
    module_id: &Option<ModuleId>,
    module_protos: &[*mut Proto],
    native_protos: Vec<NativeProtoExecDataPtr>,
    data: *const u8,
    data_size: usize,
    code: *const u8,
    code_size: usize,
  ) -> ModuleBindResult {
    let (native_module_ref, _inserted): (NativeModuleRef, bool) = if let Some(module_id) = module_id
    {
      // Safety: get_or_insert_native_module 的前置条件即 data/data_size 与 code/code_size 描述可读
      // 缓冲、native_protos 为 codegen 产出的非空 execdata 指针; 这恰是本函数头 `/// # Safety`
      // 约定由调用方保证的参数, 此处原样转发。shared_allocator 是 self 的存活字段且以 &mut 独占借用。
      unsafe {
        self.shared_allocator.get_or_insert_native_module(
          module_id,
          native_protos,
          data,
          data_size,
          code,
          code_size,
        )
      }
    } else {
      (
        // Safety: 同上——insert_anonymous_native_module 的要求亦为 data/code 指针可读且长度有效、
        // native_protos 非空, 均由本函数 `/// # Safety` 约定的调用方前提原样转发; self.shared_allocator
        // 为存活字段且此处 &mut 独占。
        unsafe {
          self.shared_allocator.insert_anonymous_native_module(
            native_protos,
            data,
            data_size,
            code,
            code_size,
          )
        },
        true,
      )
    };

    // 分配失败返回空 ref；`as_ref_option` 统一收口判空与安全共享借用（ref 的计数保活）。
    let Some(native_module) = native_module_ref.as_ref_option() else {
      return ModuleBindResult {
        compilation_result: CodeGenCompilationResult::AllocationFailed,
        functions_bound: 0,
      };
    };
    let mut native_protos = native_module.native_module_get_native_protos().to_vec();
    let protos_bound = bind_native_protos(module_protos, &mut native_protos, false);
    native_module.native_module_add_refs(protos_bound as usize);

    ModuleBindResult {
      compilation_result: CodeGenCompilationResult::Success,
      functions_bound: protos_bound,
    }
  }

  pub fn on_close_state(&mut self) {
    // SharedCodeGenContext 的生命周期与使用它的 VM 分开管理。
    // VM 销毁时，这里不需要做任何事：
    // 无需处理。
  }

  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn on_destroy_function(execdata: *mut c_void) {
    // Safety: execdata 由 VM 在销毁 codegen 生成函数的 execdata 时传入(本函数头 `/// # Safety`
    // 约定), 指向存活的 NativeProtoExecData 缓冲; 依"头部先于 instruction_offsets 数组"布局,
    // get_native_proto_exec_data_header 由有效指针算得同分配内非空且对齐的头部地址, 故派生 & 合法。
    let header = unsafe { &*get_native_proto_exec_data_header(execdata as *const u32) };
    // Safety: 共享 codegen 的 execdata 在 NativeModule::bind_native_protos/rebind_header_module_pointers
    // 中必已填入所属模块地址, 故 native_module 非空; release() 只在 &self 上递减
    // 原子引用计数, 所指 NativeModule 由计数托管存活, 无并存别名冲突。
    unsafe {
      header
        .native_module
        .as_ref()
        .expect("native_module 恒为 Some：绑定模块后才分配 execdata，销毁前必已填入所属模块")
        .release();
    }
  }

  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn shared_code_gen_context_shared_code_gen_context(
    &mut self,
    block_size: usize,
    max_total_size: usize,
    allocation_callback: *mut AllocationCallback,
    allocation_callback_context: *mut c_void,
  ) {
    // Safety: 本函数头 `/// # Safety` 已约定 allocation_callback(可空, 构造器内判空)与
    // allocation_callback_context 的合法性由调用方(create_shared_code_gen_context*)提供;
    // block_size/max_total_size 来自全局配置, 满足 CodeAllocator 对块尺寸的要求。
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

    // Safety: self 为 &mut 独占的已对齐存储; ptr::write 只做按位覆盖、不 drop 旧值, 对
    // create_shared_code_gen_context 中 alloc() 出的未初始化内存合法。栈上 base/shared_allocator
    // 以移动方式进入 write 的源值, 其 Drop(CodeAllocator::destroy / 断言)不会在栈地址上运行,
    // 最终在堆对象上就地执行。
    unsafe {
      write(
        self,
        SharedCodeGenContext {
          base,
          shared_allocator,
        },
      );
    }

    // cpp 构造函数 `sharedAllocator{&codeAllocator}`（CodeGenContext.cpp:255）
    // 指向的是最终对象内的字段；Rust 端 base 先构造在栈上再 write 进来，
    // 回指针必须在搬移后重定向，否则悬空指向已失效的栈内存。
    // write 后对象地址即最终堆地址, 此重定向消掉栈悬垂; 裸指针字段赋值, 安全操作。
    self.shared_allocator.code_allocator = NonNull::new(&mut self.base.code_allocator);
  }

  pub fn try_bind_existing_module(
    &mut self,
    module_id: &ModuleId,
    module_protos: &[*mut Proto],
  ) -> Option<ModuleBindResult> {
    // 空 ref（未命中已登记模块）与解引用统一收口到 `as_ref_option`（安全视图）。
    let native_module_ref = self.shared_allocator.try_get_native_module(module_id);
    let native_module = native_module_ref.as_ref_option()?;
    let mut native_protos = native_module.native_module_get_native_protos().to_vec();
    let protos_bound = bind_native_protos(module_protos, &mut native_protos, false);
    native_module.native_module_add_refs(protos_bound as usize);

    Some(ModuleBindResult {
      compilation_result: CodeGenCompilationResult::Success,
      functions_bound: protos_bound,
    })
  }
}

/// C 回调 shim：转发到 SharedCodeGenContext::try_bind_existing_module。
/// # Safety
/// `ctx` 必须指向 SharedCodeGenContext 实例（注册时保证）。
unsafe fn shared_try_bind_existing_module_shim(
  ctx: *mut BaseCodeGenContext,
  module_id: &ModuleId,
  module_protos: &[*mut Proto],
) -> Option<ModuleBindResult> {
  // Safety: 本 shim 仅由本构造器登记进 SharedCodeGenContext.base 的回调槽, 分派方
  // (compile_internal) 传入的 ctx 即 context 的 base 字段指针; SharedCodeGenContext 为
  // #[repr(C)] 且 base 是首字段, 基址重合故 ctx as *mut SharedCodeGenContext 回到原对象,
  // 所指内存在单次回调期间存活且无并存 &mut 别名。
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
  // Safety: 同 shared_try_bind_existing_module_shim——本 shim 只在 SharedCodeGenContext 构造时
  // 注册, ctx 必指向其 #[repr(C)] 首字段 base, 反推整对象指针合法; data/code 等指针参数的有效性
  // 由分派链顶层 compile 入口按 bind_module 的 `/// # Safety` 契约提供, 此处原样转发。
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
  // Safety: 本 shim 仅由 SharedCodeGenContext 构造器注册, ctx 为指向其 #[repr(C)] 首字段
  // base 的指针, 基址重合故回转整对象指针有效; 调用发生在 state 关闭时、context 尚未释放前。
  unsafe {
    (*(ctx as *mut SharedCodeGenContext)).on_close_state();
  }
}
