use core::{
  ffi::c_int,
  sync::atomic::{AtomicBool, Ordering},
};

use ulua_vm::{
  functions::{lua_break::lua_break, lua_isyieldable::lua_isyieldable},
  records::lua_state::LuaState,
};

/// cpp 的 `static bool skipBreak`；每次中断在 break/skip 之间交替，并行测试下
/// `static mut` 即数据竞争，故用 `AtomicBool`。
static SKIP_BREAK: AtomicBool = AtomicBool::new(false);

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_interrupt_inspection_interrupt(
  l: *mut LuaState,
  gc: c_int,
) {
  // GC 阶段放行是纯整数判断；不可 yield 的上下文同样直接返回。
  if gc >= 0 {
    return;
  }

  // Safety: `l` 为本用例存活的 LuaState；只探测当前上下文可否 yield（不改动栈）。
  if unsafe { lua_isyieldable(l) } == 0 {
    return;
  }

  // 每次中断在 break/skip 之间交替：原子量读写是 safe Rust。
  let skip = SKIP_BREAK.load(Ordering::Relaxed);
  if !skip {
    // Safety: `l` 存活；按用例开关请求在下一个安全点打断执行。
    unsafe { lua_break(l) };
  }

  SKIP_BREAK.store(!skip, Ordering::Relaxed);
}
