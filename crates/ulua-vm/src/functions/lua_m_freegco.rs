use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{freegcoblock::freegcoblock, freepage::freepage},
  macros::sizeclass::sizeclass,
  records::{gc_object::GCObject, lua_page::lua_Page},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_m_freegco(
  l: *mut lua_State,
  block: *mut GCObject,
  osize: usize,
  memcat: u8,
  page: *mut lua_Page,
) {
  unsafe {
    let g = (*l).global;
    LUAU_ASSERT!((osize == 0) == block.is_null());

    // 与 lua_m_realloc_/lua_m_free 共用 sizeclass! 查表，语义与手写算术版一致
    let oclass = sizeclass!(osize) as i32;

    if oclass >= 0 {
      (*block).gch.tt = LuaType::Nil as u8;

      freegcoblock(l, oclass, block as *mut u8, page);
    } else {
      LUAU_ASSERT!((*page).busy_blocks == 1);
      LUAU_ASSERT!((*page).block_size as usize == osize);
      LUAU_ASSERT!(block as *mut u8 == core::ptr::addr_of_mut!((*page).data) as *mut u8);

      freepage(l, core::ptr::addr_of_mut!((*g).allgcopages), page);
    }

    (*g).totalbytes = (*g).totalbytes.wrapping_sub(osize);
    (*g).memcatbytes[memcat as usize] = (*g).memcatbytes[memcat as usize].wrapping_sub(osize);
  }
}

pub use lua_m_freegco as luaM_freegco_;
