use core::{
  ffi::{c_char, c_int},
  mem::size_of,
};

use ulua_common::FFlag;

use crate::{
  functions::{c_slice, enumedge::enumedge, enumnode::enumnode},
  macros::{
    gcvalue::gcvalue, getstr::getstr, iscollectable::iscollectable, lua_idsize::LUA_IDSIZE,
  },
  records::{enum_context::EnumContext, gc_object::GCObject, luau_class::LuauClass},
};

pub(crate) unsafe fn enumclass(ctx: *mut EnumContext, lco: *mut LuauClass) {
  unsafe {
    let lco_ref = &*lco;
    let mut buf = [0 as c_char; LUA_IDSIZE as usize];
    let obj = lco as *mut GCObject;

    unsafe extern "C" {
      fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    }

    snprintf(
      buf.as_mut_ptr(),
      buf.len(),
      c"class object %s".as_ptr() as *const c_char,
      getstr(lco_ref.name),
    );

    enumnode(ctx, obj, size_of::<LuauClass>(), buf.as_ptr());
    enumedge(
      ctx,
      obj,
      lco_ref.name as *mut GCObject,
      c"classname".as_ptr() as *const c_char,
    );
    if !lco_ref.super_.is_null() {
      enumedge(
        ctx,
        obj,
        lco_ref.super_ as *mut GCObject,
        c"super".as_ptr() as *const c_char,
      );
    }

    enumedge(
      ctx,
      obj,
      lco_ref.memberstooffset as *mut GCObject,
      c"classoffsets".as_ptr() as *const c_char,
    );

    let numberofstaticmembers = lco_ref.numberofallmembers - lco_ref.numberofinstancemembers;
    // SAFETY：staticmembers / offsettomember 为 C 指针 + 计数，建类时一次分配。
    let staticmembers = c_slice(lco_ref.staticmembers, numberofstaticmembers as usize);
    let offsettomember = c_slice(lco_ref.offsettomember, lco_ref.numberofallmembers as usize);
    for (i, val) in staticmembers.iter().enumerate() {
      if !iscollectable!(val) {
        continue;
      }

      let mut membername = [0 as c_char; 32];
      let name_ptr = offsettomember[i + lco_ref.numberofinstancemembers as usize];
      snprintf(
        membername.as_mut_ptr(),
        membername.len(),
        c"%s".as_ptr() as *const c_char,
        getstr(name_ptr),
      );
      enumedge(ctx, obj, gcvalue!(val), membername.as_ptr());
    }

    for &member in offsettomember {
      enumedge(
        ctx,
        obj,
        member as *mut GCObject,
        c"membername".as_ptr() as *const c_char,
      );
    }

    if FFlag::LuauEnumMoreEdges.get() && !lco_ref.instancemetatable.is_null() {
      enumedge(
        ctx,
        obj,
        lco_ref.instancemetatable as *mut GCObject,
        c"instancemetatable".as_ptr() as *const c_char,
      );
    }
  }
}
