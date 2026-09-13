//! Differential-oracle driver: load precompiled Luau bytecode (produced by
//! the C++ `luau-compile --binary`) and run it on the Rust VM. Errors —
//! including `todo!()` panics from not-yet-ported functions — surface as Lua
//! errors via the catch_unwind in luaD_rawrunprotected, so every run either
//! prints results or names the next function to port.
//!
//! Mirrors the real `luau` CLI: the chunk is loaded into a fresh thread and
//! run with `lua_resume` (not `lua_pcall` on the main thread), so top-level
//! `coroutine.running()`/`isyieldable()` behave as in the reference CLI.

use std::{
  env::args, ffi::c_char, fs::File, io::Read, panic::set_hook, process::exit, ptr::null_mut,
  slice::from_raw_parts,
};

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_gettop::lua_gettop, lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs,
    lua_newthread::lua_newthread, lua_resume::lua_resume, lua_tolstring::lua_tolstring,
    lua_tonumberx::lua_tonumberx, lua_type::lua_type, luau_load::luau_load,
  },
  records::lua_state::lua_State,
};

/// Copies the value at `idx` into a lossy String; `None` when it is not a
/// string (tolstring may also convert numbers, mirroring the C API).
unsafe fn tolstring_lossy(l: *mut lua_State, idx: i32) -> Option<String> {
  unsafe {
    let mut len = 0usize;
    let s = lua_tolstring(l, idx, &mut len);
    if s.is_null() {
      None
    } else {
      Some(String::from_utf8_lossy(from_raw_parts(s as *const u8, len)).into_owned())
    }
  }
}

fn main() {
  set_hook(Box::new(|_| {}));

  let path = args().nth(1).expect("usage: luau_run <bytecode-file>");
  let mut bc = Vec::new();
  File::open(&path)
    .expect("cannot open bytecode file")
    .read_to_end(&mut bc)
    .expect("cannot read bytecode file");

  // mirror the C++ CLI: setLuauFlagsDefault(true) — v11+ bytecode needs it
  ulua_common::set_all_flags(true);

  unsafe {
    let l = lua_l_newstate();
    assert!(!l.is_null(), "lua_l_newstate returned null");
    lua_l_openlibs(l);

    // Run the chunk on a fresh thread, like CLI/src/Repl.cpp's runCode: the
    // thread T is rooted on l's stack, and we load the function directly into
    // T (the global string table / GC is shared), then resume it.
    let t = lua_newthread(l);
    assert!(!t.is_null(), "lua_newthread returned null");

    let rc = luau_load(
      t,
      c"=script".as_ptr(),
      bc.as_ptr() as *const c_char,
      bc.len(),
      0,
    );
    if rc != 0 {
      eprintln!("luau_load failed: rc={rc}");
      exit(2);
    }

    let status = lua_resume(t, null_mut(), 0);
    if status != 0 {
      // The error object is on top of T's stack — surface its text so the
      // differential oracle reports WHY a run failed, not just the status.
      let msg = tolstring_lossy(t, -1).unwrap_or_else(|| "<non-string error>".to_string());
      eprintln!("pcall status={status}: {msg}");
      exit(3);
    }

    let n = lua_gettop(t);
    println!("results: {n}");
    for i in 1..=n {
      if lua_type(t, i) == LuaType::String as i32 {
        match tolstring_lossy(t, i) {
          Some(text) => println!("  [{i}] = {:?}", text.as_str()),
          None => println!("  [{i}] = <non-number>"),
        }
      } else {
        let mut isnum: i32 = 0;
        let v = lua_tonumberx(t, i, &mut isnum);
        if isnum != 0 {
          println!("  [{i}] = {v}");
        } else {
          match tolstring_lossy(t, i) {
            Some(text) => println!("  [{i}] = {:?}", text.as_str()),
            None => println!("  [{i}] = <non-number>"),
          }
        }
      }
    }
  }
}
