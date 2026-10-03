use alloc::{boxed::Box, vec::Vec};
use core::{
  ffi::c_void,
  ptr::{NonNull, null_mut},
};

use ulua_vm::records::proto::Proto;

use crate::{
  enums::{code_gen_compilation_result::CodeGenCompilationResult, options::CodeGenContextKind},
  functions::{
    bind_native_protos::bind_native_protos, create_block_unwind_info::create_block_unwind_info,
    destroy_block_unwind_info::destroy_block_unwind_info, init_functions::init_functions,
    init_header_functions::init_header_functions, is_supported::is_supported,
  },
  macros::{
    codegen_assert::CODEGEN_ASSERT,
    codegen_target::{CODEGEN_TARGET_A64, CODEGEN_TARGET_X64},
  },
  records::{
    assembly_builder_a_64::AssemblyBuilderA64, assembly_builder_x_64::AssemblyBuilderX64,
    code_allocation_data::CodeAllocationData, code_allocator::CodeAllocator,
    module::ModuleBindResult, native_context::NativeContext, native_module::NativeModule,
    shared_code_allocator::SharedCodeAllocator, unwind_builder::UnwindBuilderImpl,
  },
  type_aliases::{
    api::{AllocationCallback, UserdataRemapperCallback},
    module_id::ModuleId,
    native_proto_exec_data_ptr::NativeProtoExecDataPtr,
  },
};

extern crate alloc;

/// cpp `BaseCodeGenContext`（CodeGenContext.h）的单一具体化实现。
///
/// cpp 用 `StandaloneCodeGenContext` / `SharedCodeGenContext` 两个子类的虚覆写区分
/// `bindModule` / `tryBindExistingModule` / `onCloseState` 行为；该类型集合编译期封闭，
/// Rust 侧收口为本结构 + `kind: CodeGenContextKind` 判别，分派点全部静态 `match`，
/// 不再有 `*mut BaseCodeGenContext` 下行转换的 fn 槽垫片，也不再有 `Box::from_raw(self)`
/// 自毁把戏（唯一一次所有权回收在 `functions::on_close_state` 的 ecb 边界上，与
/// `luau_codegen_create` 的 `Box::into_raw` 配对）。
#[derive(Debug)]
#[repr(C)]
pub struct BaseCodeGenContext {
  pub(crate) code_allocator: CodeAllocator,
  /// cpp 放在两个派生类各自的 `sharedAllocator` 成员；封闭集合单实现后归一到这里，
  /// `kind == Standalone` 时模块一律匿名插入（与派生类行为一致）。
  pub(crate) shared_allocator: SharedCodeAllocator,
  /// 上下文种类：Standalone（宿主 VM 独占、关闭时自毁）或 Shared（创建者持有、多 VM 共享）。
  pub(crate) kind: CodeGenContextKind,
  /// 本平台具体 unwind 构建器的**独占所有权**（cpp `unique_ptr<UnwindBuilder>` 成员）。
  /// `code_allocator.context` 登记的是 Box 堆块地址——Box 内容地址随堆分配固定，
  /// 不随本结构搬移而失效；字段声明序在 `code_allocator` 之后，故
  /// `code_allocator` 的 Drop（经 `destroy_block_unwind_info` 回调解引用该地址）
  /// 恒先于本 Box 释放，转手/回收对称且无悬垂窗口。
  pub(crate) unwind_builder: Box<UnwindBuilderImpl>,
  pub gate_allocation_data: CodeAllocationData,
  /// DELIBERATE DEVIATION / 保留理由：opaque 宿主 user-data，不是「缺席」语义的类型化
  /// 句柄候选。写入点唯一——`functions::set_userdata_remapper`（宿主经 C ABI 入口
  /// 传入的 `void* context`，与 `userdata_remapper: Option<UserdataRemapperCallback>`
  /// 成对登记）；交回 C ABI 的点在 `functions::userdata_remapper_wrap`——
  /// `unsafe extern "C-unwind" fn userdata_remapper_wrap(l: *mut LuaState,
  /// str: *const c_char, len: usize) -> u8` 内部调
  /// `remapper(ctx.userdata_remapping_context, str, len)`，被调方类型即
  /// `UserdataRemapperCallback = unsafe extern "C-unwind" fn(context: *mut c_void,
  /// name: *const c_char, name_length: usize) -> u8`。该函数指针槽最终被装进
  /// `global_State::ecb.gettypemapping`（VM 的 C 回调表），第一参在 ABI 上钉死为
  /// `void*`：换成 `Option<NonNull<T>>` 也必须在调用点用
  /// `map_or(null_mut(), NonNull::as_ptr)` 还原，只增样板不减 unsafe。
  /// `null_mut()` 初值对齐 cpp `CodeGenContext.h:62` `void* userdataRemappingContext
  /// = nullptr`，语义「尚未 set remapper」；此时 `userdata_remapper` 为 `None`，
  /// `userdata_remapper_wrap` 不会被装进 ecb，故该值无消费方。
  pub userdata_remapping_context: *mut c_void,
  // C++ 字段：`UserdataRemapperCallback* userdataRemapper`，其中 C++ 别名
  // 是*函数类型*，故该字段是可空函数指针。Rust 的
  // `UserdataRemapperCallback` 别名本身已是函数指针类型，因此
  // 忠实等价形式是 `Option<UserdataRemapperCallback>`（Some == 函数指针，
  // None == nullptr）。
  pub userdata_remapper: Option<UserdataRemapperCallback>,
  pub context: NativeContext,
  /// JIT call inlining 第 2 阶段：暖重编译重绑定时从 proto 名下转移出来的旧
  /// native module（逐 proto 一项，module 可重复）。proto 侧关闭回调只会归还
  /// 当前 execdata 所属的新 module，旧 module 的引用计数若不在此接手即成死账，
  /// code_allocator（第一字段，先 drop）的 live_allocations 归零断言必炸。
  /// [`Drop`] 实现在该断言前归还——此刻 VM 已关闭、无在途 native 帧，页权限
  /// 回收安全。
  pub warm_recompile_retires: Vec<*mut NativeModule>,
}

/// cpp CodeGenContext.cpp:171-174 `~BaseCodeGenContext`：上下文析构时必须归还
/// gate 代码块，否则 CodeAllocator 的 liveAllocations 计数与分配严格配对的断言
/// （移植期开关 LuauCodegenFreeBlocks 在 cpp 中已删除后恒为真）会被打破。
/// deallocate 对空 allocationStart 为空操作，故 gate 未分配时同样安全。
impl Drop for BaseCodeGenContext {
  fn drop(&mut self) {
    // 暖重编译转移的旧 module 引用先归还（release 归零即 deallocate），再进
    // 字段 drop——code_allocator 是第一字段，其 Drop 里的 live_allocations 归零
    // 断言要求此处账已清平。
    for &old in self.warm_recompile_retires.iter() {
      // Safety: 登记点（bind_native_protos 的重绑定分支）契约保证指针指向
      // 计数托管的存活 NativeModule；release 只递减原子引用计数。
      unsafe { (*old).release() };
    }
    self.warm_recompile_retires.clear();
    self.code_allocator.deallocate(self.gate_allocation_data);
  }
}

impl BaseCodeGenContext {
  /// 组装 base 与共享分配表；`kind` 静态选定原先由派生类构造器登记的虚表行为。
  ///
  /// `allocation_callback` 为宿主自定义分配回调（cpp 可空函数指针收进 `Option`，
  /// review.md §2）；`allocation_callback_context` 是交回该回调的 opaque 宿主
  /// user-data（保留理由见 `CodeAllocator::allocation_callback_context` 字段注）。
  ///
  /// 返回值的 `shared_allocator` 回指接线尚未完成：对象落位到最终存放处后、任何
  /// 使用之前必须调用一次 `pin_shared_allocator`（两个创建点均如此，见其契约）。
  pub fn new(
    kind: CodeGenContextKind,
    block_size: usize,
    max_total_size: usize,
    allocation_callback: Option<AllocationCallback>,
    allocation_callback_context: *mut c_void,
  ) -> Self {
    CODEGEN_ASSERT!(is_supported());

    let mut code_allocator = CodeAllocator::default();
    code_allocator.code_allocator_usize_usize_allocation_callback_void(
      block_size,
      max_total_size,
      allocation_callback,
      allocation_callback_context,
    );

    let mut unwind_builder = Box::new(UnwindBuilderImpl::default());
    code_allocator.context = &mut *unwind_builder as *mut UnwindBuilderImpl as *mut c_void;
    code_allocator.create_block_unwind_info = Some(create_block_unwind_info);
    code_allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info);

    let mut context = NativeContext::default();
    init_functions(&mut context);

    Self {
      code_allocator,
      shared_allocator: SharedCodeAllocator::default(),
      kind,
      unwind_builder,
      gate_allocation_data: CodeAllocationData::default(),
      userdata_remapping_context: null_mut(),
      userdata_remapper: None,
      context,
      warm_recompile_retires: Vec::new(),
    }
  }

  /// cpp 构造函数 `sharedAllocator{&codeAllocator}`（CodeGenContext.cpp:255）的回指接线：
  /// 共享分配器指向的是最终对象内的 `code_allocator` 字段，必须在对象落位后、使用前完成。
  ///
  /// 契约：调用之后 `self` 不得再被搬移（回指针指向自身字段），直至销毁。
  /// `pin_shared_allocator` 的两个调用点（standalone 的 Box 落位、shared 的堆块 write
  /// 落位）之后对象均不再移动。
  pub fn pin_shared_allocator(&mut self) {
    self.shared_allocator.code_allocator = NonNull::new(&mut self.code_allocator);
  }

  /// unwind 构建器的可变视图门面（借用纪律的单一收口点）：`init_header_functions`
  /// 等调用方经此派生借用，不再持有裸指针。
  ///
  /// 借用不得跨 `code_allocator.allocate` 存活——allocate 内部会经
  /// `create_block_unwind_info` 回调（context 参数即本 Box 堆块地址）重访同一
  /// unwind 对象并派生它自己的 `&mut`；本门面返回的 `&mut` 与回调侧重访之间存在
  /// 帧号依赖，调用方须逐语句取用、随语句结束（见 `functions::init_header_functions`
  /// 的借用纪律注）。
  pub fn unwind_builder_mut(&mut self) -> &mut UnwindBuilderImpl {
    &mut self.unwind_builder
  }

  /// 绑定成功后的公共收尾（原 shared/standalone 两份逐字重复的尾巴收口为单一实现）：
  /// 把 protos 重绑到模块的原生函数表并累计引用计数，对齐 cpp
  /// `NativeModule::bindNativeProtos` + `addRefs`。
  fn bind_protos_to_module(
    native_module: &NativeModule,
    module_protos: &[*mut Proto],
    rebind_retire: bool,
    retires: &mut Vec<*mut NativeModule>,
  ) -> ModuleBindResult {
    let mut native_protos = native_module.native_protos.to_vec();
    let protos_bound =
      bind_native_protos(module_protos, &mut native_protos, rebind_retire, retires);
    native_module.native_module_add_refs(protos_bound as usize);

    ModuleBindResult {
      compilation_result: CodeGenCompilationResult::Success,
      functions_bound: protos_bound,
    }
  }

  /// cpp `StandaloneCodeGenContext::bindModule`（CodeGenContext.cpp:211-232）与
  /// `SharedCodeGenContext::bindModule`（:241-266）的单一实现，按 `kind` 静态分支：
  /// shared 且带 module_id 时按 id 登记复用（`get_or_insert_native_module`），
  /// 其余（anonymous 与 standalone 忽略 module_id，统一走匿名插入
  /// `insert_anonymous_native_module`——移植期开关 LuauCodegenFreeBlocks 在 cpp 中已删除，
  /// 手工打补丁绑定的旧路径不复存在）。
  ///
  /// `data`/`code` 以切片表达长度界内性（原 `*const u8` + 长度对随切片化删除）。
  /// `rebind_retire`（同步暖重编译专用）：重绑定的 proto 名下旧 execdata 属旧
  /// module，其引用逐 proto 转移进 `warm_recompile_retires`（关闭链统一归还，
  /// 见字段注）。
  pub fn bind_module(
    &mut self,
    module_id: &Option<ModuleId>,
    module_protos: &[*mut Proto],
    native_protos: Vec<NativeProtoExecDataPtr>,
    data: &[u8],
    code: &[u8],
    rebind_retire: bool,
  ) -> ModuleBindResult {
    let module_ref = match (self.kind, module_id) {
      (CodeGenContextKind::Shared, Some(module_id)) => {
        self
          .shared_allocator
          .get_or_insert_native_module(module_id, native_protos, data, code)
          .0
      }
      _ => self
        .shared_allocator
        .insert_anonymous_native_module(native_protos, data, code),
    };

    // 分配失败返回空 ref；`as_ref_option` 统一收口判空与安全共享借用（ref 的计数保活）。
    let Some(native_module) = module_ref.as_ref_option() else {
      return ModuleBindResult {
        compilation_result: CodeGenCompilationResult::AllocationFailed,
        functions_bound: 0,
      };
    };

    Self::bind_protos_to_module(
      native_module,
      module_protos,
      rebind_retire,
      &mut self.warm_recompile_retires,
    )
  }

  /// cpp `SharedCodeGenContext::tryBindExistingModule`（命中已登记模块则重绑）与
  /// `StandaloneCodeGenContext::tryBindExistingModule`（恒 nullptr，不支持共享 native 代码）
  /// 的单一实现，按 `kind` 静态分支。
  pub fn try_bind_existing_module(
    &mut self,
    module_id: &ModuleId,
    module_protos: &[*mut Proto],
  ) -> Option<ModuleBindResult> {
    if self.kind != CodeGenContextKind::Shared {
      return None;
    }

    // 空 ref（未命中已登记模块）与解引用统一收口到 `as_ref_option`（安全视图）。
    let native_module_ref = self.shared_allocator.try_get_native_module(module_id);
    let native_module = native_module_ref.as_ref_option()?;

    Some(Self::bind_protos_to_module(
      native_module,
      module_protos,
      false,
      &mut self.warm_recompile_retires,
    ))
  }

  pub fn init_header_functions(&mut self) -> bool {
    if CODEGEN_TARGET_X64 && !init_header_functions::<AssemblyBuilderX64>(self) {
      return false;
    }

    if CODEGEN_TARGET_A64 && !init_header_functions::<AssemblyBuilderA64>(self) {
      return false;
    }

    true
  }
}
