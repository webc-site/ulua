use alloc::vec::Vec;
use core::{
  ptr::{NonNull, null},
  sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data::{
    get_native_proto_exec_data_header, get_native_proto_exec_data_header_mut,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{code_allocation_data::CodeAllocationData, shared_code_allocator::SharedCodeAllocator},
  type_aliases::{module_id::ModuleId, native_proto_exec_data_ptr::NativeProtoExecDataPtr},
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
    let native_module = self as *mut NativeModule;

    for native_proto in &self.native_protos {
      // Safety: native_protos 元素为 NonNull<u32>, as_ptr 非空; get_native_proto_exec_data_header_mut
      // 依"头部紧邻 instruction_offsets 之前"布局在同一分配内反推, 得非空且对齐的 header。所写两字段
      // 位于 execdata 外部堆分配(非 self 内), 与 &self 借用内存不相交; native_module=self as *mut 仅存值,
      // code_base 来自构造断言非空的 allocation code_start, .add(entry_offset) 落在该块内。
      unsafe {
        let header = get_native_proto_exec_data_header_mut(native_proto.as_ptr());
        (*header).native_module = native_module;
        (*header).entry_offset_or_address =
          code_base.add((*header).entry_offset_or_address as usize);
      }
    }

    // Safety: 排序键只读 header.bytecode_id; header 由 NonNull 指针按相邻布局算得, 非空对齐且指向
    // 存活 execdata 分配, 解引用合法。
    self.native_protos.sort_by_key(|native_proto| unsafe {
      (*get_native_proto_exec_data_header_mut(native_proto.as_ptr())).bytecode_id
    });

    for pair in self.native_protos.windows(2) {
      // Safety: 同排序键路径——pair 元素为 NonNull 指针, 反推 header 非空对齐且指向存活 execdata,
      // 此处只读 bytecode_id 做去重断言, 不产生可变借用。
      unsafe {
        let left = (*get_native_proto_exec_data_header_mut(pair[0].as_ptr())).bytecode_id;
        let right = (*get_native_proto_exec_data_header_mut(pair[1].as_ptr())).bytecode_id;
        CODEGEN_ASSERT!(left != right);
      }
    }
  }

  /// 构造函数运行在栈上临时对象上，header.native_module 捕获的是临时地址；
  /// Box 化搬到最终堆地址后必须调用此方法重绑，否则 release 会作用到
  /// 已失效的栈内存（cpp `make_unique<NativeModule>` 直接在最终地址构造）。
  pub(crate) fn rebind_header_module_pointers(&mut self) {
    let native_module = self as *mut NativeModule;

    for native_proto in &self.native_protos {
      // Safety: 与 bind_native_protos 同理, header 由非空对齐的 NonNull execdata 指针按相邻布局算得;
      // 此处把 native_module 改写为 self 的最终(堆)地址, 修正构造期捕获的栈临时地址, 只写字段值、
      // 不解引用目标, 无别名冲突。
      unsafe {
        let header = get_native_proto_exec_data_header_mut(native_proto.as_ptr());
        (*header).native_module = native_module;
      }
    }
  }

  pub fn native_module_add_ref(&self) -> usize {
    self.refcount.fetch_add(1, Ordering::Relaxed) + 1
  }

  pub fn native_module_add_refs(&self, count: usize) -> usize {
    self.refcount.fetch_add(count, Ordering::Relaxed) + count
  }

  /// cpp SharedCodeAllocator.cpp:115-118 `getCodeAllocationData`。
  pub fn native_module_get_code_allocation_data(&self) -> CodeAllocationData {
    self.code_allocation_data
  }

  /// cpp SharedCodeAllocator.cpp:110-113 `getModuleBaseAddress`：直接返回
  /// 分配数据里的代码起始地址（移植期开关 LuauCodegenFreeBlocks 在 cpp 中
  /// 已删除，旧的 moduleBaseAddress 字段旁路不复存在）。
  pub fn native_module_get_module_base_address(&self) -> *const u8 {
    self.code_allocation_data.code_start
  }

  #[inline]
  pub fn native_module_get_module_id(&self) -> &Option<ModuleId> {
    &self.module_id
  }

  pub fn native_module_get_native_protos(&self) -> &[NativeProtoExecDataPtr] {
    &self.native_protos
  }

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
      // Safety: mid<hi<=native_protos.len() 故 native_protos[mid] 为合法下标; NonNull::as_ptr() 非空/对齐,
      // get_native_proto_exec_data_header 按其契约(header 紧接该 exec data 之前、同一分配且按 header 对齐)
      // 返回合法 *const Header, 读取 bytecode_id 落在该存活对象内。
      let mid_bytecode_id = unsafe {
        (*get_native_proto_exec_data_header(self.native_protos[mid].as_ptr())).bytecode_id
      };

      if mid_bytecode_id < bytecode_id {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }

    if lo == self.native_protos.len() {
      return null();
    }

    let found_bytecode_id = unsafe {
      // Safety: 上方已判定 lo<native_protos.len(), 故 native_protos[lo] 为合法下标; NonNull as_ptr 非空/对齐,
      // header 反推依契约落在同一分配内, 只读 bytecode_id。
      (*get_native_proto_exec_data_header(self.native_protos[lo].as_ptr())).bytecode_id
    };
    if found_bytecode_id != bytecode_id {
      return null();
    }

    if lo + 1 < self.native_protos.len() {
      // Safety: 判定 lo+1<len, native_protos[lo+1] 合法; as_ptr 非空、header 反推契约有效, 只读 bytecode_id。
      let next_bytecode_id = unsafe {
        (*get_native_proto_exec_data_header(self.native_protos[lo + 1].as_ptr())).bytecode_id
      };
      CODEGEN_ASSERT!(next_bytecode_id != bytecode_id);
    }

    self.native_protos[lo].as_ptr()
  }
}
