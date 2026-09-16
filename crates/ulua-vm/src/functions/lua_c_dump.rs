use core::{
  ffi::{c_char, c_int, c_void},
  ptr::null_mut,
};

use crate::{
  functions::{
    c_file_write, c_file_write_bytes, c_file_write_str, dumpgco::dumpgco, dumpref::dumpref,
    lua_m_visitgco::lua_m_visitgco,
  },
  macros::gcvalue::gcvalue,
  records::gc_object::GCObject,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_c_dump(
  l: *mut lua_State,
  file: *mut c_void,
  category_name: Option<unsafe extern "C-unwind" fn(*mut lua_State, u8) -> *const c_char>,
) {
  unsafe {
    let g = (*l).global;
    let f = file;

    c_file_write_bytes(f, b"{\"objects\":{\n");

    // The C++ code uses obj2gco(g->mainthread).
    // In Rust, obj2gco! expects a pointer to a type where (*p).tt exists or is accessible.
    // Since lua_State's first field is hdr (GCheader) which contains tt, we cast to *mut GCObject directly
    // to satisfy the macro's expectation of a collectable object pointer.
    let mainthread_gco = (*g).mainthread as *mut GCObject;
    dumpgco(f, null_mut(), mainthread_gco);

    lua_m_visitgco(l, f, dumpgco as *mut c_void);

    c_file_write_bytes(
      f,
      b"\"0\":{\"type\":\"userdata\",\"cat\":0,\"size\":0}\n},\"roots\":{\n\"mainthread\":",
    );
    dumpref(f, mainthread_gco);
    c_file_write_bytes(f, b",\"registry\":");
    dumpref(f, gcvalue!(&(*g).registry));

    c_file_write(
      f,
      format_args!("}},\"stats\":{{\n\"size\":{},\n", (*g).totalbytes as c_int),
    );

    c_file_write_bytes(f, b"\"categories\":{\n");
    for (i, &bytes) in (*g).memcatbytes.iter().enumerate() {
      if bytes != 0 {
        if let Some(cat_name_fn) = category_name {
          let name = cat_name_fn(l, i as u8);
          c_file_write(f, format_args!("\"{i}\":{{\"name\":\""));
          c_file_write_str(f, name);
          c_file_write(f, format_args!("\", \"size\":{}}},\n", bytes as c_int));
        } else {
          c_file_write(f, format_args!("\"{i}\":{{\"size\":{}}},\n", bytes as i32));
        }
      }
    }
    c_file_write_bytes(f, b"\"none\":{}\n}\n}}\n");
  }
}

pub use lua_c_dump as luaC_dump;
