use crate::{
  functions::{c_slice, validateobjref::validateobjref, validateref::validateref},
  macros::obj_2_gco::obj2gco,
  records::{global_state::global_State, luau_object::LuauObject},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn validateobject(g: *mut global_State, inst: *mut LuauObject) {
  unsafe {
    let obj = obj2gco!(inst);
    validateobjref(g, obj, obj2gco!((*inst).lclass));
    let numberofmembers = (*inst).numberofmembers;
    let members = (*inst).members;
    // SAFETY：members 为 C 指针 + numberofmembers 计数，随对象分配。
    for member in c_slice(members, numberofmembers as usize) {
      validateref(g, obj, member);
    }
  }
}
