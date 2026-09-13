use core::mem::size_of;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_throw_ldo::luaD_throw, newgcoblock::newgcoblock, newpage::newpage},
  macros::{asan_unpoison_memory_region::ASAN_UNPOISON_MEMORY_REGION, sizeclass::sizeclass},
  records::{g_cheader::GCheader, gc_object::GCObject, lua_page::lua_Page},
  type_aliases::lua_state::lua_State,
};

const K_GCO_LINK_OFFSET: usize =
  (size_of::<GCheader>() + size_of::<*mut u8>() - 1) & !(size_of::<*mut u8>() - 1);

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_m_newgco(l: *mut lua_State, nsize: usize, memcat: u8) -> *mut GCObject {
  unsafe {
    LUAU_ASSERT!(nsize >= K_GCO_LINK_OFFSET + size_of::<*mut u8>());

    let g = (*l).global;
    let nclass = sizeclass!(nsize) as i32;

    let block = if nclass >= 0 {
      newgcoblock(l, nclass)
    } else {
      let page = newpage(
        l,
        core::ptr::addr_of_mut!((*g).allgcopages),
        (core::mem::offset_of!(lua_Page, data) + nsize) as i32,
        nsize as i32,
        1,
      );

      let block = (*page).data.as_mut_ptr() as *mut u8;
      ASAN_UNPOISON_MEMORY_REGION!(block, (*page).block_size as usize);

      (*page).free_next -= (*page).block_size;
      (*page).busy_blocks += 1;
      block
    };

    if block.is_null() && nsize > 0 {
      luaD_throw(l, LuaStatus::ErrMem as i32);
    }

    (*g).totalbytes = (*g).totalbytes.wrapping_add(nsize);
    (*g).memcatbytes[memcat as usize] = (*g).memcatbytes[memcat as usize].wrapping_add(nsize);

    if let Some(onallocate) = (*g).cb.onallocate {
      onallocate(l, 0, nsize);
    }

    block as *mut GCObject
  }
}

pub use lua_m_newgco as luaM_newgco_;
