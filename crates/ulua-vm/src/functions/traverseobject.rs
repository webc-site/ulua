use crate::{
  functions::c_slice,
  macros::{markobject::markobject, markvalue::markvalue},
  records::{global_state::global_State, luau_object::LuauObject},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn traverseobject(g: *mut global_State, classinst: *mut LuauObject) {
  unsafe {
    // markobject(g, classinst->lclass);
    markobject!(g, (*classinst).lclass);

    // for (int i = 0; i < classinst->numberofmembers; i++)
    //     markvalue(g, &classinst->members[i]);
    let numberofmembers = (*classinst).numberofmembers;
    let members = (*classinst).members;
    for member in c_slice(members, numberofmembers as usize) {
      markvalue!(g, member);
    }
  }
}
