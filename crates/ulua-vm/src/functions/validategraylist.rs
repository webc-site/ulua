//! `validategraylist` — validate every node in a GC gray list.
//! C++ source: `VM/src/lgcdebug.cpp:218`
//!
//! Walks the singly-linked gray list starting at `o`; asserts each node is
//! still gray and follows the per-type `gclist` pointer to the next node.
//! Returns immediately if the GC invariant is not active (sweep phase etc.).

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  macros::{
    gco_2_cl::gco2cl, gco_2_class::gco2class, gco_2_h::gco2h, gco_2_object::gco2object,
    gco_2_p::gco2p, gco_2_th::gco2th, isgray::isgray, keepinvariant::keepinvariant,
  },
  records::{gc_object::GCObject, global_state::global_State},
};

pub(crate) unsafe fn validategraylist(g: *mut global_State, mut o: *mut GCObject) {
  unsafe {
    if !keepinvariant(g) {
      return;
    }

    while !o.is_null() {
      LUAU_ASSERT!(isgray!(o));

      match (*o).gch.tt as i32 {
        t if t == LuaType::Table as i32 => {
          o = (*gco2h!(o)).gclist;
        }
        t if t == LuaType::Function as i32 => {
          o = (*gco2cl!(o)).gclist;
        }
        t if t == LuaType::Thread as i32 => {
          o = (*gco2th!(o)).gclist;
        }
        t if t == LuaType::Class as i32 => {
          o = (*gco2class!(o)).gclist;
        }
        t if t == LuaType::Object as i32 => {
          o = (*gco2object!(o)).gclist;
        }
        t if t == LuaType::Proto as i32 => {
          o = (*gco2p!(o)).gclist;
        }
        _ => {
          LUAU_ASSERT!(false);
          return;
        }
      }
    }
  }
}
