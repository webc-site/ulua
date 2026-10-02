//! Differential-oracle driver: load precompiled Luau bytecode (produced by
//! the C++ `luau-compile --binary`) and run it on the Rust VM. Errors —
//! including `todo!()` panics from not-yet-ported functions — surface as Lua
//! errors via the catch_unwind in lua_d_rawrunprotected, so every run either
//! prints results or names the next function to port.
//!
//! Mirrors the real `luau` CLI: the chunk is loaded into a fresh thread and
//! run with `lua_resume` (not `lua_pcall` on the main thread), so top-level
//! `coroutine.running()`/`isyieldable()` behave as in the reference CLI.

use std::{env::args, fs::File, io::Read, panic::set_hook, process::exit};

use ulua_common::records::f_value::set_luau_bool_flags;
#[cfg(feature = "vm-opcount")]
use ulua_vm::functions::op_count;
use ulua_vm::functions::type_feedback;
use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_newthread::lua_newthread,
    lua_tolstring::lua_tolstring_ref, lua_tonumberx::lua_tonumberx, luau_load::luau_load,
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向存活的 `LuaState`，`idx` 须为其合法栈索引；tolstring 可能把数字转为字符串
/// 并写回该栈槽（mirrors the C API）。
/// Copies the value at `idx` into a lossy String; `None` when it is not a
/// string.
unsafe fn tolstring_lossy(l: *mut LuaState, idx: i32) -> Option<String> {
  // SAFETY: 契约保证 l 存活、idx 合法，返回切片指向该栈槽串内容（借用即弃）
  unsafe { lua_tolstring_ref(l, idx) }.map(|s| String::from_utf8_lossy(s).into_owned())
}

fn main() {
  // 差分 oracle 驱动要求 stderr 只含结果：静音默认钩子。std 的 set_hook 签名
  // 强制 `Box<dyn Fn>`（review.md §4 保留：进程级一次性冷路径，泛型无从替代）。
  set_hook(Box::new(|_| {}));

  let Some(path) = args().nth(1) else {
    eprintln!("usage: luau_run <bytecode-file>");
    exit(1);
  };
  let mut bc = Vec::new();
  let mut file = match File::open(&path) {
    Ok(f) => f,
    Err(e) => {
      eprintln!("cannot open bytecode file: {e}");
      exit(1);
    }
  };
  if let Err(e) = file.read_to_end(&mut bc) {
    eprintln!("cannot read bytecode file: {e}");
    exit(1);
  }

  // mirror the C++ CLI: setLuauFlagsDefault(true) — v11+ bytecode needs it
  set_luau_bool_flags(true);
  type_feedback::init_from_env();

  // SAFETY: 块内指针均出自本作用域 newstate/newthread 且已判非空，直到块尾单线程独占使用
  unsafe {
    let l = lua_l_newstate();
    if l.is_null() {
      eprintln!("lua_l_newstate returned null");
      exit(1);
    }
    lua_l_openlibs(l);

    // Run the chunk on a fresh thread, like CLI/src/Repl.cpp's runCode: the
    // thread T is rooted on l's stack, and we load the function directly into
    // T (the global string table / GC is shared), then resume it.
    let t = lua_newthread(l);
    if t.is_null() {
      eprintln!("lua_newthread returned null");
      exit(1);
    }

    let rc = luau_load(t, "=script", &bc, 0);
    if rc != 0 {
      eprintln!("luau_load failed: rc={rc}");
      exit(2);
    }

    let status = (*t).resume_main(0);
    if status != 0 {
      // The error object is on top of T's stack — surface its text so the
      // differential oracle reports WHY a run failed, not just the status.
      let msg = tolstring_lossy(t, -1).unwrap_or_else(|| "<non-string error>".to_string());
      eprintln!("pcall status={status}: {msg}");
      exit(3);
    }

    let n = (*t).get_top();
    println!("results: {n}");
    for i in 1..=n {
      print_result(t, i);
    }

    // vm-opcount：把本趟运行的动态 opcode 直方图 + 转移表打到 stdout。融合站点
    // （luau_execute.rs 的 fuse_succ_*）与后继相邻性都据此判读，不靠猜。
    #[cfg(feature = "vm-opcount")]
    print!("{}", op_count::dump());

    // J1 Phase 1a：类型观测读数（ULUA_TYPE_FEEDBACK=1 时激活）
    if type_feedback::enabled() {
      print!("{}", type_feedback::dump());
      print!("{}", type_feedback::tsfb_dump());
    }
  }
}

/// # Safety
///
/// `t` 必须指向存活的 `LuaState`（新线程），`i` 须为 1..=gettop 的合法栈索引。
/// Prints the value at stack index `idx`：字符串按原样输出，
/// 数字经 tonumber 转换输出，其余报 <non-number>。
unsafe fn print_result(t: *mut LuaState, i: i32) {
  // SAFETY: 契约保证 t 存活、i 合法，块内 lua_type/tonumberx/tolstring 只读写该栈槽
  unsafe {
    if (*t).type_of(i) == LuaType::String {
      println!("  [{i}] = {:?}", tolstring_lossy(t, i).unwrap_or_default());
      return;
    }
    if let Some(v) = lua_tonumberx(&*t, i) {
      println!("  [{i}] = {v}");
    } else {
      println!("  [{i}] = {:?}", tolstring_lossy(t, i).unwrap_or_default());
    }
  }
}
