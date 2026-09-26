use core::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};

use ulua_vm::records::lua_state::LuaState;

/// 断点/单步命中计数与被中断线程登记（cpp debuggerHook 的静态量）。
#[derive(Debug)]
pub struct ConformanceDebuggerState {
  pub breakhits: AtomicI32,
  /// 「无登记」的裸指针空哨兵收进本字段：以地址 `0` 位型表示 `None`，
  /// 对外只经 [`Self::set_interruptedthread`] / [`Self::take_interruptedthread`]
  /// 的 `Option` 接口存取。
  interruptedthread: AtomicUsize,
  pub singlestep: AtomicBool,
  pub stephits: AtomicI32,
}

impl ConformanceDebuggerState {
  pub const fn new() -> Self {
    Self {
      breakhits: AtomicI32::new(0),
      interruptedthread: AtomicUsize::new(0),
      singlestep: AtomicBool::new(false),
      stephits: AtomicI32::new(0),
    }
  }

  pub fn reset(&self, singlestep: bool) {
    self.breakhits.store(0, Ordering::SeqCst);
    self.set_interruptedthread(None);
    self.singlestep.store(singlestep, Ordering::SeqCst);
    self.stephits.store(0, Ordering::SeqCst);
  }

  /// 登记被中断线程；`None` 表示清空登记。`Option<*mut _>` 与地址位型一一对应
  /// （`None` ⇔ 0，合法状态指针恒为非零地址）。
  pub fn set_interruptedthread(&self, thread: Option<*mut LuaState>) {
    let addr = thread.map_or(0, |p| p.addr());
    self.interruptedthread.store(addr, Ordering::SeqCst);
  }

  /// 原子取出并清空当前登记；无登记时返回 `None`。
  pub fn take_interruptedthread(&self) -> Option<*mut LuaState> {
    match self.interruptedthread.swap(0, Ordering::SeqCst) {
      0 => None,
      addr => Some(addr as *mut LuaState),
    }
  }
}

impl Default for ConformanceDebuggerState {
  fn default() -> Self {
    Self::new()
  }
}

pub static CONFORMANCE_DEBUGGER_STATE: ConformanceDebuggerState = ConformanceDebuggerState::new();
