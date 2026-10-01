use core::{mem::offset_of, ptr::null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::lua_d_throw_ldo::lua_d_throw,
  macros::asan_poison_memory_region::ASAN_POISON_MEMORY_REGION,
  records::{global_state::global_State, lua_page::lua_Page, lua_state::LuaState},
};

/// # Safety
/// 调用方须保证：`l` 存活且处于受保护帧（frealloc 返 null 时经 `l` 抛 ErrMem）；
/// `page_size >= offset_of!(lua_Page, data) + block_size * block_count`（release 下断言失效，
/// 不足即越界写页头/块区）；`pageset` 为 null 或指向可写的存活页链头指针。cpp lmem.cpp:276 `newpage`
pub(crate) unsafe fn newpage(
  l: *mut LuaState,
  pageset: *mut *mut lua_Page,
  page_size: i32,
  block_size: i32,
  block_count: i32,
) -> *mut lua_Page {
  // SAFETY: 契约保证 `l` 存活且大小经分配器协议给出，新页挂入 pageset 链前字段已按页头自洽初始化
  unsafe {
    let g: *mut global_State = (*l).global;

    LUAU_ASSERT!(page_size - (offset_of!(lua_Page, data) as i32) >= block_size * block_count);

    let frealloc_fn = (*g).frealloc;
    // 既有约定（review.md §2）：alloc 回调契约返回 null——oldptr=null+osize=0 即纯分配请求，分配器失败/无回调时页指针为 null
    let page = if let Some(f) = frealloc_fn {
      f((*g).ud, null_mut(), 0, page_size as usize) as *mut lua_Page
    } else {
      null_mut()
    };

    if page.is_null() {
      lua_d_throw(l, LuaStatus::ErrMem as i32);
    }

    ASAN_POISON_MEMORY_REGION!((*page).data.as_ptr(), (block_size * block_count) as usize);

    // setup page header
    // 既有约定（review.md §2）：GC 页/块链表结构哨兵——prev/next/listprev/listnext/free_list 空即未挂链/空 freelist，链表头尾裸指针哨兵
    (*page).prev = null_mut();
    (*page).next = null_mut();

    (*page).listprev = null_mut();
    (*page).listnext = null_mut();

    (*page).page_size = page_size;
    (*page).block_size = block_size;

    // note: we start with the last block in the page and move downward
    // either order would work, but that way we don't need to store the block count in the page
    // additionally, GC stores objects in singly linked lists, and this way the GC lists end up in increasing pointer order
    (*page).free_list = null_mut();
    (*page).free_next = (block_count - 1) * block_size;
    (*page).busy_blocks = 0;

    if !pageset.is_null() {
      (*page).listnext = *pageset;
      if !(*page).listnext.is_null() {
        (*(*page).listnext).listprev = page;
      }
      *pageset = page;
    }

    page
  }
}
