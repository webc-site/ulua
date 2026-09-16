use core::ptr::{addr_of_mut, eq};

use crate::{
  enums::lua_type::LuaType,
  functions::maybesetaboundary::maybesetaboundary,
  macros::{dummynode::dummynode, setnilvalue::setnilvalue, sizenode::sizenode},
  records::lua_table::LuaTable,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_h_clear(tt: *mut LuaTable) {
  unsafe {
    let mut i = 0;
    while i < (*tt).sizearray {
      setnilvalue!((*tt).array.add(i as usize));
      i += 1;
    }

    maybesetaboundary(tt, 0);

    if !eq((*tt).node, dummynode) {
      let size = sizenode!(tt);
      (*tt).union.lastfree = size;

      let mut i = 0;
      while i < size {
        let n = (*tt).node.add(i as usize);
        (*n).key.value = Default::default();
        (*n).key.extra = [0];
        (*n).key.set_tt(LuaType::Nil as i32);
        setnilvalue!(addr_of_mut!((*n).val));
        (*n).key.set_next(0);
        i += 1;
      }
    }

    (*tt).tmcache = !0u8;
  }
}

pub use lua_h_clear as luaH_clear;
