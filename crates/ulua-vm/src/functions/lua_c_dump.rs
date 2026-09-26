use core::{
  ffi::{c_char, c_void},
  ptr::null_mut,
};

use crate::{
  functions::{
    c_file_write, c_file_write_bytes, c_file_write_str, dumpgco::dumpgco, dumpref::dumpref,
    lua_m_visitgco::lua_m_visitgco,
  },
  macros::gcvalue::gcvalue,
  records::{gc_object::GCObject, lua_state::LuaState},
};

/// # Safety
/// `l` 须指向存活 `LuaState`，`file` 须为可写输出目标（cpp lgcdebug.cpp:708）：遍历时 heap
/// 页与各 GCObject 须保持存活且未被回收（须停顿在安全点调用）；可选 `category_name` 回调须为
/// 合法 `unsafe extern "C-unwind"` 函数。
pub unsafe fn lua_c_dump(
  l: *mut LuaState,
  file: *mut c_void,
  category_name: Option<unsafe extern "C-unwind" fn(*mut LuaState, u8) -> *const c_char>,
) {
  // Safety: 契约保证 `l`/`f` 为存活状态与可写输出目标，逐页枚举 heap 期间对象保持存活
  unsafe {
    let g = (*l).global;
    let f = file;

    c_file_write_bytes(f, b"{\"objects\":{\n");

    // cpp lgc.cpp luaC_dump：mainthread 的 obj2gco 提升（同 enumheap，
    // 移植记录无 GCObject 方法入口，直达其 hdr 头字段）
    let mainthread_gco = (*g).mainthread as *mut GCObject;
    dumpgco(f, null_mut(), mainthread_gco);

    lua_m_visitgco(l, f, dumpgco);

    c_file_write_bytes(
      f,
      b"\"0\":{\"type\":\"userdata\",\"cat\":0,\"size\":0}\n},\"roots\":{\n\"mainthread\":",
    );
    dumpref(f, mainthread_gco);
    c_file_write_bytes(f, b",\"registry\":");
    dumpref(f, gcvalue!(&(*g).registry));

    c_file_write(
      f,
      format_args!("}},\"stats\":{{\n\"size\":{},\n", (*g).totalbytes as i32),
    );

    c_file_write_bytes(f, b"\"categories\":{\n");
    for (i, &bytes) in (*g).memcatbytes.iter().enumerate() {
      if bytes != 0 {
        if let Some(cat_name_fn) = category_name {
          let name = cat_name_fn(l, i as u8);
          c_file_write(f, format_args!("\"{i}\":{{\"name\":\""));
          c_file_write_str(f, name);
          c_file_write(f, format_args!("\", \"size\":{}}},\n", bytes as i32));
        } else {
          c_file_write(f, format_args!("\"{i}\":{{\"size\":{}}},\n", bytes as i32));
        }
      }
    }
    c_file_write_bytes(f, b"\"none\":{}\n}\n}}\n");
  }
}
