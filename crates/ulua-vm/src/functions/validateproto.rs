//! Source: `VM/src/lgcdebug.cpp:115`
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
/// 输入字节流缓冲与读游标必须位于本次加载的串数据界内，长度参数覆盖所有被读字段。
pub(crate) unsafe fn validateproto(g: *mut global_State, f: *mut Proto) {
  // Safety: 契约保证读游标位于输入字节流界内且剩余长度覆盖所读字段，块内逐字段读取在失败时立即返回错误码而非越界
  unsafe {
    if !(*f).source.is_null() {
      validateobjref(g, obj2gco!(f), obj2gco!((*f).source));
    }

    if !(*f).debugname.is_null() {
      validateobjref(g, obj2gco!(f), obj2gco!((*f).debugname));
    }

    // Safety:k/upvalues/p/locvars 为 C 指针 + 计数，Proto 分配时长度定型。
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
