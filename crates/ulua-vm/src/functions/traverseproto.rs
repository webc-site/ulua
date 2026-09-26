//! Source: `VM/src/lgc.cpp` (lgc.cpp:372-396, hand-ported)

use crate::{
  functions::c_slice,
  macros::{markobject::markobject, markvalue::markvalue, stringmark::stringmark},
  records::{global_state::global_State, proto::Proto},
};

/// # Safety
/// 调用方须保证：`g` 处于 GC mark 阶段且存活；`f` 为构建完成的 Proto——sizek/sizeupvalues/sizep/
/// sizelocvars 与各数组实际分配一致（源构建期允许 source/debugname/upvalue 名为空，函数已逐项判空）；
/// 违反即越界扫描。cpp lgc.cpp:394 `traverseproto`（lgc.cpp:372-396 段）
// All marks are conditional because a GC may happen while the
// prototype is still being created
pub(crate) unsafe fn traverseproto(g: *mut global_State, f: *mut Proto) {
  // Safety: 契约保证 `f` 为存活 Proto 且 setpointer 回调满足写引用协议，块内逐常量/upval 遍历不越 size* 数组界
  unsafe {
    if !(*f).source.is_null() {
      stringmark!((*f).source);
    }
    if !(*f).debugname.is_null() {
      stringmark!((*f).debugname);
    }
    // Safety:k/upvalues/p/locvars 均为 C 指针 + 计数，
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
