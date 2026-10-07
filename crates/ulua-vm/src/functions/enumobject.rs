use core::{ffi::c_char, mem::size_of};

use crate::{
  functions::{
    c_slice,
    enumedge::enum_member_edges,
    enumnode::enumnode,
    fmt_cstr_buf::{cstr_display, fmt_cstr_buf},
  },
  macros::{getstr::getstr, lua_idsize::LUA_IDSIZE},
  records::{enum_context::EnumContext, gc_object::GCObject, luau_object::LuauObject},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn enumobject(ctx: *mut EnumContext, inst: &LuauObject) {
  unsafe {
    let mut buf = [0u8; LUA_IDSIZE as usize];
    // 枚举身份取对象首字节地址：`obj2gco!` 要求裸指针入参，共享句柄降 `*const` 即够用
    let obj = (inst as *const LuauObject).cast::<GCObject>();

    let class_name = cstr_display(getstr((*inst.lclass).name));
    fmt_cstr_buf(&mut buf, format_args!("object {class_name}"));

    enumnode(
      ctx,
      obj,
      size_of::<LuauObject>(),
      buf.as_ptr().cast::<c_char>(),
    );

    // SAFETY:members / offsettomember 为 C 指针 + 计数，建类时一次分配。
    let members = c_slice(
      inst.members,
      (*inst.lclass).numberofinstancemembers as usize,
    );
    let offsettomember = c_slice(
      (*inst.lclass).offsettomember,
      (*inst.lclass).numberofinstancemembers as usize,
    );
    enum_member_edges(ctx, obj, members, offsettomember);
  }
}
