//! Source: `VM/src/lgcdebug.cpp:115`
//!
//! GC heap-validation: assert that every GC reference reachable from a `Proto`
//! (source, debugname, constants, upvalues, child protos, local-var names)
//! points at a live, correctly-colored object.

use core::ptr::from_ref;

use crate::{
  functions::{c_slice, validateobjref::validateobjref, validateref::validateref},
  macros::obj_2_gco::obj2gco,
  records::{gc_object::GCObject, global_state::global_State, proto::Proto},
};

/// # Safety
/// `g` 须指向存活 global_State；`p` 须为存活 Proto，其 `k`/`upvalues`/`p`/`locvars`
/// 计数与分配一致（校验仅读取引用字段与着色位）。
pub(crate) unsafe fn validateproto(g: *mut global_State, p: &Proto) {
  unsafe {
    // 引用源身份：只读校验，从共享借用降 `*const` 即够用
    let obj: *const GCObject = from_ref(p).cast();

    if !p.source.is_null() {
      validateobjref(g, obj, obj2gco!(p.source));
    }

    if !p.debugname.is_null() {
      validateobjref(g, obj, obj2gco!(p.debugname));
    }

    // SAFETY:k/upvalues/p/locvars 为 C 指针 + 计数，Proto 分配时长度定型。
    for k in c_slice(p.k, p.sizek as usize) {
      validateref(g, obj, k);
    }

    for &up in c_slice(p.upvalues, p.sizeupvalues as usize) {
      if !up.is_null() {
        validateobjref(g, obj, obj2gco!(up));
      }
    }

    for &child in c_slice(p.p, p.sizep as usize) {
      if !child.is_null() {
        validateobjref(g, obj, obj2gco!(child));
      }
    }

    for lv in c_slice(p.locvars, p.sizelocvars as usize) {
      if !lv.varname.is_null() {
        validateobjref(g, obj, obj2gco!(lv.varname));
      }
    }
  }
}
