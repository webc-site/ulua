//! Node: `cxx:Function:Luau.VM:VM/src/lmem.cpp:599:luaM_realloc_`
//! Source: `VM/src/lmem.cpp:599-640` (hand-fixed: the translated version
//! invented a power-of-2 size-class scheme that disagreed with the real
//! progressive table in `sizeclass!`, so realloc'd blocks were freed under
//! the wrong class and tripped the blockSize assert in `freeblock`)

use core::ptr::{copy_nonoverlapping, null_mut};

use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_unlikely::LUAU_UNLIKELY};

use crate::{
  enums::lua_status::LuaStatus,
  functions::{freeblock::freeblock, lua_d_throw_ldo::lua_d_throw, newblock::newblock},
  macros::sizeclass::sizeclass,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn lua_m_realloc_(
  l: *mut lua_State,
  block: *mut u8,
  osize: usize,
  nsize: usize,
  memcat: u8,
) -> *mut u8 {
  unsafe {
    let g = (*l).global;
    LUAU_ASSERT!((osize == 0) == (block.is_null()));

    // frealloc 由 lstate 创建期保证非空（cpp: lua_assert(g->frealloc)）
    let frealloc = (*g).frealloc.expect("frealloc is null");

    let nclass = sizeclass!(nsize) as i32;
    let oclass = sizeclass!(osize) as i32;
    let result: *mut u8;

    // if either block needs to be allocated using a block allocator, we can't use realloc directly
    if nclass >= 0 || oclass >= 0 {
      result = if nclass >= 0 {
        newblock(l, nclass)
      } else {
        frealloc((*g).ud, null_mut(), 0, nsize)
      };

      if result.is_null() && nsize > 0 {
        lua_d_throw(l, LuaStatus::ErrMem as i32);
      }

      if osize > 0 && nsize > 0 {
        // 复制重叠区间的较小者，osize/nsize 已保证边界安全
        let copy_size = osize.min(nsize);
        copy_nonoverlapping(block, result, copy_size);
      }

      if oclass >= 0 {
        freeblock(l, oclass, block);
      } else {
        frealloc((*g).ud, block, osize, 0);
      }
    } else {
      result = frealloc((*g).ud, block, osize, nsize);
      if result.is_null() && nsize > 0 {
        lua_d_throw(l, LuaStatus::ErrMem as i32);
      }
    }

    LUAU_ASSERT!((nsize == 0) == (result.is_null()));
    (*g).totalbytes = (*g).totalbytes.wrapping_sub(osize).wrapping_add(nsize);
    let memcat_bytes = &mut (*g).memcatbytes;
    memcat_bytes[memcat as usize] = memcat_bytes[memcat as usize]
      .wrapping_add(nsize)
      .wrapping_sub(osize);

    if LUAU_UNLIKELY!((*g).cb.onallocate.is_some()) {
      ((*g).cb.onallocate.unwrap_unchecked())(l, osize, nsize);
    }

    result
  }
}
