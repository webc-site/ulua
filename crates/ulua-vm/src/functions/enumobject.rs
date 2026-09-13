use core::{
  ffi::{c_char, c_int},
  mem::size_of,
};

use crate::{
  functions::{c_slice, enumedge::enumedge, enumnode::enumnode},
  macros::{
    gcvalue::gcvalue, getstr::getstr, iscollectable::iscollectable, lua_idsize::LUA_IDSIZE,
  },
  records::{enum_context::EnumContext, gc_object::GCObject, luau_object::LuauObject},
};

pub(crate) unsafe fn enumobject(ctx: *mut EnumContext, inst: *mut LuauObject) {
  unsafe {
    let inst_ref = &*inst;
    let mut buf = [0 as c_char; LUA_IDSIZE as usize];

    let obj = inst as *mut GCObject;

    unsafe extern "C" {
      fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    }

    snprintf(
      buf.as_mut_ptr(),
      buf.len(),
      c"object %s".as_ptr() as *const c_char,
      getstr((*inst_ref.lclass).name),
    );

    enumnode(ctx, obj, size_of::<LuauObject>(), buf.as_ptr());

    // SAFETY：members / offsettomember 为 C 指针 + 计数，建类时一次分配。
    let members = c_slice(
      inst_ref.members,
      (*inst_ref.lclass).numberofinstancemembers as usize,
    );
    let offsettomember = c_slice(
      (*inst_ref.lclass).offsettomember,
      (*inst_ref.lclass).numberofinstancemembers as usize,
    );
    for (i, val) in members.iter().enumerate() {
      if !iscollectable!(val) {
        continue;
      }

      let mut membername = [0 as c_char; 32];
      let name_ptr = offsettomember[i];
      snprintf(
        membername.as_mut_ptr(),
        membername.len(),
        c"%s".as_ptr() as *const c_char,
        getstr(name_ptr),
      );

      enumedge(ctx, obj, gcvalue!(val), membername.as_ptr());
    }
  }
}
