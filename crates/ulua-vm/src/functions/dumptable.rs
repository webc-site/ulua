use core::{ffi::c_void, mem::size_of, ptr::eq, slice};

use crate::{
  functions::{c_file_write_bytes, dump_json_head, dumpref::dumpref, dumprefs::dumprefs},
  macros::{
    dummynode::dummynode, gcvalue::gcvalue, iscollectable::iscollectable, obj_2_gco::obj2gco,
    sizenode::sizenode,
  },
  records::{lua_node::LuaNode, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn dumptable(f: *mut c_void, h: &LuaTable) {
  unsafe {
    let size = size_of::<LuaTable>()
      + if eq(h.node, dummynode) {
        0
      } else {
        sizenode!(h) as usize * size_of::<LuaNode>()
      }
      + h.sizearray as usize * size_of::<TValue>();

    dump_json_head(f, "table", h.memcat, size as i32);

    if !eq(h.node, dummynode) {
      c_file_write_bytes(f, b",\"pairs\":[");

      let mut first = true;

      for n in slice::from_raw_parts(h.node, sizenode!(h) as usize) {
        if !n.val.is_nil() && (iscollectable!(&n.key) || iscollectable!(&n.val)) {
          if !first {
            c_file_write_bytes(f, b",");
          }
          first = false;

          if iscollectable!(&n.key) {
            dumpref(f, gcvalue!(&n.key));
          } else {
            c_file_write_bytes(f, b"null");
          }

          c_file_write_bytes(f, b",");

          if iscollectable!(&n.val) {
            dumpref(f, gcvalue!(&n.val));
          } else {
            c_file_write_bytes(f, b"null");
          }
        }
      }

      c_file_write_bytes(f, b"]");
    }

    if h.sizearray != 0 {
      c_file_write_bytes(f, b",\"array\":[");
      dumprefs(f, h.array, h.sizearray as usize);
      c_file_write_bytes(f, b"]");
    }

    if !h.metatable.is_null() {
      c_file_write_bytes(f, b",\"metatable\":");
      dumpref(f, obj2gco!(h.metatable));
    }

    c_file_write_bytes(f, b"}");
  }
}
