use core::{
  ffi::{c_int, c_void},
  mem::size_of,
  ptr::eq,
  slice,
};

use crate::{
  functions::{c_file_write, c_file_write_bytes, dumpref::dumpref, dumprefs::dumprefs},
  macros::{
    dummynode::dummynode, gcvalue::gcvalue, iscollectable::iscollectable, obj_2_gco::obj2gco,
    sizenode::sizenode, ttisnil::ttisnil,
  },
  records::{lua_node::LuaNode, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn dumptable(f: *mut c_void, h: *mut LuaTable) {
  unsafe {
    let h_ref = &*h;

    let size = size_of::<LuaTable>()
      + if eq(h_ref.node, dummynode) {
        0
      } else {
        sizenode!(h) as usize * size_of::<LuaNode>()
      }
      + h_ref.sizearray as usize * size_of::<TValue>();

    c_file_write(
      f,
      format_args!(
        "{{\"type\":\"table\",\"cat\":{},\"size\":{}",
        h_ref.memcat, size as c_int
      ),
    );

    if !eq(h_ref.node, dummynode) {
      c_file_write_bytes(f, b",\"pairs\":[");

      let mut first = true;

      for n in slice::from_raw_parts(h_ref.node, sizenode!(h) as usize) {
        if !ttisnil!(&n.val) && (iscollectable!(&n.key) || iscollectable!(&n.val)) {
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

    if h_ref.sizearray != 0 {
      c_file_write_bytes(f, b",\"array\":[");
      dumprefs(f, h_ref.array, h_ref.sizearray as usize);
      c_file_write_bytes(f, b"]");
    }

    if !h_ref.metatable.is_null() {
      c_file_write_bytes(f, b",\"metatable\":");
      dumpref(f, obj2gco!(h_ref.metatable));
    }

    c_file_write_bytes(f, b"}");
  }
}
