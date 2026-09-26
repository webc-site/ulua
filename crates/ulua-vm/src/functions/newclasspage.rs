use core::mem::{offset_of, size_of};

use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_noinline::LUAU_NOINLINE};

use crate::{
  functions::newpage::newpage,
  records::{lua_page::lua_Page, lua_state::LuaState, size_class_config::K_SIZE_CLASS_CONFIG},
};

// cpp `lmem.cpp:139-146`：大对象（> 512B）用大页；页尺寸统一减 24B 以避开
// 外部分配器的元数据取整浪费。
pub(crate) const K_EXTERNAL_ALLOCATOR_META_DATA_REDUCTION: i32 = 24;
pub(crate) const K_LARGE_PAGE_THRESHOLD: i32 = 512;
pub(crate) const K_SMALL_PAGE_SIZE: i32 = 16 * 1024 - K_EXTERNAL_ALLOCATOR_META_DATA_REDUCTION;
pub(crate) const K_LARGE_PAGE_SIZE: i32 = 32 * 1024 - K_EXTERNAL_ALLOCATOR_META_DATA_REDUCTION;
/// cpp `kBlockHeader`：足以在所有平台上对齐 double 与 void*。
pub(crate) const K_BLOCK_HEADER: i32 = if size_of::<f64>() > size_of::<*mut ()>() {
  size_of::<f64>() as i32
} else {
  size_of::<*mut ()>() as i32
};

LUAU_NOINLINE! {
/// # Safety
/// `l` 必须指向存活 `LuaState` 且 `size_class` 在 K_SIZE_CLASS_CONFIG 表界内；`freepageset`/`pageset` 指向对应 size 类的可写页链头。
    pub(crate) unsafe fn newclasspage(
        l: *mut LuaState,
        freepageset: *mut *mut lua_Page,
        pageset: *mut *mut lua_Page,
        size_class: u8,
        store_metadata: bool,
    // Safety: 契约保证 `l` 存活且 size_class 在 K_SIZE_CLASS_CONFIG 表界内，新页经 luaM_realloc_ 分配并按 size 类元数据自洽
    ) -> *mut lua_Page { unsafe {
        let size_of_class = K_SIZE_CLASS_CONFIG.size_of_class[size_class as usize];
        let page_size = if size_of_class > K_LARGE_PAGE_THRESHOLD {
            K_LARGE_PAGE_SIZE
        } else {
            K_SMALL_PAGE_SIZE
        };
        let block_size = size_of_class + if store_metadata { K_BLOCK_HEADER } else { 0 };
        let block_count = (page_size - offset_of!(lua_Page, data) as i32) / block_size;

        let page = newpage(l, pageset, page_size, block_size, block_count);

        LUAU_ASSERT!((*freepageset.add(size_class as usize)).is_null());
        *freepageset.add(size_class as usize) = page;

        page
    }}
}
