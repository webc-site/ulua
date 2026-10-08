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
/// 全程 safe：`data` 是 `#[repr(C)]` `Page` 的定长尾字段，基址恒等于
/// 页首址 + 编译期常量 [`PAGE_DATA_OFFSET`]，纯地址算术即可得出，无需解引用
/// 页头（对齐推进不再触碰 Page 内存本身）。
fn try_take_from_root(root: NonNull<Page>, offset: usize, size: usize) -> Option<(*mut u8, usize)> {
  let data_ptr = root.as_ptr() as usize + PAGE_DATA_OFFSET;
  // bump 对齐推进：把游标推到 ALIGN 上界，再判断默认页数据窗是否装得下本次请求
  // （慢路径建页时数据区 ≥ DEFAULT_PAGE_DATA_SIZE，故该窗对任何页都界内）。
  let result = (data_ptr + offset + ALIGN - 1) & !(ALIGN - 1);

  (result + size <= data_ptr + DEFAULT_PAGE_DATA_SIZE)
    .then_some((result as *mut u8, result - data_ptr + size))
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
    let mut ptr = match NonNull::new(unsafe { alloc(layout) }.cast::<Page>()) {
      Some(ptr) => ptr,
      None => handle_alloc_error(layout),
    };

    // Safety: ptr 是刚分配成功的块（alloc 返回非空即内存到手），next/alloc_size/data
    // 均为该块内的普通字段；此处只做初始化写入（先写后用，无未初始化读取），
    // data 仅取基址、不越界。
    let page = unsafe { ptr.as_mut() };
    // 新页压顶，旧链（可能为 None = cpp 的链尾 nullptr）整体成为本页的 next。
    page.next = self.root;
    // 记录精确分配大小，Drop 时用同一 Layout 释放（超大请求会超额分配）。
    page.alloc_size = layout.size();

    self.root = Some(ptr);
    self.offset = size;

    page.data.as_mut_ptr()
  }
}
