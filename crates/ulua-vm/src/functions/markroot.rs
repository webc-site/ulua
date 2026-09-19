//! Source: `VM/src/lgc.cpp` (lgc.cpp:805-837, hand-ported)

use core::ptr::{addr_of_mut, null_mut};

use ulua_common::fflag;

use crate::{
  functions::{markmt::markmt, marktaggetmt::marktaggetmt},
  macros::{gc_spropagate::GCSPROPAGATE, markobject::markobject, markvalue::markvalue},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
// mark root set
pub(crate) unsafe fn markroot(l: *mut lua_State) {
  unsafe {
    let g = (*l).global;
    (*g).gray = null_mut();
    (*g).grayagain = null_mut();
    (*g).weak = null_mut();
    markobject!(g, (*g).mainthread);
    // make global table be traversed before main stack
    markobject!(g, (*(*g).mainthread).gt);
    // registry(l) — &l->global->registry
    markvalue!(g, addr_of_mut!((*g).registry));

    if fflag::LuauUdataDirectAccess6.get() {
      for udatadirect in (*g).udatadirect.iter_mut() {
        markvalue!(g, addr_of_mut!(udatadirect.indextm));
        markvalue!(g, addr_of_mut!(udatadirect.newindextm));
        markvalue!(g, addr_of_mut!(udatadirect.namecalltm));
      }
    }

    if fflag::LuauDirectFieldGet.get() {
      for &field in (*g).udatadirectfields.iter() {
        if !field.is_null() {
          markobject!(g, field);
        }
      }
    }

    markmt(g);
    // cpp lgc.cpp:930：补标标签 userdata 元表（GC 根之一，缺失会导致元表被 sweep）
    marktaggetmt(g);
    (*g).gcstate = GCSPROPAGATE as u8;
  }
}
