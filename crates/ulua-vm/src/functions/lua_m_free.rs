use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::freeblock::freeblock, macros::sizeclass::sizeclass, type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_m_free(l: *mut lua_State, block: *mut u8, osize: usize, memcat: u8) {
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
    (*g).memcatbytes[memcat as usize] = (*g).memcatbytes[memcat as usize].wrapping_sub(osize);
  }
}

pub use lua_m_free as luaM_free_;
