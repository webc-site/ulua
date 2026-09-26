//! Luau interrupt support. Mirrors `mlua::Lua::set_interrupt` / `VmState`.
//!
//! Luau's VM calls a single global `interrupt` callback at safepoints (loop
//! back-edges, calls/returns, GC). mlua exposes this as `Lua::set_interrupt`,
//! taking a Rust closure that returns a [`VmState`] telling the VM whether to
//! continue or to **yield** the current coroutine.
//!
//! ulua's `lua_callbacks().interrupt` is a plain C function pointer, so we
//! install a fixed trampoline ([`interrupt_trampoline`]) and keep the Rust
//! closure in the crate-wide per-VM table (see [`crate::vm_store`]) keyed by the
//! VM's *global* pointer (shared by all threads of one `Lua`). The trampoline
//! looks up the closure, runs it with a borrowed [`Lua`], and:
//!
//! * `Ok(VmState::Continue)`  — returns normally; the VM keeps executing.
//! * `Ok(VmState::Yield)`     — calls `lua_break`, which sets the running
//!   thread's status so the VM unwinds back to `lua_resume` (a *yield* at a
//!   yieldable point; ignored otherwise, exactly like upstream Luau).
//! * `Err(e)`                 — raises `e` as a Lua error via `lua_error`.

use std::{
  borrow::Cow,
  cell::Cell,
  panic::{AssertUnwindSafe, catch_unwind},
};

use ulua_vm::functions::lua_break::lua_break;

use crate::{
  callback::{panic_error_message, raise_lua_error},
  error::{Error, Result},
  state::Lua,
  sync::MaybeSend,
  sys::*,
  vm_store::{VmKey, define_vm_store, vm_key},
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
///
/// 与 `callback.rs` 的同步回调不同：interrupt 是**每 VM 一枚**的可替换槽位
/// （[`Lua::set_interrupt`] 可随时换掉，故无法按闭包具体类型单态化），只能以
/// `dyn` 形式存进 per-VM 表。
///
/// Under `send` the boxed closure is `Send`: the closure is installed through
/// [`Lua::set_interrupt`], which already requires [`MaybeSend`], and the per-VM
/// table holding it is process-wide under that feature (see [`crate::vm_store`]).
#[cfg(feature = "send")]
type InterruptFn = Box<dyn Fn(&Lua) -> Result<VmState> + Send>;

/// See the `send`-gated variant above.
#[cfg(not(feature = "send"))]
type InterruptFn = Box<dyn Fn(&Lua) -> Result<VmState>>;

define_vm_store! {
  /// Per-VM interrupt closure, keyed by the `global_State` pointer (stable for
  /// the lifetime of the VM and shared by all of its threads).
  InterruptStore, InterruptFn
}

thread_local! {
    /// `set_interrupt` / `remove_interrupt` 的累计变更次数。trampoline 用它
    /// 检测"用户回调内部改动了 interrupt"（含 remove），从而决定是否把取出的
    /// 闭包放回槽位。
    ///
    /// 与 [`InterruptStore`] 不同，这个计数器**必须**留在 thread-local：它只在
    /// 单次 safepoint 内被读—改—比较，全程发生在驱动 VM 的那个线程上，跨 VM 移动
    /// 也不影响其正确性（下标只与同一次 trampoline 调用里的读数比较）。
    static INTERRUPT_REVISION: Cell<usize> = const { Cell::new(0) };
}

/// 记一次 interrupt 槽位的外部变更（install / remove）。
#[inline]
fn bump_revision() {
  INTERRUPT_REVISION.with(|r| r.set(r.get() + 1));
}

/// 若自 `revision` 读数后无人改动本 VM 的 interrupt 槽位（既没 `set_interrupt`
/// 换掉、也没 `remove_interrupt` 摘掉），就把 safepoint 期间取出的闭包 `cb`
/// 放回槽内；否则尊重那次变更、丢弃 `cb`。trampoline 的 panic 分支与正常分支
/// 共用此收口（原两处 `if … == revision { insert }` 逐字同形）。
#[inline]
fn restore_if_untouched(key: VmKey, cb: InterruptFn, revision: usize) {
  if INTERRUPT_REVISION.with(Cell::get) == revision {
    InterruptStore::with(|m| {
      m.insert(key, cb);
    });
  }
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
    // Safety: `state` 为存活 VM 状态（`&self` 的 `XRc<LuaInner>` 持有），
    // `global` 构造期接线非空——满足 `vm_key` 契约。
    let key = unsafe { vm_key(state) };
    InterruptStore::with(|m| {
      m.insert(key, Box::new(callback));
    });
    bump_revision();
    // Safety: `lua_callbacks(state)` 返回 `&mut global_State.cb`——`global`
    // 非空且比持有者长寿，字段内嵌于 `global_State`，地址稳定。写 `interrupt`
    // 槽与读它的 safepoint 都发生在当前驱动 VM 的线程上（`send` 契约为移动
    // 而非并发），无并发写；表项先于钩子就位（上方 insert），不存在钩子已
    // 装但闭包缺失的窗口。
    unsafe {
      let cb = &mut *lua_callbacks(state);
      cb.interrupt = Some(interrupt_trampoline);
    }
  }

  /// Remove a previously installed interrupt callback. Mirrors
  /// `mlua::Lua::remove_interrupt`.
  pub fn remove_interrupt(&self) {
    let state = self.state();
    // Safety: 同 `set_interrupt`——`state` 存活，`global` 非空长寿。
    let key = unsafe { vm_key(state) };
    InterruptStore::with(|m| {
      m.remove(&key);
    });
    bump_revision();
    // Safety: 同 `set_interrupt` 的写钩子论证；先摘闭包再置 `None`，反序也
    // 安全（trampoline 对表缺失直接 return），只读写字段均为 fn 指针槽。
    unsafe {
      let cb = &mut *lua_callbacks(state);
      cb.interrupt = None;
    }
  }
}

/// Drop this VM's interrupt closure. Called from `LuaInner::drop` so the closure
/// (and anything it captured) is released and the per-VM map entry does not leak
/// one slot per state created. (If a closure captured Lua handles it would pin
/// the VM and this never runs — but the common case captures non-Lua state.)
pub(crate) fn clear_interrupt(state: *mut LuaState) {
  // Safety: 唯一调用点是 `LuaInner::drop` 中 `lua_close` **之前**的 clear 序列
  // （见 `state.rs` Drop），此刻 state 与 `global` 均存活，`vm_key` 契约成立。
  let key = unsafe { vm_key(state) };
  InterruptStore::with(|m| {
    m.remove(&key);
  });
}

/// The fixed C trampoline installed as `lua_callbacks().interrupt`.
///
/// `gc` is non-negative only for GC interrupts; mlua ignores GC interrupts in
/// the user callback path, and so do we (return immediately) so the user
/// closure only sees real instruction safepoints.
///
/// # Safety
/// 仅由 VM 在 safepoint 回调：`state` 存活且正由当前线程驱动，其 `global` 即本
/// VM 的 `vm_key`；安装/摘除 trampoline 与查表都走同一 key，表项生命周期由
/// `InterruptStore` 同 per-VM 表保证。
unsafe extern "C-unwind" fn interrupt_trampoline(state: *mut LuaState, gc: c_int) {
  if gc >= 0 {
    // GC step interrupt — not surfaced to the user callback.
    return;
  }
  // Safety: `state` 由 VM 在 safepoint 回调内提供（gc<0 分支只在指令 safepoint
  // 到达），存活且正由当前线程驱动；其 `global` 即本 VM 的表 key，
  // trampoline 装表与摘表（`remove_interrupt`/`clear_interrupt`）都经同一 key。
  let key = unsafe { vm_key(state) };
  // Take the closure out of the map for the duration of the call so a
  // re-entrant `set_interrupt` from inside the callback can't alias the
  // borrow. The revision counter tells us afterwards whether the callback
  // itself installed a new one or removed it (both must win over the taken
  // closure — a `remove_interrupt` inside the callback must stay removed).
  let revision = INTERRUPT_REVISION.with(Cell::get);
  let cb = InterruptStore::with(|m| m.remove(&key));
  let Some(cb) = cb else { return };

  // `from_borrowed` 现为带契约的安全封装（契约见 state.rs 文档）：`state` 在
  // 本次调用全程存活（VM 正驱动到 safepoint），覆盖返回句柄及其克隆——
  // 借用语义（`owned:false`）保证该句柄 drop 时不会关 VM；与 `callback.rs`
  // trampoline 同纪律：闭包不得把克隆出的句柄寄存到超出本次回调。
  let lua = Lua::from_borrowed(state);
  // The callback is user code: a panic must not unwind through the VM's
  // frames (it would corrupt the interpreter state mid-safepoint). Convert it
  // into a catchable Lua error instead, like the callback trampoline does.
  let result = catch_unwind(AssertUnwindSafe(|| cb(&lua)));

  let outcome = match result {
    Ok(r) => r,
    Err(payload) => {
      // Surface the panic as a catchable Lua error. mlua 语义：回调 panic 后
      // 中断回调保持安装——先按 revision 把取出的闭包放回槽内再 diverge，
      // 否则 cb.interrupt trampoline 仍挂着而槽内无闭包，后续 safepoint
      // 会静默 no-op（中断被永久禁用）。
      restore_if_untouched(key, cb, revision);
      // Safety: 正处 interrupt trampoline 内——VM 的 C 边界 safepoint，
      // 是 `raise_error`/`raise_lua_error` 文档契约要求的"可吸收 lua_error
      // 展开的执行点"；panic 已被 `catch_unwind` 拦下，展开从此刻重新以
      // Lua 错误形式走受保护路径。取出的闭包有意留在槽外（错误后 VM 不再
      // 回到本 safepoint 序列的恢复代码，`raise_error` diverges）。
      unsafe { raise_error(state, &Error::runtime(panic_error_message(&*payload))) };
    }
  };

  // Restore the taken closure only if the callback left the slot untouched.
  restore_if_untouched(key, cb, revision);

  match outcome {
    Ok(VmState::Continue) => {}
    Ok(VmState::Yield) => {
      // Request a yield — but only at a yieldable point. Inside a
      // metamethod / C-call boundary Luau's `lua_break` would raise
      // "attempt to break across metamethod/C-call boundary"; upstream
      // (and mlua) silently ignore the yield request there, so we gate it
      // on `lua_isyieldable` and otherwise just continue.
      // Safety: `state` 为 safepoint 上正在执行的线程（VM 回调参数）；
      // `lua_isyieldable` 只读该线程当前是否处于可让出点，不触指针、不动栈。
      let yieldable = unsafe { lua_isyieldable(state) } != 0;
      if yieldable {
        // Safety: 上一行已探测到可让出点——`lua_break` 只在该点设状态标记令 VM
        // 展开回 `lua_resume`（越界会 raise 的错误已被上面的闸门挡下），不触指针。
        let _ = unsafe { lua_break(state) };
      }
    }
    Err(e) => {
      // Raise the error as a Lua error. Push the message and longjmp.
      // Safety: 同 panic 分支——此刻仍在 trampoline 的 C 边界内，是
      // `raise_error` 契约要求的位置；用户闭包已正常返回 `Err`，无借用悬存，
      // 展开经外层受保护调用转成可捕获的 Lua 错误。
      unsafe { raise_error(state, &e) }
    }
  }
}

/// Push `e`'s message as a string error object and `lua_error` it (diverges).
/// push + `lua_error` 样板复用 [`raise_lua_error`]。
///
/// # Safety
/// `state` 必须正处在可吸收 `lua_error` 展开的执行点（VM 中断/回调的 C 边界
/// 内，即 [`raise_lua_error`] 的前提），且存活、由当前线程驱动。栈空位由本
/// 函数自己经 `lua_rawcheckstack` 预留，调用方无需额外保证。
unsafe fn raise_error(state: *mut LuaState, e: &Error) -> ! {
  // Safety: 满足本函数 `# Safety` 契约——仅在 VM 中断回调的 C 边界内被调用，
  // 栈空位由内部 `raise_lua_error` 路径自行预留。
  // Use the bare message for a runtime error (so it round-trips back through
  // `pop_error` as `RuntimeError(msg)` without a doubled "runtime error: "
  // prefix); fall back to the full Display for other error kinds.
  let msg: Cow<'_, str> = match e {
    Error::RuntimeError(m) => Cow::Borrowed(m.as_str()),
    other => Cow::Owned(other.to_string()),
  };
  // Safety: 满足本 `unsafe fn` 契约——两个调用点都在 interrupt_trampoline 的
  // safepoint 边界内（见彼处证成）。`msg` 为存活 `Cow<str>` 的字节切片
  // （len 可为 0，指针仍有效；`lua_pushlstring` 返回前完整拷贝字节），
  // `lua_rawcheckstack` 自留这一层头寸（safepoint 处 `top` 可能恰抵 `ci->top`，
  // 注释如上），`raise_lua_error` 的 push+`lua_error` 前提由此齐备。
  unsafe {
    // The interrupt fires at an arbitrary VM safepoint where `l->top` may be
    // flush against the call-info top; make room before pushing so the
    // `api_incr_top` stack invariant in `lua_pushlstring` holds.
    lua_rawcheckstack(state, 1);
    raise_lua_error(state, &msg)
  }
}
