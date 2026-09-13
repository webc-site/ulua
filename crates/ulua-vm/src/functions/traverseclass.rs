use crate::{
  functions::c_slice,
  macros::{markobject::markobject, markvalue::markvalue},
  records::{global_state::global_State, luau_class::LuauClass},
};

pub(crate) unsafe fn traverseclass(g: *mut global_State, classobject: *mut LuauClass) {
  unsafe {
    let classobject = &*classobject;

    markobject!(g, classobject.name);
    if !classobject.super_.is_null() {
      markobject!(g, classobject.super_);
    }
    markobject!(g, classobject.memberstooffset);

    // SAFETY：offsettomember/staticmembers 为 C 指针 + 计数，建类时一次分配。
    for &member in c_slice(
      classobject.offsettomember,
      classobject.numberofallmembers as usize,
    ) {
      markobject!(g, member);
    }

    let static_count = classobject.numberofallmembers - classobject.numberofinstancemembers;
    for member in c_slice(classobject.staticmembers, static_count as usize) {
      markvalue!(g, member);
    }

    if !classobject.instancemetatable.is_null() {
      markobject!(g, classobject.instancemetatable);
    }
  }
}
