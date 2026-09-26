use alloc::{boxed::Box, collections::BTreeMap, vec::Vec};
use core::{
  ptr::NonNull,
  sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    code_allocator::CodeAllocator, native_module::NativeModule, native_module_ref::NativeModuleRef,
  },
  type_aliases::{module_id::ModuleId, native_proto_exec_data_ptr::NativeProtoExecDataPtr},
};

#[derive(Debug)]
pub struct SharedCodeAllocator {
  pub(crate) identified_modules: BTreeMap<ModuleId, Box<NativeModule>>,
  pub(crate) anonymous_module_count: AtomicUsize,
  /// 所属代码分配器（由外层 codegen context 构造点接线；`None` 仅是 Default 占位态）。
  pub(crate) code_allocator: Option<NonNull<CodeAllocator>>,
}

impl SharedCodeAllocator {
  /// 独占访问接线好的分配器。`None` 只可能出现在从未完成构造的 Default 占位实例上。
  pub(crate) fn allocator(&mut self) -> &mut CodeAllocator {
    let mut code_allocator = self
      .code_allocator
      .expect("code_allocator 恒为 Some：构造点接线，分配器比共享分配器长寿");
    // Safety: Some 分支指针由构造点以 `&mut CodeAllocator`（非空、对齐、独占）登记，
    // 且分配器寿命覆盖 self，此处 &mut self 已保证对共享分配器的独占访问。
    unsafe { code_allocator.as_mut() }
  }

  pub fn erase_native_module_if_unreferenced(&mut self, native_module: &NativeModule) {
    self.shared_code_allocator_erase_native_module_if_unreferenced(native_module);
  }

  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn get_or_insert_native_module(
    &mut self,
    module_id: &ModuleId,
    native_protos: Vec<NativeProtoExecDataPtr>,
    data: *const u8,
    data_size: usize,
    code: *const u8,
    code_size: usize,
  ) -> (NativeModuleRef, bool) {
    // 契约（贯穿下述各 unsafe 块）：data/code 指针的有效性由本函数头 `/// # Safety` 约定
    // 调用方保证, 仅按长度转发给 allocator().allocate, 其 result.start 先判空再
    // 使用; `self as *mut _` 登记为模块的所属分配器, 分配器寿命覆盖其持有的模块。Box::new 后先
    // rebind_header_module_pointers 修正头指针, native_module 地址取自存活 Box 非空;
    // 所有权移交 identified_modules 后仍比返回的 ref 长寿(refcount 托管删除时机)。
    let existing_module = self.try_get_native_module_with_lock_held(module_id);
    if !existing_module.native_module_ref_empty() {
      return (existing_module, false);
    }

    // cpp SharedCodeAllocator.cpp:228-250 `getOrInsertNativeModule`：
    // 分配统一走 CodeAllocationData（移植期开关 LuauCodegenFreeBlocks
    // 在 cpp 中已删除，旧 allocate 出参路径不复存在）。
    // Safety: 见上; allocate 为 unsafe fn, 入参有效性由函数头契约承担。
    let result = unsafe { self.allocator().allocate(data, data_size, code, code_size) };

    if result.start.is_null() {
      return (NativeModuleRef::default(), false);
    }

    let mut native_module = Box::new(
                  NativeModule::native_module_shared_code_allocator_optional_module_id_code_allocation_data_vector_native_proto_exec_data_ptr(
                      self as *mut SharedCodeAllocator,
                      &Some(*module_id),
                      result,
                      native_protos,
                  ),
              );
    native_module.rebind_header_module_pointers();
    let native_module_ptr: *const NativeModule = &*native_module;
    self.identified_modules.insert(*module_id, native_module);

    // Safety: native_module_ptr 源自存活 Box（已入 identified_modules, refcount 保活）,
    // 非空有效, 满足构造器对非空指针只做原子 add_ref 的前提。
    (
      unsafe { NativeModuleRef::native_module_ref_native_module(native_module_ptr) },
      true,
    )
  }

  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn insert_anonymous_native_module(
    &mut self,
    native_protos: Vec<NativeProtoExecDataPtr>,
    data: *const u8,
    data_size: usize,
    code: *const u8,
    code_size: usize,
  ) -> NativeModuleRef {
    // Safety: 纯转发, 参数与前置条件与本函数头 `/// # Safety` 对被调 unsafe fn 的要求完全一致。
    unsafe {
      self.shared_code_allocator_insert_anonymous_native_module(
        native_protos,
        data,
        data_size,
        code,
        code_size,
      )
    }
  }

  pub fn shared_code_allocator_code_allocator(&mut self, code_allocator: *mut CodeAllocator) {
    self.identified_modules.clear();
    self.anonymous_module_count = AtomicUsize::new(0);
    self.code_allocator = NonNull::new(code_allocator);
  }

  pub fn try_get_native_module(&self, module_id: &ModuleId) -> NativeModuleRef {
    self.try_get_native_module_with_lock_held(module_id)
  }

  pub fn try_get_native_module_with_lock_held(&self, module_id: &ModuleId) -> NativeModuleRef {
    match self.identified_modules.get(module_id) {
      // Safety: Some 分支中 &**native_module 由存活 Box 派生, 非空且对齐; 构造器对非空指针只做
      // 原子 add_ref, 返回的 ref 由此持有一份额外引用, 保证所指模块不先于 ref 失效。
      Some(native_module) => unsafe {
        NativeModuleRef::native_module_ref_native_module(&**native_module)
      },
      None => NativeModuleRef::default(),
    }
  }

  pub fn shared_code_allocator_erase_native_module_if_unreferenced(
    &mut self,
    native_module: &NativeModule,
  ) {
    if native_module.native_module_get_refcount() != 0 {
      return;
    }

    // cpp SharedCodeAllocator.cpp:284：归还是无条件的（移植期开关
    // LuauCodegenFreeBlocks 在 cpp 中已删除）。
    // Safety: code_allocator 为构造期接线、非空且比 self 长寿的分配器(allocator() 内论证), 归还
    // 该模块独占的代码分配数据(CodeAllocationData 此刻仍由 native_module 持有、有效)。前置是本函数开头
    // 已确认 refcount==0, 即无任何存活持有者, 故不存在对同一分配器/同一模块的并发写或二次归还。
    self
      .allocator()
      .deallocate(native_module.native_module_get_code_allocation_data());

    if let Some(module_id) = native_module.native_module_get_module_id() {
      let removed = self.identified_modules.remove(module_id);
      CODEGEN_ASSERT!(removed.is_some());
    } else {
      CODEGEN_ASSERT!(self.anonymous_module_count.fetch_sub(1, Ordering::Relaxed) != 0);
      // Safety: 匿名模块在 insert_anonymous_native_module 里由 Box::new→Box::into_raw 泄漏, 其唯一所有权移交
      // NativeModuleRef(引用计数托管)。执行到此处的前提是函数开头已判定 refcount==0(且调用方 release() 正是
      // 在 SeqCst 递减至 0 后转入), 即无任何存活的 NativeModuleRef 指向它; native_module 的地址正等于当初
      // Box::into_raw 的返回值, 故 Box::from_raw 精确回收那个泄漏的 Box 并析构一次(侵入式引用计数的 dealloc,
      // 语义同 Rc::drop), 不产生第二所有者/双 drop, 亦非悬垂; release() 在调用本函数后即返回、不再使用 self。
      unsafe {
        drop(Box::from_raw(
          native_module as *const NativeModule as *mut NativeModule,
        ));
      }
    }
  }

  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn shared_code_allocator_insert_anonymous_native_module(
    &mut self,
    native_protos: Vec<NativeProtoExecDataPtr>,
    data: *const u8,
    data_size: usize,
    code: *const u8,
    code_size: usize,
  ) -> NativeModuleRef {
    // 契约（贯穿下述各 unsafe 块）：本函数为 unsafe fn, data/code 的有效性由函数头
    // `/// # Safety` 约定的调用方前提保证, 且仅按 data_size/code_size 原样转发给
    // (*self.code_allocator).allocate, 其返回的 result.start 先经 is_null 判定再使用。
    // self.code_allocator 为构造点接线、非空且比 self 长寿的分配器裸指针。
    // `self as *mut _` 被登记为新模块的所属分配器, 而分配器寿命覆盖其持有的每个模块。
    // Box::new 后先 rebind_header_module_pointers(&mut), 再 Box::into_raw 泄漏 Box 使
    // native_module_ptr 成为非空有效指针, 其唯一所有权移交返回的 NativeModuleRef
    // (引用计数托管, release 时经分配器释放)。
    // cpp SharedCodeAllocator.cpp:253-269 `insertAnonymousNativeModule`：
    // 分配统一走 CodeAllocationData（移植期开关 LuauCodegenFreeBlocks
    // 在 cpp 中已删除，旧 allocate 出参路径不复存在）。
    // Safety: 见上; 裸指针解引用 + allocate 为 unsafe fn, 入参按函数头契约原样转发。
    let result = unsafe { self.allocator().allocate(data, data_size, code, code_size) };

    if result.start.is_null() {
      return NativeModuleRef::default();
    }

    let mut native_module = Box::new(
                  NativeModule::native_module_shared_code_allocator_optional_module_id_code_allocation_data_vector_native_proto_exec_data_ptr(
                      self as *mut SharedCodeAllocator,
                      &None,
                      result,
                      native_protos,
                  ),
              );

    native_module.rebind_header_module_pointers();
    let native_module_ptr = Box::into_raw(native_module);
    self.anonymous_module_count.fetch_add(1, Ordering::Relaxed);

    // Safety: 见上; into_raw 泄漏 Box 后指针非空有效、所有权已由 ref 的引用计数托管。
    unsafe { NativeModuleRef::native_module_ref_native_module(native_module_ptr) }
  }
}

impl Default for SharedCodeAllocator {
  fn default() -> Self {
    Self {
      identified_modules: BTreeMap::new(),
      anonymous_module_count: AtomicUsize::new(0),
      code_allocator: None,
    }
  }
}

impl Drop for SharedCodeAllocator {
  fn drop(&mut self) {
    CODEGEN_ASSERT!(self.identified_modules.is_empty());
    CODEGEN_ASSERT!(self.anonymous_module_count.load(Ordering::Relaxed) == 0);
  }
}
