use core::{
  mem::offset_of,
  ptr::{null_mut, NonNull},
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::lua_d_throw_ldo::lua_d_throw,
  macros::asan_poison_memory_region::ASAN_POISON_MEMORY_REGION,
  records::{lua_page::lua_Page, lua_state::LuaState},
};

/// # Safety
/// 调用方须保证：`l` 存活且处于受保护帧（frealloc 返 null 时经 `l` 抛 ErrMem）；
/// `page_size >= offset_of!(lua_Page, data) + block_size * block_count`（release 下断言失效，
/// 不足即越界写页头/块区）；`pageset` 以 `&mut` 借用非空，指向可写的存活页链头指针。cpp lmem.cpp:276 `newpage`
pub(crate) fn newpage(
  l: *mut LuaState,
  pageset: &mut *mut lua_Page,
  page_size: i32,
  block_size: i32,
  block_count: i32,
) -> *mut lua_Page {
  // 一句一借（gs_ref 契约）：取出分配器回调与 ud 后立即结束对 global_State 的只读
  // 借用，绝不把该借用跨 `frealloc` 这个可重入的宿主分配器调用持有
  let (frealloc_fn, ud) = {
    // SAFETY: 契约保证 `l` 存活、`(*l).global` 指向有效 global_State
    let g = unsafe { &*(*l).global };
    (g.frealloc, g.ud)
  };

  LUAU_ASSERT!(page_size - (offset_of!(lua_Page, data) as i32) >= block_size * block_count);

  // 既有约定（review.md §2）：frealloc 是宿主 C 分配器回调（FFI 边界），
  // oldptr=null + osize=0 即纯分配请求；无回调或分配失败即抛 ErrMem。
  // SAFETY: FFI 边界——契约保证 `frealloc_fn` 满足 lua_Alloc 语义、`ud` 由宿主保活
  let raw = match frealloc_fn {
    Some(f) => unsafe { f(ud, null_mut(), 0, page_size as usize) },
    None => null_mut(),
  };

  // 分配失败（或无回调）即抛 ErrMem；`lua_d_throw` 发散不返回，故其后页指针必非空
  let Some(mut page_ptr) = NonNull::new(raw.cast::<lua_Page>()) else {
    lua_d_throw(l, LuaStatus::ErrMem as i32);
  };

  // SAFETY: 契约保证新页非空、按 lua_Page 对齐且容纳整页；在归还前本函数独占该块，
  // `&mut` 视图仅在此借用窗内使用
  let page = unsafe { page_ptr.as_mut() };

  ASAN_POISON_MEMORY_REGION!(page.data.as_ptr(), (block_size * block_count) as usize);

  // 页头链表字段既有约定（review.md §2）：`lua_Page` 是 repr(C) 页分配器 ABI 结构，
  // prev/next/listprev/listnext/free_list 的 null 即「未挂链 / 空 freelist」结构哨兵
  // （字段改型属后续 arena 阶段，本阶段维持裸指针），此处以具名字段赋值取代手工解引用
  page.prev = null_mut();
  page.next = null_mut();
  page.listprev = null_mut();
  page.listnext = null_mut();

  page.page_size = page_size;
  page.block_size = block_size;

  // note: we start with the last block in the page and move downward
  // either order would work, but that way we don't need to store the block count in the page
  // additionally, GC stores objects in singly linked lists, and this way the GC lists end up in increasing pointer order
  page.free_list = null_mut();
  page.free_next = (block_count - 1) * block_size;
  page.busy_blocks = 0;

  // 挂入页链头：pageset 经 `&mut` 借用非空，全部调用方均传可写链头，判空死分支删除（review.md §2）
  page.listnext = *pageset;
  if let Some(mut head) = NonNull::new(*pageset) {
    // SAFETY: 契约保证 `*pageset` 指向存活的已挂链页头；旧链头的 listprev 回填为新页
    unsafe { head.as_mut().listprev = page_ptr.as_ptr() };
  }
  *pageset = page_ptr.as_ptr();

  page_ptr.as_ptr()
}
