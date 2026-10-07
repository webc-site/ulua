use alloc::vec::Vec;
use core::{
  ptr::{NonNull, null},
  sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{code_allocation_data::CodeAllocationData, shared_code_allocator::SharedCodeAllocator},
  type_aliases::{
    module_id::ModuleId,
    native_proto_exec_data_ptr::{NativeProtoExecDataHeaderExt, NativeProtoExecDataPtr},
  },
};

#[derive(Debug)]
pub struct NativeModule {
  pub(crate) refcount: AtomicUsize,
  /// 所属共享分配器（构造点接线，寿命覆盖本模块；`None` 仅占位态不存在于存活模块）。
  pub(crate) allocator: Option<NonNull<SharedCodeAllocator>>,
  pub(crate) module_id: Option<ModuleId>,
  pub(crate) code_allocation_data: CodeAllocationData,
  pub(crate) native_protos: Vec<NativeProtoExecDataPtr>,
}

impl NativeModule {
  /// cpp SharedCodeAllocator.cpp:42-69 `NativeModule` ctor：模块唯一以
  /// `CodeAllocationData` 描述其代码块（移植期开关 LuauCodegenFreeBlocks
  /// 在 cpp 中已删除，旧的裸 `moduleBaseAddress` 构造不复存在）。
  pub fn native_module_shared_code_allocator_optional_module_id_code_allocation_data_vector_native_proto_exec_data_ptr(
    allocator: *mut SharedCodeAllocator,
    module_id: &Option<ModuleId>,
    code_allocation_data: CodeAllocationData,
    native_protos: Vec<NativeProtoExecDataPtr>,
  ) -> Self {
    CODEGEN_ASSERT!(!code_allocation_data.start.is_null());

    let mut result = Self {
      refcount: AtomicUsize::new(0),
      allocator: NonNull::new(allocator),
      module_id: *module_id,
      code_allocation_data,
      native_protos,
    };

    result.bind_native_protos(code_allocation_data.code_start);
    result
  }

  fn bind_native_protos(&mut self, code_base: *const u8) {
    // `code_base` 来自构造断言非空的 allocation code_start，`.add(entry_offset)` 落在该块内；
    // 写入的两字段位于 execdata 外部堆分配（非 self 内），与 `&mut self` 借用内存不相交。
    // header 读写一律经 `NativeProtoExecDataHeaderExt` 门面（unsafe 收口见该 trait 契约）。
    // `&mut self` 派生的 `NonNull` 由引用不变量保证非空，临时借用随语句结束（后续循环
    // 再独占 `self.native_protos`），无需裸指针中转。
    let native_module = NonNull::from(&mut *self);

    for native_proto in self.native_protos.iter_mut() {
      let header = native_proto.header_mut();
      header.native_module = Some(native_module);
      // Safety: code_base 来自构造断言非空的 allocation code_start，entry_offset 为该
      // 代码块内的合法偏移，`add` 落在块界内；仅地址算术，不解引用。
      header.entry_offset_or_address =
        unsafe { code_base.add(header.entry_offset_or_address as usize) };
    }

    self
      .native_protos
      .sort_by_key(|native_proto| native_proto.exec_data_bytecode_id());

    for pair in self.native_protos.windows(2) {
      CODEGEN_ASSERT!(pair[0].exec_data_bytecode_id() != pair[1].exec_data_bytecode_id());
    }
  }

  /// 构造函数运行在栈上临时对象上，header.native_module 捕获的是临时地址；
  /// Box 化搬到最终堆地址后必须调用此方法重绑，否则 release 会作用到
  /// 已失效的栈内存（cpp `make_unique<NativeModule>` 直接在最终地址构造）。
  pub(crate) fn rebind_header_module_pointers(&mut self) {
    // `&mut self` 派生 `NonNull`：非空由引用不变量给出，临时借用随语句结束。
    let native_module = NonNull::from(&mut *self);

    for native_proto in self.native_protos.iter_mut() {
      // 把 native_module 改写为 self 的最终(堆)地址，修正构造期捕获的栈临时地址；
      // header 写经 `header_mut` 门面（独占借用随 `&mut self` 存活，无别名冲突）。
      native_proto.header_mut().native_module = Some(native_module);
    }
  }

  pub fn native_module_add_ref(&self) -> usize {
    self.refcount.fetch_add(1, Ordering::Relaxed) + 1
  }

  pub fn native_module_add_refs(&self, count: usize) -> usize {
    self.refcount.fetch_add(count, Ordering::Relaxed) + count
  }

  /// cpp SharedCodeAllocator.cpp:110-113 `getModuleBaseAddress`：直接返回
  /// 分配数据里的代码起始地址（移植期开关 LuauCodegenFreeBlocks 在 cpp 中
  /// 已删除，旧的 moduleBaseAddress 字段旁路不复存在）。
  /// 薄字段读取 `code_allocation_data`/`module_id`/`native_protos` 已字段化（调用点
  /// 直读 pub(crate) 字段），本方法因 ulua-unit-test 外部消费保留 pub。
  pub fn native_module_get_module_base_address(&self) -> *const u8 {
    self.code_allocation_data.code_start
  }

  /// 原子计数读取不可字段化（refcount 为 AtomicUsize），外部测试消费保留 pub。
  pub fn native_module_get_refcount(&self) -> usize {
    self.refcount.load(Ordering::Relaxed)
  }

  pub fn release(&self) -> usize {
    let new_refcount = self.refcount.fetch_sub(1, Ordering::SeqCst).wrapping_sub(1);
    if new_refcount != 0 {
      return new_refcount;
    }

    // Safety: allocator 是模块构造点接线的所属代码分配器（Some 分支由 NonNull 保证非空，
    // 比本 NativeModule 长寿）。进入此分支意味着 SeqCst 递减后 new_refcount==0, 即本模块已无任何
    // 持有者, erase 可安全接管其生命周期; 调用以 self 的指针/标识为键在分配器表中定位并移除该
    // 模块, 不对 *self 建立并存的 &mut 别名。
    unsafe {
      self
        .allocator
        .expect("allocator 恒为 Some：模块由共享分配器构造点接线")
        .as_mut()
    }
    .erase_native_module_if_unreferenced(self);

    0
  }

  pub fn native_module_try_get_native_proto(&self, bytecode_id: u32) -> *const u32 {
    let mut lo = 0usize;
    let mut hi = self.native_protos.len();

    while lo < hi {
      let mid = lo + (hi - lo) / 2;
      // mid<hi<=native_protos.len() 故下标合法；bytecode_id 读取经 header 门面。
      let mid_bytecode_id = self.native_protos[mid].exec_data_bytecode_id();

      if mid_bytecode_id < bytecode_id {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }

    if lo == self.native_protos.len() {
      return null();
    }

    let found_bytecode_id = self.native_protos[lo].exec_data_bytecode_id();
    if found_bytecode_id != bytecode_id {
      return null();
    }

    if lo + 1 < self.native_protos.len() {
      CODEGEN_ASSERT!(self.native_protos[lo + 1].exec_data_bytecode_id() != bytecode_id);
    }

    self.native_protos[lo].as_ptr()
  }
}
