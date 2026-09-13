//! Node: `cxx:Function:Luau.VM:VM/src/lgc.cpp:805:markroot`
//! Source: `VM/src/lgc.cpp` (lgc.cpp:805-837, hand-ported)

use core::ptr::null_mut;

use ulua_common::FFlag;

use crate::{
  functions::markmt::markmt,
  macros::{gc_spropagate::GCSPROPAGATE, markobject::markobject, markvalue::markvalue},
  type_aliases::lua_state::lua_State,
};

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
    markvalue!(g, core::ptr::addr_of_mut!((*g).registry));

    if FFlag::LuauUdataDirectAccess6.get() {
      for udatadirect in (*g).udatadirect.iter_mut() {
        markvalue!(g, core::ptr::addr_of_mut!(udatadirect.indextm));
        markvalue!(g, core::ptr::addr_of_mut!(udatadirect.newindextm));
        markvalue!(g, core::ptr::addr_of_mut!(udatadirect.namecalltm));
      }
    }

    if FFlag::LuauDirectFieldGet.get() {
      for &field in (*g).udatadirectfields.iter() {
        if !field.is_null() {
          markobject!(g, field);
        }
      }
    }

    markmt(g);
    (*g).gcstate = GCSPROPAGATE as u8;
  }
}
