use crate::{
  functions::{c_slice, validateobjref::validateobjref, validateref::validateref},
  macros::obj_2_gco::obj2gco,
  records::{gc_object::GCObject, global_state::global_State, luau_class::LuauClass},
};

pub(crate) unsafe fn validateclass(g: *mut global_State, lco: *mut LuauClass) {
  unsafe {
    let obj = obj2gco!(lco);
    validateobjref(g, obj, obj2gco!((*lco).name));
    if !(*lco).super_.is_null() {
      validateobjref(g, obj, obj2gco!((*lco).super_ as *mut GCObject));
    }
    validateobjref(g, obj, obj2gco!((*lco).memberstooffset));

    // SAFETY：offsettomember/staticmembers 为 C 指针 + 计数，建类时一次分配。
    let offsettomember = c_slice((*lco).offsettomember, (*lco).numberofallmembers as usize);
    let staticmembers = c_slice(
      (*lco).staticmembers,
      ((*lco).numberofallmembers - (*lco).numberofinstancemembers) as usize,
    );
    for (i, &member) in offsettomember.iter().enumerate() {
      validateobjref(g, obj, obj2gco!(member));
      if i >= (*lco).numberofinstancemembers as usize {
        validateref(
          g,
          obj,
          &staticmembers[i - (*lco).numberofinstancemembers as usize],
        );
      }
    }

    if !(*lco).instancemetatable.is_null() {
      validateobjref(g, obj, obj2gco!((*lco).instancemetatable));
    }
  }
}
