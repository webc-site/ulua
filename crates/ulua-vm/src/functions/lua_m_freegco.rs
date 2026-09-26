use core::ptr::addr_of_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{freegcoblock::freegcoblock, freepage::freepage},
  macros::sizeclass::sizeclass,
  records::{gc_object::GCObject, lua_page::lua_Page, lua_state::LuaState},
};

/// # Safety
/// `l` 须存活且 `g` 记账可用；`block` 必须是分配器产出、尚未归还的 GCObject 且
/// `osize == 0 ⟺ block == null`（cpp lmem.cpp:695）：小对象经 `sizeclass!` 查表归还页链，
/// 大对象则 `page` 必须是 `block` 所属单页（`busy_blocks == 1` 断言兜底），`memcat` 须与
/// 分配时一致以回退 `totalbytes/memcatbytes` 记账。
pub unsafe fn lua_m_freegco(
  l: *mut LuaState,
  block: *mut GCObject,
  osize: usize,
  memcat: u8,
  page: *mut lua_Page,
) {
  // Safety: 契约保证 `block` 为分配器产出、未归还的存活 GCObject，先解除内存记账再按 size 归还页块
  unsafe {
    let g = (*l).global;
    LUAU_ASSERT!((osize == 0) == block.is_null());

    // 与 lua_m_realloc_/lua_m_free 共用 sizeclass! 查表，语义与手写算术版一致
    let oclass = sizeclass!(osize) as i32;

    if oclass >= 0 {
      (*block).header_mut().tt = LuaType::Nil as u8;

      freegcoblock(l, oclass, block as *mut u8, page);
    } else {
      let p = &*page;
      LUAU_ASSERT!(p.busy_blocks == 1);
      LUAU_ASSERT!(p.block_size as usize == osize);
      LUAU_ASSERT!(block as *mut u8 == addr_of_mut!((*page).data) as *mut u8);

      freepage(l, addr_of_mut!((*g).allgcopages), page);
    }

    (*g).totalbytes = (*g).totalbytes.wrapping_sub(osize);
    let memcatbytes = &mut (*g).memcatbytes;
    memcatbytes[memcat as usize] = memcatbytes[memcat as usize].wrapping_sub(osize);
  }
}
