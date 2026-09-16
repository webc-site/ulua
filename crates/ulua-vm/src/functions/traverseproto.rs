//! Node: `cxx:Function:Luau.VM:VM/src/lgc.cpp:372:traverseproto`
//! Source: `VM/src/lgc.cpp` (lgc.cpp:372-396, hand-ported)

use crate::{
  functions::c_slice,
  macros::{markobject::markobject, markvalue::markvalue, stringmark::stringmark},
  records::{global_state::global_State, proto::Proto},
};

// All marks are conditional because a GC may happen while the
// prototype is still being created
pub(crate) unsafe fn traverseproto(g: *mut global_State, f: *mut Proto) {
  unsafe {
    if !(*f).source.is_null() {
      stringmark!((*f).source);
    }
    if !(*f).debugname.is_null() {
      stringmark!((*f).debugname);
    }
    // SAFETY：k/upvalues/p/locvars 均为 C 指针 + 计数，
    // Proto 构建完成后长度与分配一致（luaF_newproto 一次性分配定型）。
    for k in c_slice((*f).k, (*f).sizek as usize) {
      // mark literals
      markvalue!(g, k);
    }
    for &upvalue in c_slice((*f).upvalues, (*f).sizeupvalues as usize) {
      // mark upvalue names
      if !upvalue.is_null() {
        stringmark!(upvalue);
      }
    }
    for &p in c_slice((*f).p, (*f).sizep as usize) {
      // mark nested protos
      if !p.is_null() {
        markobject!(g, p);
      }
    }
    for locvar in c_slice((*f).locvars, (*f).sizelocvars as usize) {
      // mark local-variable names
      let varname = locvar.varname;
      if !varname.is_null() {
        stringmark!(varname);
      }
    }
  }
}
