use alloc::alloc::{Layout, alloc, handle_alloc_error};
use core::{
  cmp::max,
  mem::{align_of, offset_of},
  ptr::NonNull,
};

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
const PAGE_DATA_OFFSET: usize = offset_of!(Page, data);
/// Page 自身的对齐。
const PAGE_ALIGN: usize = align_of::<Page>();

/// 快路径：尝试在 root 页内按 `ALIGN` 对齐分配 `size` 字节。
/// 返回 (对齐后的起始指针, 分配后的页内偏移)；放不下则返回 None。
///
/// 入参 `root` 已是 [`NonNull`] 句柄，解引用目标是 `Allocator` 独占拥有的页链
/// 字段（`self.root` 由 `allocate` 写入、仅本链表持有，`&mut self` 借据在调用
/// 瞬间仍在）——「非空」由类型承载、「存活且可独占」由字段所有权不变量承载，
/// 故本函数为 safe `fn`，残留 unsafe 块各带 `// Safety:` 写明前提（§2 收口）。
fn try_take_from_root(root: NonNull<Page>, offset: usize, size: usize) -> Option<(*mut u8, usize)> {
  // Safety: 上段契约——root 出自 `Allocator::root`（Option<NonNull> 证非空、
  // 页链唯一所有权）；data 是 Page 内定长数组字段，地址随页固定；此处仅读数组基址。
  let data_ptr = unsafe { (*root.as_ptr()).data.as_ptr() as usize };
  let result = (data_ptr + offset + ALIGN - 1) & !(ALIGN - 1);

  if result + size <= data_ptr + DEFAULT_PAGE_DATA_SIZE {
    Some((result as *mut u8, (result - data_ptr) + size))
  } else {
    None
  }
}

impl Allocator {
  pub fn allocate(&mut self, size: usize) -> *mut u8 {
    // 无页（root 为 None）即快路径必然失败，直接走慢路径建页，不再手写判空。
    if let Some(root) = self.root {
      // root 出自本 Allocator 独占持有的页链，快路径为 safe fn，直调无 unsafe。
      if let Some((ptr, new_offset)) = try_take_from_root(root, self.offset, size) {
        self.offset = new_offset;
        return ptr;
      }
    }

    // 慢路径：当前页放不下，整页申请；超大请求按实际大小超额分配。
    let page_size = max(size, DEFAULT_PAGE_DATA_SIZE);
    let layout = Layout::from_size_align(PAGE_DATA_OFFSET + page_size, PAGE_ALIGN).expect(
      "Page 布局恒合法：大小≥PAGE_DATA_OFFSET+8192 非零、对齐为 align_of::<Page>() 恒为 2 的幂",
    );

    // Safety: layout 由合法常量与请求大小构造，非零且对齐为 Page 对齐的整数倍；
    // 返回空即内存耗尽，立即 handle_alloc_error 中止，故 ptr 恒为非空块首地址。
    let ptr = match NonNull::new(unsafe { alloc(layout) }.cast::<Page>()) {
      Some(ptr) => ptr,
      None => handle_alloc_error(layout),
    };

    // Safety: ptr 是刚分配成功的块，next/alloc_size 为区域内普通字段；写 None 之外的
    // 初值前先写后读，无未初始化读取。data 紧随头部，as_mut_ptr 取到的区域大小即
    // 本次申请的 page_size。
    unsafe {
      let page = ptr.as_ptr();
      // 新页压顶，旧链（可能为 None = cpp 的链尾 nullptr）整体成为本页的 next。
      (*page).next = self.root;
      // 记录精确分配大小，Drop 时用同一 Layout 释放（超大请求会超额分配）。
      (*page).alloc_size = layout.size();
    }

    self.root = Some(ptr);
    self.offset = size;

    // Safety: ptr 有效且独占，data 是页内定长数组字段，取基址不越界。
    unsafe { (*ptr.as_ptr()).data.as_mut_ptr() }
  }
}
