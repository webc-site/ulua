use core::ptr::from_ref;

use crate::{
  functions::{c_slice, validateobjref::validateobjref, validateref::validateref},
  macros::obj_2_gco::obj2gco,
  records::{gc_object::GCObject, global_state::global_State, luau_object::LuauObject},
};

/// # Safety
/// `g` 须指向存活 global_State；`inst` 须为存活 LuauObject，其 `members[0..numberofmembers]`
/// 与分配一致（校验仅读取引用字段与着色位）。
pub(crate) unsafe fn validateobject(g: *mut global_State, inst: &LuauObject) {
  unsafe {
    // 引用源身份：只读校验，从共享借用降 `*const` 即够用
    let obj: *const GCObject = from_ref(inst).cast();
    validateobjref(g, obj, obj2gco!(inst.lclass));
    let numberofmembers = inst.numberofmembers;
    let members = inst.members;
    // SAFETY:members 为 C 指针 + numberofmembers 计数，随对象分配。
    for member in c_slice(members, numberofmembers as usize) {
      validateref(g, obj, member);
    }
  }
}
