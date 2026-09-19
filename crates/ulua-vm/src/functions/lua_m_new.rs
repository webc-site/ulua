use core::ptr::null_mut;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_throw_ldo::luaD_throw, newblock::newblock},
  macros::sizeclass::sizeclass,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_m_new(l: *mut lua_State, nsize: usize, memcat: u8) -> *mut u8 {
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
      luaD_throw(l, LuaStatus::ErrMem as i32);
    }

    (*g).totalbytes = (*g).totalbytes.wrapping_add(nsize);
    (*g).memcatbytes[memcat as usize] = (*g).memcatbytes[memcat as usize].wrapping_add(nsize);

    if let Some(onallocate) = (*g).cb.onallocate {
      onallocate(l, 0, nsize);
    }

    block
  }
}

pub use lua_m_new as luaM_new_;
