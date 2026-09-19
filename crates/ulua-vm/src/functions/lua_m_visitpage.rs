use core::{
  ffi::{c_char, c_void},
  ptr::null_mut,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::lua_m_getpagewalkinfo::lua_m_getpagewalkinfo,
  records::{gc_object::GCObject, lua_page::lua_Page},
  type_aliases::gc_visitor::GcVisitor,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_m_visitpage(
  page: *mut lua_Page,
  context: *mut c_void,
  visitor: GcVisitor,
) {
  unsafe {
    let mut start: *mut c_char = null_mut();
    let mut end: *mut c_char = null_mut();
    let mut busy_blocks: i32 = 0;
    let mut block_size: i32 = 0;

    lua_m_getpagewalkinfo(
      page,
      &mut start,
      &mut end,
      &mut busy_blocks,
      &mut block_size,
    );

    let mut pos = start;
    while pos != end {
      let gco = pos as *mut GCObject;

      // skip memory blocks that are already freed
      if (*gco).gch.tt == LuaType::Nil as u8 {
        pos = pos.add(block_size as usize);
        continue;
      }

      // when true is returned it means that the element was deleted
      if visitor(context, page, gco) {
        LUAU_ASSERT!(busy_blocks > 0);

        // if the last block was removed, page would be removed as well
        busy_blocks -= 1;
        if busy_blocks == 0 {
          break;
        }
      }

      pos = pos.add(block_size as usize);
    }
  }
}
