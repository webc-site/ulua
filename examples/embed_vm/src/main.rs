//! Embed the VM directly: compile, load onto a fresh thread, run, and read the
//! returned value. Use this lower-level (C-style) API when you need more control
//! than `ulua::eval` — custom globals, inspecting results, multiple chunks, etc.
//!
//!     cargo run -p ulua-example-embed-vm

use core::{
  ffi::{c_char, c_int},
  ptr::null_mut,
};

use ulua::{
  common::set_all_flags,
  vm::functions::{
    lua_gettop::lua_gettop, lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs,
    lua_newthread::lua_newthread, lua_resume::lua_resume, lua_tonumberx::lua_tonumberx,
    luau_load::luau_load,
  },
};
fn main() {
  let bytecode = ulua::compile("return 6 * 7").expect("compile failed");

  // v11+ bytecode needs the default Luau flags (matches the CLI).
  set_all_flags(true);

  unsafe {
    let l = lua_l_newstate();
    assert!(!l.is_null(), "could not create Lua state");
    lua_l_openlibs(l);

    // Run on a fresh thread, like the reference CLI's runCode.
    let t = lua_newthread(l);
    assert!(!t.is_null(), "could not create thread");

    let rc = luau_load(
      t,
      c"=embed".as_ptr(),
      bytecode.as_ptr() as *const c_char,
      bytecode.len(),
      0,
    );
    assert_eq!(rc, 0, "luau_load failed (rc={rc})");

    let status = lua_resume(t, null_mut(), 0);
    assert_eq!(status, 0, "script raised an error (status={status})");

    // The chunk's return values are left on the thread's stack.
    let n = lua_gettop(t);
    println!("script returned {n} value(s):");
    for i in 1..=n {
      let mut is_num: c_int = 0;
      let v = lua_tonumberx(t, i, &mut is_num);
      if is_num != 0 {
        println!("  [{i}] = {v}");
      }
    }
  }
}
