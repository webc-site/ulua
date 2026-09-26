use core::{ffi::c_char, mem::size_of};

use ulua_common::fflag;

use crate::{
  functions::{
    c_slice,
    enumedge::{enum_edge, enum_member_edges},
    enumnode::enumnode,
    fmt_cstr_buf::{cstr_display, fmt_cstr_buf},
  },
  macros::{getstr::getstr, lua_idsize::LUA_IDSIZE},
  records::{enum_context::EnumContext, gc_object::GCObject, luau_class::LuauClass},
};

/// GC 枚举边名（NUL 结尾字节串，交给 [`enum_edge`] 取指针；§10 不用 `CStr`/`c"…"`）。
const EDGE_CLASS_NAME: &[u8] = b"classname\0";
const EDGE_SUPER: &[u8] = b"super\0";
const EDGE_CLASS_OFFSETS: &[u8] = b"classoffsets\0";
const EDGE_MEMBER_NAME: &[u8] = b"membername\0";
const EDGE_INSTANCE_METATABLE: &[u8] = b"instancemetatable\0";

/// # Safety
/// `ctx` 须指向存活 EnumContext（`node`/`edge`/`context` 回调可用），`lco` 须指向存活 LuauClass：`name` 字符串
/// 可读；`staticmembers` 覆盖 `numberofallmembers - numberofinstancemembers` 项、`offsettomember` 覆盖
/// `numberofallmembers` 项（界由这两个计数字段给出）；`super_`/`memberstooffset`/`instancemetatable` 按分支解引用。
/// 只读遍历并向 ctx 回调上报。cpp/VM/src/lgcdebug.cpp:1026 enumclass。
pub(crate) unsafe fn enumclass(ctx: *mut EnumContext, lco: *mut LuauClass) {
  unsafe {
    let lco_ref = &*lco;
    let mut buf = [0 as c_char; LUA_IDSIZE as usize];
    let obj = lco as *mut GCObject;

    let class_name = cstr_display(getstr(lco_ref.name));
    fmt_cstr_buf(&mut buf, format_args!("class object {class_name}"));

    enumnode(ctx, obj, size_of::<LuauClass>(), buf.as_ptr());
    enum_edge(ctx, obj, lco_ref.name, EDGE_CLASS_NAME);
    if !lco_ref.super_.is_null() {
      enum_edge(ctx, obj, lco_ref.super_, EDGE_SUPER);
    }

    enum_edge(ctx, obj, lco_ref.memberstooffset, EDGE_CLASS_OFFSETS);

    let numberofstaticmembers = lco_ref.numberofallmembers - lco_ref.numberofinstancemembers;
    // Safety:staticmembers / offsettomember 为 C 指针 + 计数，建类时一次分配。
    let staticmembers = c_slice(lco_ref.staticmembers, numberofstaticmembers as usize);
    let offsettomember = c_slice(lco_ref.offsettomember, lco_ref.numberofallmembers as usize);
    // 静态成员段是名字表的尾窗（实例段之后），切片基址即原 `i + numberofinstancemembers` 下标
    enum_member_edges(
      ctx,
      obj,
      staticmembers,
      &offsettomember[lco_ref.numberofinstancemembers as usize..],
    );

    for &member in offsettomember {
      enum_edge(ctx, obj, member, EDGE_MEMBER_NAME);
    }

    if fflag::LuauEnumMoreEdges.get() && !lco_ref.instancemetatable.is_null() {
      enum_edge(ctx, obj, lco_ref.instancemetatable, EDGE_INSTANCE_METATABLE);
    }
  }
}
