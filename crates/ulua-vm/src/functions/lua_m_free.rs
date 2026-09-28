use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::freeblock::freeblock, macros::sizeclass::sizeclass, records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 且 `(*l).global` 有效（更新 `totalbytes`/`memcatbytes[memcat]` 记账）；`block` 须为先前
/// 由同尺寸 `osize` 分配的块或 `NULL`（`LUAU_ASSERT` 要求 `osize == 0` 当且仅当 `block` 为空）；`memcat` 须 `< MEMCAT__COUNT`
/// 以在界内索引 `memcatbytes`。小尺寸走 `freeblock` 页内回收，否则调 `frealloc` 用户回调。cpp/VM/src/lmem.cpp:674 luaM_free_。
pub unsafe fn lua_m_free(l: *mut LuaState, block: *mut u8, osize: usize, memcat: u8) {
  unsafe {
    let g = (*l).global;
    LUAU_ASSERT!((osize == 0) == block.is_null());

    // 与 lua_m_realloc_ 共用 sizeclass! 查表，语义与手写算术版一致（已全量比对）
    let oclass = sizeclass!(osize) as i32;

    if oclass >= 0 {
      freeblock(l, oclass, block);
    } else if let Some(frealloc) = (*g).frealloc {
      frealloc((*g).ud, block, osize, 0);
    }

    (*g).totalbytes = (*g).totalbytes.wrapping_sub(osize);
    let memcatbytes = &mut (*g).memcatbytes;
    memcatbytes[memcat as usize] = memcatbytes[memcat as usize].wrapping_sub(osize);
  }
}
