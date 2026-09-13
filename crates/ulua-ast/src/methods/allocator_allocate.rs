use alloc::alloc::{Layout, alloc, handle_alloc_error};
use core::{cmp::max, mem::align_of};

use crate::records::{allocator::Allocator, page::Page};

/// 页内分配对齐：max(void*, double) 对齐，覆盖所有 AST 节点字段类型。
const ALIGN: usize = {
  let align_void = align_of::<*mut ()>();
  let align_double = align_of::<f64>();
  if align_void > align_double {
    align_void
  } else {
    align_double
  }
};
/// 默认页数据区大小（字节）。
const DEFAULT_PAGE_DATA_SIZE: usize = 8192;
/// Page 头部到 data 字段的偏移。
const PAGE_DATA_OFFSET: usize = core::mem::offset_of!(Page, data);
/// Page 自身的对齐。
const PAGE_ALIGN: usize = align_of::<Page>();

/// 快路径：尝试在 root 页内按 `ALIGN` 对齐分配 `size` 字节。
/// 返回 (对齐后的起始指针, 分配后的页内偏移)；放不下则返回 None。
///
/// # Safety
/// `root` 必须指向有效且仍被拥有的 `Page`。
unsafe fn try_take_from_root(
  root: *mut Page,
  offset: usize,
  size: usize,
) -> Option<(*mut u8, usize)> {
  let data_ptr = unsafe { (*root).data.as_ptr() as usize };
  let result = (data_ptr + offset + ALIGN - 1) & !(ALIGN - 1);

  if result + size <= data_ptr + DEFAULT_PAGE_DATA_SIZE {
    Some((result as *mut u8, (result - data_ptr) + size))
  } else {
    None
  }
}

impl Allocator {
  pub fn allocate(&mut self, size: usize) -> *mut u8 {
    if !self.root.is_null() {
      // SAFETY: root 由本 Allocator 独占拥有，指向有效 Page。
      if let Some((ptr, new_offset)) = unsafe { try_take_from_root(self.root, self.offset, size) } {
        self.offset = new_offset;
        return ptr;
      }
    }

    // 慢路径：当前页放不下，整页申请；超大请求按实际大小超额分配。
    let page_size = max(size, DEFAULT_PAGE_DATA_SIZE);
    let layout = Layout::from_size_align(PAGE_DATA_OFFSET + page_size, PAGE_ALIGN)
      .expect("Invalid layout for Page allocation");

    // SAFETY: layout 由合法常量与请求大小构造，非零且对齐为 Page 对齐的整数倍。
    let page_ptr = unsafe { alloc(layout) as *mut Page };
    if page_ptr.is_null() {
      handle_alloc_error(layout);
    }

    // SAFETY: page_ptr 刚分配成功，指向未初始化但已拥有的 Page 内存。
    unsafe {
      (*page_ptr).next = self.root;
      // 记录精确分配大小，Drop 时用同一 Layout 释放（超大请求会超额分配）。
      (*page_ptr).alloc_size = layout.size();
    }
    self.root = page_ptr;
    self.offset = size;

    // SAFETY: 同上，page_ptr 有效且 data 字段紧随头部。
    unsafe { (*page_ptr).data.as_mut_ptr() }
  }
}
