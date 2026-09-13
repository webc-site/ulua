//! Node: `cxx:Function:Luau.VM:VM/src/lgcdebug.cpp:115:validateproto`
//!
//! GC heap-validation: assert that every GC reference reachable from a `Proto`
//! (source, debugname, constants, upvalues, child protos, local-var names)
//! points at a live, correctly-colored object.

use crate::{
  functions::{c_slice, validateobjref::validateobjref, validateref::validateref},
  macros::obj_2_gco::obj2gco,
  records::{global_state::global_State, proto::Proto},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn validateproto(g: *mut global_State, f: *mut Proto) {
  unsafe {
    if !(*f).source.is_null() {
      validateobjref(g, obj2gco!(f), obj2gco!((*f).source));
    }

    if !(*f).debugname.is_null() {
      validateobjref(g, obj2gco!(f), obj2gco!((*f).debugname));
    }

    // SAFETY：k/upvalues/p/locvars 为 C 指针 + 计数，Proto 分配时长度定型。
    for k in c_slice((*f).k, (*f).sizek as usize) {
      validateref(g, obj2gco!(f), k);
    }

    for &up in c_slice((*f).upvalues, (*f).sizeupvalues as usize) {
      if !up.is_null() {
        validateobjref(g, obj2gco!(f), obj2gco!(up));
      }
    }

    for &p in c_slice((*f).p, (*f).sizep as usize) {
      if !p.is_null() {
        validateobjref(g, obj2gco!(f), obj2gco!(p));
      }
    }

    for lv in c_slice((*f).locvars, (*f).sizelocvars as usize) {
      if !lv.varname.is_null() {
        validateobjref(g, obj2gco!(f), obj2gco!(lv.varname));
      }
    }
  }
}
