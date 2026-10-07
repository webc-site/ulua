use core::{ffi::c_int, sync::atomic::Ordering};

use ulua_vm::{
  luaL_error,
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

use crate::common::{
  functions::safe_api::{getinfo, yield_, zero_debug},
  records::conformance_interrupt_state::{
    CONFORMANCE_INTERRUPT_MODE_EXPECTED_HITS, CONFORMANCE_INTERRUPT_MODE_HANG,
    CONFORMANCE_INTERRUPT_MODE_HANG_PCALL, CONFORMANCE_INTERRUPT_MODE_INFLOOP,
    CONFORMANCE_INTERRUPT_STATE,
  },
};

/// 读取 level 0 栈帧的当前行号（cpp `lua_getinfo(L, "l", &ar)` 后取 `ar.currentline`）。
fn current_line(l: *mut LuaState) -> c_int {
  let mut ar: LuaDebug = zero_debug();
  getinfo(l, 0, b"l", &mut ar);
  ar.currentline
}

/// EXPECTED_HITS 模式：逐次核对中断命中行号，第 4 次（index+1==4）让出。
fn interrupt_expected_hits(l: *mut LuaState) {
  static EXPECTED_HITS: [i32; 22] = [
    11, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 8, 20, 15, 15, 15, 15, 18, 25, 23, 26,
  ];

  let index = CONFORMANCE_INTERRUPT_STATE
    .index
    .fetch_add(1, Ordering::SeqCst);
  assert!(
    (index as usize) < EXPECTED_HITS.len(),
    "interrupt index {index} exceeded expected hit count"
  );

  let line = current_line(l);
  assert_eq!(line, EXPECTED_HITS[index as usize]);

  if index + 1 == 4 {
    // yield 仅在活跃 resume 上下文中调用（本 hook 的调用序契约）。
    yield_(l, 0);
  }
}

/// INFLOOP 模式：累计 11 次中断后让出，越限即断言失败。
fn interrupt_infloop(l: *mut LuaState) {
  let index = CONFORMANCE_INTERRUPT_STATE
    .index
    .fetch_add(1, Ordering::SeqCst)
    + 1;
  assert!(index <= 11, "interrupt index {index} exceeded infloop cap");

  if index == 11 {
    yield_(l, 0);
  }
}

/// HANG 模式：不清零计数，越限后每次中断立即抛 timeout。
fn interrupt_hang(l: *mut LuaState) {
  // cpp `Conformance.test.cpp:3719-3727`：`index++; if (index >= 1'000)
  // luaL_error(L, "timeout");` —— 计数不清零，所以越限之后的每一次中断都会立刻再抛，
  // hang7 那种 `pcall(l0)` 的指数递归才会沿 sibling 分支在 O(深度) 步内被掐断；
  // 若在这里清零（下面 HANG_PCALL 的语义），sibling 分支会重新攒满 1000 次中断，
  // 用例就跑不完了。
  let index = CONFORMANCE_INTERRUPT_STATE
    .index
    .fetch_add(1, Ordering::SeqCst)
    + 1;

  if index >= 1_000 {
    // Safety: `l` 为存活受保护帧；`luaL_error` 经 long-jump 语义终止本 hook。
    unsafe { luaL_error!(&mut *l, "timeout") };
  }
}

/// HANG_PCALL 模式：每 1000 次中断抛一次 timeout，抛完清零重新计数。
fn interrupt_hang_pcall(l: *mut LuaState) {
  // cpp `Conformance.test.cpp:3745-3756`：`if (index == 1'000) { index = 0;
  // luaL_error(L, "timeout"); }` —— 每 1000 次中断抛一次，抛完重新计数，
  // hangpcall 的 100 轮 `pcall(string.find, ...)` 因此每轮都能收到一个 "timeout"。
  let index = CONFORMANCE_INTERRUPT_STATE
    .index
    .fetch_add(1, Ordering::SeqCst)
    + 1;

  if index == 1_000 {
    CONFORMANCE_INTERRUPT_STATE.index.store(0, Ordering::SeqCst);
    // Safety: 同 [`interrupt_hang`]——`luaL_error` 发散不返回。
    unsafe { luaL_error!(&mut *l, "timeout") };
  }
}

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_interrupt_interrupt(l: *mut LuaState, gc: c_int) {
  if gc >= 0 {
    return;
  }

  match CONFORMANCE_INTERRUPT_STATE.mode.load(Ordering::SeqCst) {
    CONFORMANCE_INTERRUPT_MODE_EXPECTED_HITS => interrupt_expected_hits(l),
    CONFORMANCE_INTERRUPT_MODE_INFLOOP => interrupt_infloop(l),
    CONFORMANCE_INTERRUPT_MODE_HANG => interrupt_hang(l),
    CONFORMANCE_INTERRUPT_MODE_HANG_PCALL => interrupt_hang_pcall(l),
    mode => panic!("unknown conformance interrupt mode {mode}"),
  }
}
