use core::{ffi::c_char, mem::size_of};

use crate::{
  functions::{
    c_slice,
    enumedge::enumedge,
    enumnode::enumnode,
    fmt_cstr_buf::{cstr_display, fmt_cstr_buf},
  },
  macros::{
    gcvalue::gcvalue, getstr::getstr, iscollectable::iscollectable, lua_idsize::LUA_IDSIZE,
  },
  records::{enum_context::EnumContext, gc_object::GCObject, luau_object::LuauObject},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn enumobject(ctx: *mut EnumContext, inst: *mut LuauObject) {
  unsafe {
    let inst_ref = &*inst;
    let mut buf = [0 as c_char; LUA_IDSIZE as usize];

    let obj = inst as *mut GCObject;

    let class_name = cstr_display(getstr((*inst_ref.lclass).name));
    fmt_cstr_buf(&mut buf, format_args!("object {class_name}"));

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
      let name = cstr_display(getstr(offsettomember[i]));
      fmt_cstr_buf(&mut membername, format_args!("{name}"));

      enumedge(ctx, obj, gcvalue!(val), membername.as_ptr());
    }
  }
}
