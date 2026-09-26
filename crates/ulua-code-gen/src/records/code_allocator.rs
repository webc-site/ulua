use alloc::vec::Vec;
use core::{
  ffi::c_void,
  ptr,
  ptr::{NonNull, null_mut},
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
  type_aliases::allocation_callback::AllocationCallback,
};

#[derive(Debug)]
#[repr(C)]
pub struct CodeAllocator {
  /// unwind 回调的 opaque 上下文（指向本平台具体 UnwindBuilder 实现）。
  pub context: *mut c_void,
  pub create_block_unwind_info: Option<
    unsafe extern "C-unwind" fn(
      context: *mut c_void,
      block: *mut u8,
      block_size: usize,
      start_offset: &mut usize,
    ) -> *mut c_void,
  >,
  pub destroy_block_unwind_info:
    Option<unsafe extern "C-unwind" fn(context: *mut c_void, unwind_data: *mut c_void)>,
  /// 当前块写入游标（地址）；`0` 表示尚无活动块。
  pub(crate) block_pos: usize,
  /// 当前块 one-past-end（地址）。
  pub(crate) block_end: usize,
  /// 历次映射块的基址（地址），`destroy` 时逐个解除映射。
  pub(crate) blocks: Vec<usize>,
  /// 历次映射块的 unwind 信息句柄（由 `create_block_unwind_info` 产出）。
  pub(crate) unwind_infos: Vec<*mut c_void>,
  pub(crate) block_size: usize,
  pub(crate) max_total_size: usize,
  pub(crate) live_allocations: usize,
  pub(crate) allocation_callback: Option<AllocationCallback>,
  pub(crate) allocation_callback_context: *mut c_void,
  pub(crate) destroyed: bool,
}

impl CodeAllocator {
  pub(crate) const K_MAX_RESERVED_DATA_SIZE: usize = 256;

  /// 当前块剩余可写字节数（纯地址差，无活动块时为 0）。
  #[inline]
  pub(crate) fn block_remaining(&self) -> usize {
    self.block_end.wrapping_sub(self.block_pos)
  }

  pub fn align_to_page_size(size: usize) -> usize {
    #[cfg(target_os = "windows")]
    let page_size = 4096;

    #[cfg(not(target_os = "windows"))]
    // Safety: getpagesize 为 POSIX C ABI 无参函数，调用不含指针/长度前提，返回内核页大小
    // 正整数；extern 声明与 libc 签名一致，转 usize 仅用于其后的对齐掩码算术。
    let page_size = unsafe {
      unsafe extern "C" {
        fn getpagesize() -> i32;
      }

      getpagesize() as usize
    };

    (size + page_size - 1) & !(page_size - 1)
  }

  /// 以当前游标为基派生块内绝对地址（可执行页边界，见 records::code_allocator 模块注释）。
  ///
  /// # Safety
  /// 当前游标必须指向存活映射块（allocate_new_block 成功后至 destroy 前），且 `offset`
  /// 满足 `offset <= block_end - block_pos`（结果允许恰为 one-past-end）。
  #[inline]
  unsafe fn cursor_ptr(&self, offset: usize) -> *mut u8 {
    // Safety: 依本函数契约——游标地址来自存活映射块，offset 界内，故还原指针的 add 落在
    // 该分配内（或恰为其 one-past-end）。
    unsafe { (self.block_pos as *mut u8).add(offset) }
  }

  /// 当前块已映射区间的页对齐起点（等价 cpp 对 block_pos 的页对齐断言前件）。
  #[inline]
  fn cursor_page_aligned(&self) -> bool {
    CodeAllocator::align_to_page_size(self.block_pos) == self.block_pos
  }

  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn allocate(
    &mut self,
    data: *const u8,
    data_size: usize,
    code: *const u8,
    code_size: usize,
  ) -> CodeAllocationData {
    use ulua_common::fflag;

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
      let k_code_alignment = 32;
      let aligned_data_size = (data_size + (k_code_alignment - 1)) & !(k_code_alignment - 1);
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
      // Safety: `block_pos`/`block_end` 界定当前 `allocate_new_block` 经 mmap 取得的可写代码页,
      // 前面各分支的容量检查与 `CODEGEN_ASSERT!` 已保证 `data_offset + data_size` 落在
      // `[block_pos, block_end)`;`data` 由调用方提供、指向 assembler 刚产出的缓冲且长度
      // ≥ `data_size`;写入本分配独占的新页(尚未 mprotect),`copy_nonoverlapping` 逐字节无对齐
      // 要求且源/目标不重叠,`data_size != 0` 时才进入。
      unsafe {
        ptr::copy_nonoverlapping(data, self.cursor_ptr(data_offset), data_size);
      }
    }
    if code_size != 0 {
      // Safety: 同上,`code_offset + code_size` 亦经容量校验落在 `[block_pos, block_end)` 内,
      // `code` 指向 assembler 产出的机器码缓冲且长度 ≥ `code_size`;目标为本分配独占新页,
      // 逐字节非重叠拷贝,`code_size != 0`。
      unsafe {
        ptr::copy_nonoverlapping(code, self.cursor_ptr(code_offset), code_size);
      }
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
    // Safety: `code_offset` 界内(容量分支已证),所得地址指向本分配 code 起点;在把该可执行内存交给
    // 调用方运行之前,于此刻对其刷新指令缓存,保证 A64 自修改代码的 icache 一致性——拷贝已在
    // mprotect 之前于可写页完成。
    flush_instruction_cache(unsafe { self.cursor_ptr(code_offset) }, code_size);

    let result = CodeAllocationData {
      // Safety: `start_offset`/`code_offset` 由前面的容量校验保证落在 `[block_pos, block_end)` 内,
      // 所得偏移地址有效。
      start: unsafe { self.cursor_ptr(start_offset) },
      size: total_size,
      // Safety: 同上,`code_offset` 界内,所得指针指向本分配的 code 起点。
      code_start: unsafe { self.cursor_ptr(code_offset) },
      allocation_start: self.block_pos as *mut u8,
      allocation_size: page_aligned_size,
    };

    if page_aligned_size <= self.block_remaining() {
      // Safety: 该分支已判定 `page_aligned_size <= block_end - block_pos`,故游标推进后仍不超过
      // `block_end`,停留在映射区内(下行 CODEGEN_ASSERT 复核)。
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
  pub fn allocate_new_block(&mut self) -> Option<usize> {
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

      const K_CODE_ALIGNMENT: usize = 32;
      start_offset = (start_offset + (K_CODE_ALIGNMENT - 1)) & !(K_CODE_ALIGNMENT - 1);

      CODEGEN_ASSERT!(start_offset <= CodeAllocator::K_MAX_RESERVED_DATA_SIZE);

      if unwind_info.is_null() {
        return None;
      }

      self.unwind_infos.push(unwind_info);
    }

    Some(start_offset)
  }

  /// 映射一页对齐的可写块；失败（映射返回空）以 `None` 表达。
  pub fn allocate_pages(&self, size: usize) -> Option<NonNull<u8>> {
    let page_aligned_size = CodeAllocator::align_to_page_size(size);
    let mem = allocate_pages_impl(page_aligned_size);
    let mem = NonNull::new(mem)?;

    // Safety: allocation_callback 为构造期注册的合法 C 回调，其契约接收 context（可 null，
    // 宿主提供）、旧块 null/0 与新映射 mem/页对齐 size，参数与此一致。
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
      for unwind_info in &self.unwind_infos {
        // Safety: destroy_block_unwind_info 为本 crate 提供的 extern "C-unwind" 回调，契约接受
        // (context, unwind_data)；self.context 为构造期接线的 unwind_builder cast 指针、*unwind_info
        // 为先前 create_block_unwind_info 登记的存活块 unwind 数据，遍历清空前二者仍有效。
        unsafe {
          destroy_block_unwind_info_fn(self.context, *unwind_info);
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
