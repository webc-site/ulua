//! Source: `VM/src/lmem.cpp:599-642`（onallocate 调用在 821） (hand-fixed: the translated version
//! invented a power-of-2 size-class scheme that disagreed with the real
//! progressive table in `sizeclass!`, so realloc'd blocks were freed under
//! the wrong class and tripped the blockSize assert in `freeblock`)

use core::{
  ffi::c_void,
  ptr::{copy_nonoverlapping, null_mut},
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{lua_status::LuaStatus, lua_type::LuaType},
  functions::{freeblock::freeblock, lua_d_throw_ldo::lua_d_throw, newblock::newblock},
  macros::sizeclass::sizeclass,
  records::lua_state::LuaState,
};

/// # Safety
/// 调用方须保证：`block` 为空且 `osize==0`，或为本分配器早前分配、长度恰为 `osize` 的存活块；
/// `l` 存活且 `(*l).global` 完整（frealloc 非空并遵守 realloc 契约，`memcat` 落在 memcatbytes 数组界内）；
/// nsize>0 且 OOM 时经 `l` 抛 ErrMem 不返回，须在受保护帧内调用。cpp lmem.cpp:779 `luaM_realloc_`
pub(crate) unsafe fn lua_m_realloc_(
  l: *mut LuaState,
  block: *mut u8,
  osize: usize,
  nsize: usize,
  memcat: u8,
) -> *mut u8 {
  // Safety: 契约保证 `ptr..ptr+osize` 与 nsize 新块均按分配器协议有效可读/可写，nsize>0 时 OOM 走统一错误路径
  unsafe {
    let g = (*l).global;
    LUAU_ASSERT!((osize == 0) == (block.is_null()));

    // frealloc 由 lstate 创建期保证非空（cpp: lua_assert(g->frealloc)）
    let frealloc = (*g)
      .frealloc
      .expect("global_State.frealloc 由 lstate 创建期契约保证非空");
    let ud = (*g).ud;

    let nclass = sizeclass!(nsize) as i32;
    let oclass = sizeclass!(osize) as i32;
    let result: *mut u8;

    // if either block needs to be allocated using a block allocator, we can't use realloc directly
    if nclass >= 0 || oclass >= 0 {
      result = if nclass >= 0 {
        newblock(l, nclass)
      } else {
        frealloc(ud, null_mut(), 0, nsize)
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
        frealloc(ud, block, osize, 0);
      }
    } else {
      result = frealloc(ud, block, osize, nsize);
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

    if let Some(onallocate) = (*g).cb.onallocate {
      // cpp lmem.cpp:821：实参为新块指针 result（旧块已释放，传旧指针即
      // use-after-free 风险）；tt 为 LUA_T_ALL 哨兵，tag=0
      onallocate(
        l,
        result.cast::<c_void>(),
        osize,
        nsize,
        memcat,
        LuaType::ALL_SENTINEL,
        0,
      );
    }

    result
  }
}
