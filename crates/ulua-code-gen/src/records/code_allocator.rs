use alloc::vec::Vec;
use core::{
  ffi::c_void,
  ptr::{NonNull, null_mut},
  slice::from_raw_parts_mut,
};

use crate::{
  functions::{
    allocate_pages_impl_code_allocator::allocate_pages_impl,
    flush_instruction_cache_code_allocator::flush_instruction_cache,
    free_pages_impl_code_allocator::free_pages_impl,
    make_pages_executable_code_allocator::make_pages_executable,
    make_pages_not_executable_code_allocator::make_pages_not_executable,
    make_pages_read_only_code_allocator::make_pages_read_only,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::code_allocation_data::CodeAllocationData,
  type_aliases::api::AllocationCallback,
};

#[derive(Debug)]
#[repr(C)]
pub struct CodeAllocator {
  /// DELIBERATE DEVIATION / 保留理由（review.md §2 的可空裸指针豁免项）：
  /// 本字段是**跨 ABI 的 opaque user-data**，不是「缺席/未接线」语义，故不收成
  /// `Option<NonNull<T>>`——它的类型 `*mut c_void` 由被调方签名钉死，换型即换 ABI：
  /// - 写入点唯一：`BaseCodeGenContext::base_code_gen_context`（本平台 `UnwindBuilderImpl`
  ///   的 `Box::into_raw` 结果经 `.cast()` 抹成 `c_void`）；
  /// - 交回 C ABI 的点有两个，都在本文件：`allocate_new_block` 里
  ///   `create_block_unwind_info(self.context, block.as_ptr(), self.block_size,
  ///   &mut start_offset)`，被调方类型为
  ///   `Option<unsafe extern "C-unwind" fn(context: *mut c_void, block: *mut u8,
  ///   block_size: usize, start_offset: &mut usize) -> *mut c_void>`（即本结构
  ///   `create_block_unwind_info` 字段）；`destroy` 里
  ///   `destroy_block_unwind_info_fn(self.context, *unwind_info, self.block_size)`，
  ///   被调方类型为 `destroy_block_unwind_info` 字段那个
  ///   `unsafe extern "C-unwind" fn(context: *mut c_void, unwind_data: *mut c_void,
  ///   unwind_block_size: usize)`。
  ///
  /// 两处第一实参都必须是宿主回调认得的 `void*`，`Option<NonNull<T>>` 只能经
  /// `map_or(null_mut(), NonNull::as_ptr)` 还原成同一个裸指针——形式转换，无收益。
  /// `null_mut()` 初值对应 cpp `CodeAllocator.h` 的 `void* context = nullptr`，
  /// 含义是「未接 unwind builder，两个回调槽为 None」，与回调是否可空一致。
  pub context: *mut c_void,
  pub create_block_unwind_info: Option<
    unsafe extern "C-unwind" fn(
      context: *mut c_void,
      block: *mut u8,
      block_size: usize,
      start_offset: &mut usize,
    ) -> *mut c_void,
  >,
  /// 注销块 unwind 信息；`unwind_block_size` 让被调方以切片界定 FDE 遍历的读取上界
  /// （cpp `destroyBlockUnwindInfo` 仅凭指针自终止遍历，Rust 侧指针必须配长度）。
  pub destroy_block_unwind_info: Option<
    unsafe extern "C-unwind" fn(
      context: *mut c_void,
      unwind_data: *mut c_void,
      unwind_block_size: usize,
    ),
  >,
  /// 当前块写入游标（地址）；`0` 表示尚无活动块。
  pub(crate) block_pos: usize,
  /// 当前块 one-past-end（地址）。
  pub(crate) block_end: usize,
  /// 历次映射块的基址（地址），`destroy` 时逐个解除映射。
  pub(crate) blocks: Vec<usize>,
  /// 历次映射块的 unwind 信息句柄（由 `create_block_unwind_info` 产出，产出点经
  /// `NonNull::new` 收编，恒非空；`destroy` 时逐个交回 C 回调——裸形只在该 ABI 边界还原）。
  pub(crate) unwind_infos: Vec<NonNull<c_void>>,
  pub(crate) block_size: usize,
  pub(crate) max_total_size: usize,
  pub(crate) live_allocations: usize,
  pub(crate) allocation_callback: Option<AllocationCallback>,
  /// DELIBERATE DEVIATION / 保留理由：与 `context` 同类的 opaque user-data，收
  /// `Option<NonNull<T>>` 只会把同一处样板推给被调点。本字段由创建方原样注入
  /// （`code_allocator_usize_usize_allocation_callback_void` 形参 /
  /// `luau_codegen_create` 与 `create_shared_code_gen_context` 传入的宿主指针），
  /// 并在本文件两处交回 C ABI：
  /// - `allocate_pages`：`callback(self.allocation_callback_context, null_mut(), 0,
  ///   mem.as_ptr().cast(), page_aligned_size)`；
  /// - `free_pages`：`callback(self.allocation_callback_context, mem.cast(),
  ///   page_aligned_size, null_mut(), 0)`。
  ///
  /// 两处 `callback` 即 `allocation_callback` 字段，类型为
  /// `AllocationCallback = unsafe extern "C-unwind" fn(context: *mut c_void,
  /// old_pointer: *mut c_void, old_size: usize, new_pointer: *mut c_void,
  /// new_size: usize)`——第一参在 ABI 上就是宿主自持的 `void*`，本侧从不解引用。
  /// `null_mut()` 初值对应 cpp `CodeAllocator` 的 `void* allocationCallbackContext`
  /// 零初值，语义是「回调为 None 时无人消费」，故 `code_allocator_usize_usize`
  /// 的默认构造也传 null。
  pub(crate) allocation_callback_context: *mut c_void,
  pub(crate) destroyed: bool,
}

impl CodeAllocator {
  pub(crate) const K_MAX_RESERVED_DATA_SIZE: usize = 256;

  /// code 起始/游标对齐粒度（字节）：单源常量，消 `allocate`/`allocate_new_block`/
  /// unwind 建表三处重复的 `32` 魔法数（review.md §4 编译期化）。
  pub(crate) const K_CODE_ALIGNMENT: usize = 32;

  /// 当前块剩余可写字节数（纯地址差，无活动块时为 0）。即便 `block_pos`/`block_end`
  /// 的不变量被破坏也饱和到 0 而非回绕成巨值，令后续分配安全落入
  /// `allocate_new_block` 路径（cpp `size_t(blockEnd - blockPos)` 的防御化等价）。
  #[inline]
  pub(crate) fn block_remaining(&self) -> usize {
    self.block_end.saturating_sub(self.block_pos)
  }

  pub fn align_to_page_size(size: usize) -> usize {
    #[cfg(target_os = "windows")]
    let page_size = 4096;

    #[cfg(not(target_os = "windows"))]
    // Safety: getpagesize 为 POSIX C ABI 无参函数，调用不含指针/长度前提，返回内核页大小
    // 正整数；extern 声明与 libc 签名一致，转 usize 仅用于其后的对齐算术。
    let page_size = unsafe {
      unsafe extern "C" {
        fn getpagesize() -> i32;
      }

      getpagesize() as usize
    };

    // 标准库「向上取整到倍数」（页大小恒为 2 的幂，与原掩码公式逐位一致，
    // 且不引入 `size + page_size - 1` 的中间溢出）。
    size.next_multiple_of(page_size)
  }

  /// 以当前游标为基派生块内绝对地址（可执行页边界，见 records::code_allocator 模块注释）。
  ///
  /// 纯地址算术、从不解引用（同 `VmFrame::shift` 先例）：`offset` 是否落在
  /// `[block_pos, block_end)` 由调用点的容量校验/断言论证，读写兜底统一在
  /// [`CodeAllocator::block_bytes_mut`] 的界内切片视图处。
  #[inline]
  fn cursor_ptr(&self, offset: usize) -> *mut u8 {
    (self.block_pos as *mut u8).wrapping_add(offset)
  }

  /// 当前块 `[offset, offset+len)` 的可写字节视图；越界返回 `None`（不 UB，
  /// 由调用点断言兜底）。mmap 页地址 → 切片的 `from_raw_parts_mut` 解引用
  /// 边界收口于本方法一处（review.md §2「最小封装边界」）。
  fn block_bytes_mut(&mut self, offset: usize, len: usize) -> Option<&mut [u8]> {
    // 类型不变量：`[block_pos, block_end)` 恒为 `allocate_new_block` 经 mmap 取得、
    // 仍存活映射的可写代码页区间（构造期为空区间 0..0，`destroy` 先行复位游标），
    // 且 `&mut self` 提供视图存续期的独占写权限。
    let remaining = self.block_end.checked_sub(self.block_pos)?;
    if offset.checked_add(len)? > remaining {
      return None;
    }
    // Safety: 上式已验证 offset..offset+len ⊆ [block_pos, block_end)，即存活 mmap
    // 可写页内区间；切片互不重叠、写权限由 `&mut self` 独占保证。
    Some(unsafe { from_raw_parts_mut(self.cursor_ptr(offset), len) })
  }

  /// 当前块已映射区间的页对齐起点（等价 cpp 对 block_pos 的页对齐断言前件）。
  #[inline]
  fn cursor_page_aligned(&self) -> bool {
    CodeAllocator::align_to_page_size(self.block_pos) == self.block_pos
  }

  /// 把汇编产出的 data 段与 code 段拷入当前（或新映射的）可写代码块，完成页保护切换
  /// 并返回分配描述（cpp `CodeAllocator::allocate`；输入以切片表达长度界内性，
  /// 原 `data_size`/`code_size` 裸指针+长度对随切片化删除）。
  pub fn allocate(&mut self, data: &[u8], code: &[u8]) -> CodeAllocationData {
    use ulua_common::fflag;

    let data_size = data.len();
    let code_size = code.len();

    let mut start_offset = 0;
    let code_offset: usize;
    let data_offset: usize;
    let page_aligned_size: usize;
    let total_size: usize;

    if fflag::LuauCodegenProtectData.get() {
      if data_size != 0 {
        if CodeAllocator::align_to_page_size(Self::K_MAX_RESERVED_DATA_SIZE + data_size) + code_size
          > self.block_size
        {
          return CodeAllocationData::default();
        }

        if CodeAllocator::align_to_page_size(data_size) + code_size > self.block_remaining() {
          let Some(offset) = self.allocate_new_block() else {
            return CodeAllocationData::default();
          };
          start_offset = offset;
          CODEGEN_ASSERT!(
            CodeAllocator::align_to_page_size(start_offset + data_size) + code_size
              <= self.block_remaining()
          );
        }

        code_offset = CodeAllocator::align_to_page_size(start_offset + data_size);
        data_offset = code_offset - data_size;
        total_size = CodeAllocator::align_to_page_size(data_size) + code_size;
        page_aligned_size = CodeAllocator::align_to_page_size(code_offset + code_size);
      } else {
        let ts = code_size;
        if ts > self.block_size - Self::K_MAX_RESERVED_DATA_SIZE {
          return CodeAllocationData::default();
        }

        if ts > self.block_remaining() {
          let Some(offset) = self.allocate_new_block() else {
            return CodeAllocationData::default();
          };
          start_offset = offset;
          CODEGEN_ASSERT!(ts <= self.block_remaining());
        }

        data_offset = start_offset;
        code_offset = start_offset;
        page_aligned_size = CodeAllocator::align_to_page_size(start_offset + ts);
        total_size = ts;
      }
    } else {
      // 'Round up' 保留 code 对齐（cpp 同段），标准库取倍无中间溢出。
      let aligned_data_size = data_size.next_multiple_of(Self::K_CODE_ALIGNMENT);
      let ts = aligned_data_size + code_size;

      if ts > self.block_size - Self::K_MAX_RESERVED_DATA_SIZE {
        return CodeAllocationData::default();
      }

      if ts > self.block_remaining() {
        let Some(offset) = self.allocate_new_block() else {
          return CodeAllocationData::default();
        };
        start_offset = offset;
        CODEGEN_ASSERT!(ts <= self.block_remaining());
      }

      data_offset = start_offset + aligned_data_size - data_size;
      code_offset = start_offset + aligned_data_size;
      page_aligned_size = CodeAllocator::align_to_page_size(start_offset + ts);
      total_size = ts;
    }

    CODEGEN_ASSERT!(self.cursor_page_aligned());

    if data_size != 0 {
      // 容量校验与各分支 CODEGEN_ASSERT! 已保证 `data_offset + data_size` 落在
      // `[block_pos, block_end)`——界内视图取不到即为逻辑缺陷，panic 兜底（原实现
      // 此处为越界 memcpy UB）。拷贝经切片 `copy_from_slice`，源/目标长度恒等。
      self
        .block_bytes_mut(data_offset, data_size)
        .expect("data 段界内性由上方容量校验保证")
        .copy_from_slice(data);
    }
    if code_size != 0 {
      // 同上：`code_offset + code_size` 亦经容量校验落在当前块映射区内。
      self
        .block_bytes_mut(code_offset, code_size)
        .expect("code 段界内性由上方容量校验保证")
        .copy_from_slice(code);
    }

    // Safety: 先 `make_pages_read_only(block_pos, code_offset)` 将 data 段改为只读,再对其后的 code 页
    // `make_pages_executable`——两者都是对已 mmap、仍存活的整块区域调 `mprotect`,地址落在分配内且按页对齐。
    if fflag::LuauCodegenProtectData.get() {
      if data_size != 0 {
        if !make_pages_read_only(self.block_pos as *mut u8, code_offset) {
          return CodeAllocationData::default();
        }
        // Safety: `code_offset` 界内(上方容量校验),所得地址落在映射区内的页对齐 code 起点,
        // 长度 `page_aligned_size - code_offset` 为已算好的页对齐余量,均在 block 分配内;mprotect 参数合法。
        if !unsafe {
          make_pages_executable(
            self.cursor_ptr(code_offset),
            page_aligned_size - code_offset,
          )
        } {
          return CodeAllocationData::default();
        }
      } else if !unsafe {
        // Safety: `data_size==0` 时整块即 code,`block_pos` 起 `page_aligned_size` 字节为已 mmap 且存活的
        // 页对齐区域,`make_pages_executable` 对其调 `mprotect(PROT_READ|PROT_EXEC)`,参数合法。
        make_pages_executable(self.block_pos as *mut u8, page_aligned_size)
      } {
        return CodeAllocationData::default();
      }
    } else if !unsafe {
      // Safety: 非保护分支同——`[block_pos, block_pos+page_aligned_size)` 为已 mmap 存活的页对齐区域,
      // `make_pages_executable` 的 mprotect 参数界内且对齐。
      make_pages_executable(self.block_pos as *mut u8, page_aligned_size)
    } {
      return CodeAllocationData::default();
    }

    self.live_allocations += 1;
    // Safety: `code_offset` 界内(容量分支已证),所得地址指向本分配 code 起点(平移界内性同上);
    // 在把该可执行内存交给调用方运行之前,于此刻对其刷新指令缓存,保证 A64 自修改代码的 icache
    // 一致性——拷贝已在 mprotect 之前于可写页完成。
    flush_instruction_cache(self.cursor_ptr(code_offset), code_size);

    // `start_offset`/`code_offset` 由前面的容量校验保证落在 `[block_pos, block_end)` 内,
    // 所得地址即本分配的 data/code 起点(cursor_ptr 为纯平移,界内性论证随消费侧)。
    let result = CodeAllocationData {
      start: self.cursor_ptr(start_offset),
      size: total_size,
      code_start: self.cursor_ptr(code_offset),
      allocation_start: self.block_pos as *mut u8,
      allocation_size: page_aligned_size,
    };

    if page_aligned_size <= self.block_remaining() {
      // 该分支已判定 `page_aligned_size <= block_end - block_pos`,故游标推进后仍不超过
      // `block_end`,停留在映射区内(下行 CODEGEN_ASSERT 复核)。纯整数游标算术,safe。
      self.block_pos += page_aligned_size;
      CODEGEN_ASSERT!(self.cursor_page_aligned());
      CODEGEN_ASSERT!(self.block_pos <= self.block_end);
    } else {
      self.block_pos = self.block_end;
    }

    result
  }

  /// 申请一个新映射块并把游标复位到块首。成功返回首个可用写偏移
  /// （unwind 表预留区对齐后的 `start_offset`，无 unwind 回调时为 `0`）。
  pub(crate) fn allocate_new_block(&mut self) -> Option<usize> {
    if (self.blocks.len() + 1) * self.block_size > self.max_total_size {
      return None;
    }

    let block = self.allocate_pages(self.block_size)?;
    let block_addr = block.as_ptr() as usize;

    self.block_pos = block_addr;
    self.block_end = block_addr + self.block_size;
    self.blocks.push(block_addr);

    let mut start_offset = 0;
    if let Some(create_block_unwind_info) = self.create_block_unwind_info {
      // Safety: create_block_unwind_info 为本 crate 提供的 extern "C-unwind" 回调, 契约接受
      // (context, block, block_size, &mut start_offset); self.context 为构造期接线的 opaque 用户指针,
      // block 为上方刚分配、非空、页对齐的存活代码页。
      let unwind_info = unsafe {
        create_block_unwind_info(
          self.context,
          block.as_ptr(),
          self.block_size,
          &mut start_offset,
        )
      };

      // 'Round up' 对齐改用标准库取倍（同 cpp 注释语义，见 align_to_page_size 论证）。
      start_offset = start_offset.next_multiple_of(Self::K_CODE_ALIGNMENT);

      CODEGEN_ASSERT!(start_offset <= CodeAllocator::K_MAX_RESERVED_DATA_SIZE);

      // cpp `if (!unwindInfo) return false` 的空哨兵判定：句柄在此边界收编为非空，
      // 内部存储不再出现可空裸指针（失败即放弃本块，与 cpp 一致不回滚已登记的
      // block/游标）。
      let unwind_info = NonNull::new(unwind_info)?;

      self.unwind_infos.push(unwind_info);
    }

    Some(start_offset)
  }

  /// 映射一页对齐的可写块；失败（映射返回空）以 `None` 表达。
  pub fn allocate_pages(&self, size: usize) -> Option<NonNull<u8>> {
    let page_aligned_size = CodeAllocator::align_to_page_size(size);
    let mem = allocate_pages_impl(page_aligned_size)?;

    // Safety: allocation_callback 为构造期注册的合法 C 回调，其契约接收 context（可 null，
    // 宿主提供）、旧块 null/0 与新映射 mem/页对齐 size，参数与此一致。
    // 真 FFI 边界（review.md §2/§3 豁免项）：`AllocationCallback` 是宿主侧
    // `unsafe extern "C-unwind" fn(void* ctx, void* old, size_t, void* new, size_t)`，
    // 这里的 `null_mut()` 是该 C ABI 的「旧块缺席」实参编码，不是本 crate 的哨兵。
    if let Some(callback) = self.allocation_callback {
      unsafe {
        callback(
          self.allocation_callback_context,
          null_mut(),
          0,
          mem.as_ptr().cast(),
          page_aligned_size,
        );
      }
    }

    Some(mem)
  }

  pub fn code_allocator_usize_usize(&mut self, block_size: usize, max_total_size: usize) {
    self.code_allocator_usize_usize_allocation_callback_void(
      block_size,
      max_total_size,
      None,
      null_mut(),
    );
  }

  pub fn code_allocator_usize_usize_allocation_callback_void(
    &mut self,
    block_size: usize,
    max_total_size: usize,
    allocation_callback: Option<AllocationCallback>,
    allocation_callback_context: *mut c_void,
  ) {
    self.block_pos = 0;
    self.block_end = 0;
    self.blocks.clear();
    self.unwind_infos.clear();
    self.block_size = block_size;
    self.max_total_size = max_total_size;
    self.live_allocations = 0;
    self.allocation_callback = allocation_callback;
    self.allocation_callback_context = allocation_callback_context;
    self.destroyed = false;

    debug_assert!(block_size > CodeAllocator::K_MAX_RESERVED_DATA_SIZE);
    debug_assert!(max_total_size >= block_size);
  }

  pub fn destroy(&mut self) {
    if self.destroyed {
      return;
    }

    self.destroyed = true;

    if let Some(destroy_block_unwind_info_fn) = self.destroy_block_unwind_info {
      for &unwind_info in &self.unwind_infos {
        // Safety: destroy_block_unwind_info 为本 crate 提供的 extern "C-unwind" 回调，契约接受
        // (context, unwind_data, unwind_block_size)；self.context 为构造期接线的 unwind_builder cast 指针、
        // unwind_info.as_ptr() 为先前 create_block_unwind_info 登记的存活块基址（NonNull 句柄
        // 在此 ABI 边界还原为 c_void* 裸形），self.block_size 是该块的映射长度下限，
        // 遍历清空前二者仍有效（free_pages 在下方才执行）。
        unsafe {
          destroy_block_unwind_info_fn(self.context, unwind_info.as_ptr(), self.block_size);
        }
      }
    }

    // cpp CodeAllocator.cpp:174：析构前所有分配必须已归还
    CODEGEN_ASSERT!(self.live_allocations == 0);

    for block in &self.blocks {
      self.free_pages(*block, self.block_size);
    }

    self.unwind_infos.clear();
    self.blocks.clear();
    self.block_pos = 0;
    self.block_end = 0;
  }

  /// cpp CodeAllocator.cpp:323-335 `deallocate`：仅解除页可执行属性并递减
  /// 存活计数（移植期开关 LuauCodegenFreeBlocks 在 cpp 中已删除，
  /// 块复用留有上游 TODO，本侧不再走旧的整体归还路径）。
  pub fn deallocate(&mut self, code_allocation_data: CodeAllocationData) {
    if code_allocation_data.allocation_start.is_null() {
      return;
    }

    let result = make_pages_not_executable(
      code_allocation_data.allocation_start,
      code_allocation_data.allocation_size,
    );
    CODEGEN_ASSERT!(result);

    CODEGEN_ASSERT!(self.live_allocations != 0);
    self.live_allocations -= 1;
  }

  /// `mem` 为 `allocate_pages` 产出的块基址（地址形态，见模块注释）。
  pub fn free_pages(&self, mem: usize, size: usize) {
    let page_aligned_size = CodeAllocator::align_to_page_size(size);
    let mem = mem as *mut u8;

    // Safety: allocation_callback 为注册期提供的合法 C 回调，此处按 C++ 语义以
    // (context, mem, 页对齐 size, null, 0) 通知解除映射，mem/size 为先前 allocate_pages 的配对块。
    if let Some(callback) = self.allocation_callback {
      unsafe {
        callback(
          self.allocation_callback_context,
          mem.cast(),
          page_aligned_size,
          null_mut(),
          0,
        );
      }
    }

    // Safety: free_pages_impl 依同一页对齐存活块契约释放，参数满足其前置条件。
    free_pages_impl(mem, page_aligned_size);
  }
}

impl Default for CodeAllocator {
  fn default() -> Self {
    Self {
      context: null_mut(),
      create_block_unwind_info: None,
      destroy_block_unwind_info: None,
      block_pos: 0,
      block_end: 0,
      blocks: Vec::new(),
      unwind_infos: Vec::new(),
      block_size: 0,
      max_total_size: 0,
      live_allocations: 0,
      allocation_callback: None,
      allocation_callback_context: null_mut(),
      destroyed: false,
    }
  }
}

impl Drop for CodeAllocator {
  fn drop(&mut self) {
    CodeAllocator::destroy(self);
  }
}
