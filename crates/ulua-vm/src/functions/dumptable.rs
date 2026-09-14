use core::{
  ffi::{c_char, c_int, c_void},
  mem::size_of,
  ptr::eq,
  slice,
};

use crate::{
  functions::{dumpref::dumpref, dumprefs::dumprefs},
  macros::{
    dummynode::dummynode, gcvalue::gcvalue, iscollectable::iscollectable, obj_2_gco::obj2gco,
    sizenode::sizenode, ttisnil::ttisnil,
  },
  records::{lua_node::LuaNode, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

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

    unsafe extern "C" {
      fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    }

    unsafe extern "C" {
      fn fputc(c: c_int, stream: *mut c_void) -> c_int;
    }

    fprintf(
      f,
      c"{\"type\":\"table\",\"cat\":%d,\"size\":%d".as_ptr() as *const c_char,
      h_ref.memcat as c_int,
      size as c_int,
    );

    if !eq(h_ref.node, dummynode) {
      fprintf(f, c",\"pairs\":[".as_ptr() as *const c_char);

      let mut first = true;

      for n in slice::from_raw_parts(h_ref.node, sizenode!(h) as usize) {
        if !ttisnil!(&n.val) && (iscollectable!(&n.key) || iscollectable!(&n.val)) {
          if !first {
            fputc(',' as c_int, f);
          }
          first = false;

          if iscollectable!(&n.key) {
            dumpref(f, gcvalue!(&n.key));
          } else {
            fprintf(f, c"null".as_ptr() as *const c_char);
          }

          fputc(',' as c_int, f);

          if iscollectable!(&n.val) {
            dumpref(f, gcvalue!(&n.val));
          } else {
            fprintf(f, c"null".as_ptr() as *const c_char);
          }
        }
      }

      fprintf(f, c"]".as_ptr() as *const c_char);
    }

    if h_ref.sizearray != 0 {
      fprintf(f, c",\"array\":[".as_ptr() as *const c_char);
      dumprefs(f, h_ref.array, h_ref.sizearray as usize);
      fprintf(f, c"]".as_ptr() as *const c_char);
    }

    if !h_ref.metatable.is_null() {
      fprintf(f, c",\"metatable\":".as_ptr() as *const c_char);
      dumpref(f, obj2gco!(h_ref.metatable));
    }

    fprintf(f, c"}".as_ptr() as *const c_char);
  }
}
