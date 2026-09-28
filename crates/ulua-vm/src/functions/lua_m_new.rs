use core::{ffi::c_void, ptr::null_mut};

use crate::{
  enums::{lua_status::LuaStatus, lua_type::LuaType},
  functions::{lua_d_throw_ldo::lua_d_throw, newblock::newblock},
  macros::sizeclass::sizeclass,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `(*l).global`（含 `frealloc/ud/cb/totalbytes/memcatbytes`）有效；
/// `memcat` 须为合法内存类别下标（用于索引 `memcatbytes[..]`，越界即 UB）；分配失败且 `nsize>0` 时
/// `luaD_throw(ErrMem)` 抛错；`nsize==0` 可返回 NULL。cpp `lmem.cpp:543`。
pub unsafe fn lua_m_new(l: *mut LuaState, nsize: usize, memcat: u8) -> *mut u8 {
  unsafe {
    let g = (*l).global;
    let nclass = sizeclass!(nsize) as i32;

    let block = if nclass >= 0 {
      newblock(l, nclass)
    } else if let Some(frealloc) = (*g).frealloc {
      frealloc((*g).ud, null_mut(), 0, nsize)
    } else {
      null_mut()
    };

    if block.is_null() && nsize > 0 {
      lua_d_throw(l, LuaStatus::ErrMem as i32);
    }

    (*g).totalbytes = (*g).totalbytes.wrapping_add(nsize);
    let memcatbytes = &mut (*g).memcatbytes;
    memcatbytes[memcat as usize] = memcatbytes[memcat as usize].wrapping_add(nsize);

    if let Some(onallocate) = (*g).cb.onallocate {
      // cpp lmem.cpp:558：tt 为 LUA_T_ALL 哨兵（非真实类型 tag），tag=0
      onallocate(
        l,
        block.cast::<c_void>(),
        0,
        nsize,
        memcat,
        LuaType::ALL_SENTINEL,
        0,
      );
    }

    block
  }
}
