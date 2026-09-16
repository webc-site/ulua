//! Luau interrupt support. Mirrors `mlua::Lua::set_interrupt` / `VmState`.
//!
//! Luau's VM calls a single global `interrupt` callback at safepoints (loop
//! back-edges, calls/returns, GC). mlua exposes this as `Lua::set_interrupt`,
//! taking a Rust closure that returns a [`VmState`] telling the VM whether to
//! continue or to **yield** the current coroutine.
//!
//! ulua's `lua_callbacks().interrupt` is a plain C function pointer, so we
//! install a fixed trampoline ([`interrupt_trampoline`]) and keep the Rust
//! closure in a thread-local keyed by the VM's *global* pointer (shared by all
//! threads of one `Lua`). The trampoline looks up the closure, runs it with a
//! borrowed [`Lua`], and:
//!
//! * `Ok(VmState::Continue)`  — returns normally; the VM keeps executing.
//! * `Ok(VmState::Yield)`     — calls `lua_break`, which sets the running
//!   thread's status so the VM unwinds back to `lua_resume` (a *yield* at a
//!   yieldable point; ignored otherwise, exactly like upstream Luau).
//! * `Err(e)`                 — raises `e` as a Lua error via `lua_error`.

use core::ffi::c_void;
use std::{
  borrow::Cow,
  cell::{Cell, RefCell},
  collections::HashMap,
  panic::{AssertUnwindSafe, catch_unwind},
};

use ulua_vm::functions::lua_break::lua_break;

use crate::{
  callback::panic_message,
  error::{Error, Result},
  state::{Lua, vm_global_key},
  sync::MaybeSend,
  sys::*,
};

/// The action an interrupt callback asks the VM to take. Mirrors
/// `mlua::VmState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmState {
  /// Keep executing.
  Continue,
  /// Yield the currently running coroutine (no-op at a non-yieldable point).
  Yield,
}

/// The type-erased interrupt closure stored per-VM (clippy type_complexity
/// 认为内联过长，保留别名).
type InterruptFn = Box<dyn Fn(&Lua) -> Result<VmState>>;

thread_local! {
    /// Per-VM interrupt closure, keyed by the `global_State` pointer (stable for
    /// the lifetime of the VM and shared by all of its threads).
    static INTERRUPTS: RefCell<HashMap<*mut c_void, InterruptFn>> =
        RefCell::new(HashMap::new());

    /// `set_interrupt` / `remove_interrupt` 的累计变更次数。trampoline 用它
    /// 检测"用户回调内部改动了 interrupt"（含 remove），从而决定是否把取出的
    /// 闭包放回槽位。
    static INTERRUPT_REVISION: Cell<usize> = const { Cell::new(0) };
}

/// 记一次 interrupt 槽位的外部变更（install / remove）。
#[inline]
fn bump_revision() {
  INTERRUPT_REVISION.with(|r| r.set(r.get() + 1));
}

impl Lua {
  /// Install an interrupt callback. Mirrors `mlua::Lua::set_interrupt`.
  ///
  /// The callback runs at VM safepoints; returning [`VmState::Yield`] yields
  /// the running coroutine, and returning `Err` raises a Lua error.
  pub fn set_interrupt<F>(&self, callback: F)
  where
    F: Fn(&Lua) -> Result<VmState> + MaybeSend + 'static,
  {
    let state = self.state();
    let key = unsafe { vm_global_key(state) };
    INTERRUPTS.with(|m| {
      m.borrow_mut().insert(key, Box::new(callback));
    });
    bump_revision();
    unsafe {
      let cb = lua_callbacks(state);
      (*cb).interrupt = Some(interrupt_trampoline);
    }
  }

  /// Remove a previously installed interrupt callback. Mirrors
  /// `mlua::Lua::remove_interrupt`.
  pub fn remove_interrupt(&self) {
    let state = self.state();
    let key = unsafe { vm_global_key(state) };
    INTERRUPTS.with(|m| {
      m.borrow_mut().remove(&key);
    });
    bump_revision();
    unsafe {
      let cb = lua_callbacks(state);
      (*cb).interrupt = None;
    }
  }
}

/// Drop this VM's interrupt closure. Called from `LuaInner::drop` so the closure
/// (and anything it captured) is released and the per-VM map entry does not leak
/// one slot per state created. (If a closure captured Lua handles it would pin
/// the VM and this never runs — but the common case captures non-Lua state.)
pub(crate) fn clear_interrupt(state: *mut lua_State) {
  let key = unsafe { vm_global_key(state) };
  INTERRUPTS.with(|m| {
    m.borrow_mut().remove(&key);
  });
}

/// The fixed C trampoline installed as `lua_callbacks().interrupt`.
///
/// `gc` is non-negative only for GC interrupts; mlua ignores GC interrupts in
/// the user callback path, and so do we (return immediately) so the user
/// closure only sees real instruction safepoints.
unsafe extern "C-unwind" fn interrupt_trampoline(state: *mut lua_State, gc: c_int) {
  if gc >= 0 {
    // GC step interrupt — not surfaced to the user callback.
    return;
  }
  let key = unsafe { vm_global_key(state) };
  // Take the closure out of the map for the duration of the call so a
  // re-entrant `set_interrupt` from inside the callback can't alias the
  // borrow. The revision counter tells us afterwards whether the callback
  // itself installed a new one or removed it (both must win over the taken
  // closure — a `remove_interrupt` inside the callback must stay removed).
  let revision = INTERRUPT_REVISION.with(Cell::get);
  let cb = INTERRUPTS.with(|m| m.borrow_mut().remove(&key));
  let Some(cb) = cb else { return };

  let lua = unsafe { Lua::from_borrowed(state) };
  // The callback is user code: a panic must not unwind through the VM's
  // frames (it would corrupt the interpreter state mid-safepoint). Convert it
  // into a catchable Lua error instead, like the callback trampoline does.
  let result = catch_unwind(AssertUnwindSafe(|| cb(&lua)));

  let outcome = match result {
    Ok(r) => r,
    Err(payload) => {
      // Surface the panic as a catchable Lua error (the taken closure stays
      // out of the map).
      let msg = panic_message(&payload);
      unsafe { raise_error(state, &Error::runtime(format!("rust panic: {msg}"))) };
    }
  };

  // Restore the taken closure only if the callback left the slot untouched.
  if INTERRUPT_REVISION.with(Cell::get) == revision {
    INTERRUPTS.with(|m| {
      m.borrow_mut().insert(key, cb);
    });
  }

  match outcome {
    Ok(VmState::Continue) => {}
    Ok(VmState::Yield) => unsafe {
      // Request a yield — but only at a yieldable point. Inside a
      // metamethod / C-call boundary Luau's `lua_break` would raise
      // "attempt to break across metamethod/C-call boundary"; upstream
      // (and mlua) silently ignore the yield request there, so we gate it
      // on `lua_isyieldable` and otherwise just continue.
      if lua_isyieldable(state) != 0 {
        let _ = lua_break(state);
      }
    },
    Err(e) => unsafe {
      // Raise the error as a Lua error. Push the message and longjmp.
      raise_error(state, &e);
    },
  }
}

/// Push `e`'s message as a string error object and `lua_error` it (does not
/// return).
unsafe fn raise_error(state: *mut lua_State, e: &Error) -> ! {
  // Use the bare message for a runtime error (so it round-trips back through
  // `pop_error` as `RuntimeError(msg)` without a doubled "runtime error: "
  // prefix); fall back to the full Display for other error kinds.
  let msg: Cow<'_, str> = match e {
    Error::RuntimeError(m) => Cow::Borrowed(m.as_str()),
    other => Cow::Owned(other.to_string()),
  };
  unsafe {
    // The interrupt fires at an arbitrary VM safepoint where `l->top` may be
    // flush against the call-info top; make room before pushing so the
    // `api_incr_top` stack invariant in `lua_pushlstring` holds.
    lua_rawcheckstack(state, 1);
    lua_pushlstring(state, msg.as_ptr() as *const c_char, msg.len());
    lua_error(state)
  }
}
