use core::{
  ffi::{c_char, c_int, c_void},
  ptr::null_mut,
};

use crate::{
  functions::{dumpgco::dumpgco, dumpref::dumpref, lua_m_visitgco::lua_m_visitgco},
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

    unsafe extern "C" {
      fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    }

    fprintf(f, c"{\"objects\":{\n".as_ptr() as *const c_char);

    // The C++ code uses obj2gco(g->mainthread).
    // In Rust, obj2gco! expects a pointer to a type where (*p).tt exists or is accessible.
    // Since lua_State's first field is hdr (GCheader) which contains tt, we cast to *mut GCObject directly
    // to satisfy the macro's expectation of a collectable object pointer.
    let mainthread_gco = (*g).mainthread as *mut GCObject;
    dumpgco(f, null_mut(), mainthread_gco);

    lua_m_visitgco(l, f, dumpgco as *mut c_void);

    fprintf(
      f,
      c"\"0\":{\"type\":\"userdata\",\"cat\":0,\"size\":0}\n},\"roots\":{\n\"mainthread\":".as_ptr()
        as *const c_char,
    );
    dumpref(f, mainthread_gco);
    fprintf(f, c",\"registry\":".as_ptr() as *const c_char);
    dumpref(f, gcvalue!(&(*g).registry));

    fprintf(
      f,
      c"},\"stats\":{\n\"size\":%d,\n".as_ptr() as *const c_char,
      (*g).totalbytes as c_int,
    );

    fprintf(f, c"\"categories\":{\n".as_ptr() as *const c_char);
    for (i, &bytes) in (*g).memcatbytes.iter().enumerate() {
      if bytes != 0 {
        if let Some(cat_name_fn) = category_name {
          let name = cat_name_fn(l, i as u8);
          fprintf(
            f,
            c"\"%d\":{\"name\":\"%s\", \"size\":%d},\n".as_ptr() as *const c_char,
            i,
            name,
            bytes as c_int,
          );
        } else {
          fprintf(
            f,
            c"\"%d\":{\"size\":%d},\n".as_ptr() as *const c_char,
            i,
            bytes as c_int,
          );
        }
      }
    }
    fprintf(f, c"\"none\":{}\n}\n}}\n".as_ptr() as *const c_char);
  }
}

pub use lua_c_dump as luaC_dump;
