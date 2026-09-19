//! 直接嵌入 VM：编译、加载到新线程、运行、读取返回值。当需要比 `ulua::eval`
//! 更多的控制（自定义全局、检查结果、多个 chunk 等）时，用这个低层（C 风格）
//! API。
//!
//!     cargo run -p ulua-example-embed-vm

use core::{
  ffi::{c_char, c_int},
  ptr::null_mut,
};

use ulua::{
  common::set_all_flags,
  vm::functions::{
    lua_close::lua_close, lua_gettop::lua_gettop, lua_l_newstate::lua_l_newstate,
    lua_l_openlibs::lua_l_openlibs, lua_newthread::lua_newthread, lua_resume::lua_resume,
    lua_tonumberx::lua_tonumberx, luau_load::luau_load,
  },
};

fn main() {
  let bytecode = ulua::compile("return 6 * 7").expect("compile failed");

  // v11+ 字节码需要默认 Luau flags（与 CLI 一致）。
  // SAFETY: 进程启动期写入旗标，此后只读（同 C++ 全局初始化契约）
  unsafe { set_all_flags(true) };

  unsafe {
    let l = lua_l_newstate();
    assert!(!l.is_null(), "could not create Lua state");
    lua_l_openlibs(l);

    // 在新线程上运行，对齐参考 CLI 的 runCode。
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

    // chunk 的返回值留在线程栈上。
    let n = lua_gettop(t);
    println!("script returned {n} value(s):");
    for i in 1..=n {
      let mut is_num: c_int = 0;
      let v = lua_tonumberx(t, i, &mut is_num);
      if is_num != 0 {
        println!("  [{i}] = {v}");
      }
    }
    lua_close(l);
  }
}
