//! Luau-specific `Lua` extensions: sandboxing, safeenv, fflags, and the
//! per-VM compiler. Mirrors the Luau-only parts of mlua's `Lua` surface.

use core::ptr::null_mut;

use crate::{
  compiler::Compiler,
  error::{Error, Result},
  function::Function,
  light_userdata::LightUserData,
  registry::RegHandle,
  state::{Lua, ensure_stack, ensure_stack_or_panic},
  string::LuaString,
  sys::*,
  table::Table,
  thread::Thread,
  value::Value,
  vector::Vector,
  vm_store::{define_vm_store, vm_key, vm_key_of},
};

define_vm_store! {
    /// Per-VM compiler installed via `Lua::set_compiler`, keyed by global state.
    VmCompilers, Compiler
}

define_vm_store! {
    /// Per-VM `catch_rust_panics` option (recorded from `LuaOptions`), keyed by
    /// global state. Currently recorded for parity; see `set_catch_rust_panics`.
    VmCatchPanics, bool
}

define_vm_store! {
    /// Per-VM "is sandboxed" flag, keyed by global state. Mirrors mlua's
    /// `extra.sandboxed`; consulted by `Lua::set_globals`.
    VmSandboxed, bool
}

/// Registry name under which `sandbox(true)` saves the ORIGINAL globals table so
/// `sandbox(false)` can restore it. Kept in the state's registry (freed on
/// `lua_close`), NOT in a thread-local `Table` handle: a handle owns an
/// `XRc<LuaInner>`, so a thread-local holding it would pin the VM alive forever
/// (the state could never drop) if the caller dropped a still-sandboxed `Lua`.
const SANDBOX_SAVED_GLOBALS_NAME: &str = "__ulua_sandbox_saved_globals";

/// Drop this VM's compiler / catch-panics / sandboxed-flag entries so they don't
/// leak one slot per state created. Called from `LuaInner::drop`. (These hold no
/// Lua handle, so the state drops normally and this runs; the saved-globals table
/// lives in the registry and is freed with the state on `lua_close`.)
pub(crate) fn clear_vm_state(state: *mut LuaState) {
  // Safety: 本函数只在 `LuaInner::drop` 里、`lua_close` 之前调用，`state` 是
  // 该存活 VM 的 main state；`vm_key` 只读其 `global` 指针的地址作整数 key，
  // 不解引用。
  let key = unsafe { vm_key(state) };
  VmCompilers::with(|m| {
    m.remove(&key);
  });
  VmCatchPanics::with(|m| {
    m.remove(&key);
  });
  VmSandboxed::with(|m| {
    m.remove(&key);
  });
}

impl Lua {
  /// Enable or disable sandbox mode. Mirrors `mlua::Lua::sandbox`.
  ///
  /// Enabling sets every library table (and the globals table) read-only and
  /// activates `safeenv`, then installs a fresh proxy global table (via
  /// `luaL_sandboxthread`) so that script-level global writes go to a
  /// throwaway table whose `__index` is the original environment. Disabling
  /// restores the original globals table and clears the read-only/safeenv
  /// flags.
  ///
  /// **DEVIATION:** Luau's standard library (as bundled in ulua) does not
  /// register `collectgarbage`; mlua's sandbox test additionally checks that
  /// `collectgarbage` is restricted under the sandbox. That part is not
  /// exercisable here (see `tests/mlua_luau.rs`).
  pub fn sandbox(&self, enabled: bool) -> Result<()> {
    let state = self.state();
    let key = vm_key_of(self);
    VmSandboxed::with(|m| {
      m.insert(key, enabled);
    });
    if enabled {
      // Save the ORIGINAL globals table (once) in the registry so we can
      // restore it later. Registry-rooted, not a thread-local handle —
      // see SANDBOX_SAVED_GLOBALS_NAME. `or_insert` semantics: only save
      // if not already saved (a second sandbox(true) keeps the first)。
      if self
        .named_registry_value::<Table>(SANDBOX_SAVED_GLOBALS_NAME)
        .is_err()
      {
        let original = self.globals();
        let _ = self.set_named_registry_value(SANDBOX_SAVED_GLOBALS_NAME, original);
      }
      // Make libraries + base metatables read-only and set safeenv.
      // Safety: `state` 由 self 的 `XRc<LuaInner>` 锚定存活且由当前线程驱动。
      // `lua_l_sandbox` 是宿主侧对存活 main state 的 Luau 沙箱 API：内部压弹的临时
      // 值全自平衡，不跨入任何 Rust 借用指针。
      unsafe { lua_l_sandbox(state) };
      // Install the proxy global table for script-level writes.
      // Safety: 同上——`lua_l_sandboxthread` 只对该 main state 自身的 `LUA_GLOBALSINDEX`
      // 安装代理表，push/replace 自平衡，不触碰其它状态、不跨 Rust 借用指针。
      unsafe { lua_l_sandboxthread(state) };
    } else {
      // Restore the original globals table (dropping the proxy and any
      // globals written into it), then clear the saved slot。
      if let Ok(orig) = self.named_registry_value::<Table>(SANDBOX_SAVED_GLOBALS_NAME) {
        // 预留 `orig` 压入的一层（`lua_replace` 随即弹掉）。
        ensure_stack(state, 1)?;
        // `orig.push_to_stack()` 是 safe 封装（`lua_rawgeti` 自带栈预留）；owning VM
        // 与本 VM 一致由 move-not-share 句柄纪律保证，同 `set_globals`。
        orig.push_to_stack();
        // Safety: 栈顶即刚压入的 `orig`，`lua_replace` 把它写到合法伪索引
        // `LUA_GLOBALSINDEX`，净弹一层，头寸由上面的 `ensure_stack` 覆盖。
        unsafe { lua_replace(state, LUA_GLOBALSINDEX) };
        // Clear read-only + safeenv on the restored globals so it is
        // writable again.
        // Safety: 对同一伪索引 `LUA_GLOBALSINDEX` 的表写只读标志位，不压弹栈。
        unsafe { lua_setreadonly(state, LUA_GLOBALSINDEX, 0) };
        // safeenv 的清除复用 `Self::set_safeenv`（safe fn，其契约在自身调用点）。
        self.set_safeenv(false);
        // Also clear read-only on the library tables.
        self.clear_library_readonly();
        // Clear the saved slot so a later sandbox(true) re-saves.
        let _ = self.set_named_registry_value(SANDBOX_SAVED_GLOBALS_NAME, Value::Nil);
      }
    }
    Ok(())
  }

  /// Clear the read-only flag on every library table reachable from the
  /// (restored) globals. Used when leaving sandbox mode.
  fn clear_library_readonly(&self) {
    let globals = self.globals();
    for pair in globals.pairs::<Value, Value>() {
      if let Ok((_, Value::Table(t))) = pair {
        t.set_readonly(false);
      }
    }
  }

  /// Set or clear the `safeenv` flag on the globals table. Mirrors
  /// `mlua::Globals::set_safeenv` applied to the main globals.
  ///
  /// `safeenv` lets the VM fast-path global reads; clearing it forces the slow
  /// path (needed when globals/`__index` may change at runtime).
  pub fn set_safeenv(&self, enabled: bool) {
    let state = self.state();
    // Safety: `state` 存活（self 的 `XRc<LuaInner>`）；运行 VM 中
    // `LUA_GLOBALSINDEX` 恒指向 globals 表，满足 `lua_setsafeenv` 对
    // objindex 解出表的内部断言；该调用只在表对象上翻 safeenv 标志位，
    // 不压弹栈、不解引用任何 Rust 侧指针。
    unsafe {
      lua_setsafeenv(state, LUA_GLOBALSINDEX, enabled as c_int);
    }
  }

  /// Install a default [`Compiler`] used to compile every chunk loaded by this
  /// VM (unless a chunk overrides it via
  /// [`Chunk::set_compiler`](crate::Chunk::set_compiler)). Mirrors
  /// `mlua::Lua::set_compiler`.
  pub fn set_compiler(&self, compiler: Compiler) {
    let key = vm_key_of(self);
    VmCompilers::with(|m| {
      m.insert(key, compiler);
    });
  }

  /// Record the `catch_rust_panics` behavioral option for this VM.
  ///
  /// **DEVIATION:** ulua-rt's callback trampoline always catches a Rust panic
  /// and converts it into a catchable Lua error (so the VM is never left
  /// half-unwound). The mlua option that lets a panic propagate as a Rust
  /// unwind across the VM boundary is therefore recorded here but not enforced
  /// — see the deferred `test_panic` in `tests/mlua_core.rs`.
  pub(crate) fn set_catch_rust_panics(&self, enabled: bool) {
    let key = vm_key_of(self);
    VmCatchPanics::with(|m| {
      m.insert(key, enabled);
    });
  }

  /// Whether this VM is currently sandboxed (set by [`Lua::sandbox`]). Mirrors
  /// mlua's `extra.sandboxed` flag; consulted by [`Lua::set_globals`].
  pub(crate) fn is_sandboxed(&self) -> bool {
    let key = vm_key_of(self);
    VmSandboxed::with(|m| m.get(&key).copied().unwrap_or(false))
  }

  /// The VM-default compiler installed via [`Lua::set_compiler`], if any.
  pub(crate) fn vm_compiler(&self) -> Option<Compiler> {
    let key = vm_key_of(self);
    VmCompilers::with(|m| m.get(&key).cloned())
  }

  /// Set (or clear) the metatable shared by all values of a Luau built-in
  /// type `T`. Mirrors `mlua::Lua::set_type_metatable`.
  ///
  /// Implemented for [`Vector`], `bool`, [`Number`](f64),
  /// [`LuaString`], [`Function`],
  /// [`Thread`], and
  /// [`LightUserData`]. Setting it installs a metatable
  /// in the VM's global per-type metatable slot, so e.g. `v.x`/`v:method`
  /// dispatch through it.
  pub fn set_type_metatable<T: TypeMetatable>(&self, metatable: Option<Table>) {
    T::set_type_metatable(self, metatable);
  }

  /// The metatable shared by all values of a Luau built-in type `T`, if one
  /// has been installed. Mirrors `mlua::Lua::type_metatable`.
  pub fn type_metatable<T: TypeMetatable>(&self) -> Option<Table> {
    T::type_metatable(self)
  }

  /// Set a Luau fast-flag (FFlag) by name. Mirrors `mlua::Lua::set_fflag`.
  ///
  /// **DEVIATION:** ulua's FastFlags are a fixed, compile-time `FFlag` enum
  /// rather than a string-keyed registry, so there is no way to look a flag up
  /// by an arbitrary name. This therefore always reports the name as unknown
  /// (`Err`) — which matches mlua's contract for an unrecognized flag (the
  /// only behavior its `test_fflags` asserts). Known flags are configured at
  /// VM-construction time via `ulua_common::set_luau_bool_flags`.
  pub fn set_fflag(name: &str, _enabled: bool) -> Result<()> {
    Err(Error::runtime(format!("fflag '{name}' is not supported")))
  }
}

impl Thread {
  /// Sandbox this coroutine: install a fresh proxy global table on its own
  /// state so global writes inside the coroutine stay local to it. Mirrors
  /// `mlua::Thread::sandbox`.
  pub fn sandbox(&self) -> Result<()> {
    let co = self.thread_state.as_ptr();
    // Safety: `co` 存活——Thread 的注册表引用锚定其线程值，值可达期间协程
    // 对象不被 GC（`Thread::from_ref` 的构造接线）。`lua_l_sandboxthread`
    // 只对该协程自身的 `LUA_GLOBALSINDEX` 安装代理表，内部 push/replace 自
    // 平衡，不触碰其它状态、不跨 Rust 借用指针。
    unsafe {
      lua_l_sandboxthread(co);
    }
    Ok(())
  }
}

/// 空串的 NUL 结尾字节串形态：`TypeMetatable::push_representative` 用作 `lua_pushlstring`
/// 的空串代表值与 `lua_pushcclosurek` 的空 `debugname`，交给 `*const c_char` 收口点。
const EMPTY_NUL: &[u8] = b"\0";

/// Luau built-in types that have a shared, per-type metatable settable via
/// [`Lua::set_type_metatable`]. Mirrors mlua's sealed `LuauType` trait.
pub trait TypeMetatable: private::Sealed {
  /// Push a representative value of this type onto the stack (so the VM's
  /// `lua_setmetatable`/`lua_getmetatable` operate on the type's global slot).
  ///
  /// # Safety
  /// `state` must be a live `LuaState` driven by the current thread with at
  /// least one free stack slot; the implementation must push exactly one value
  /// of this Lua type and nothing else (callers pop precisely one slot).
  #[doc(hidden)]
  unsafe fn push_representative(state: *mut LuaState);

  /// Install (or clear) the shared metatable for this type.
  fn set_type_metatable(lua: &Lua, metatable: Option<Table>) {
    let state = lua.state();
    // 栈峰值：代表值 + 元表/nil 两层（`lua_setmetatable` 弹元表、结尾弹代表值）。
    ensure_stack_or_panic(state, 2);
    // Safety: 紧邻 `ensure_stack_or_panic(state, 2)` 预留两层，且 state 由本
    // 方法自 `lua.state()` 取得、存活并由当前线程驱动，满足 `push_representative`
    // 的契约（push 恰一个代表值）。
    unsafe { Self::push_representative(state) };
    // Safety: 栈上已有一层代表值，剩余头寸覆盖本步再压一层。None 分支
    // `lua_pushnil` 压恰一层（`state` 存活）；Some 分支 `mt.push_to_stack` 是 safe
    // 封装（owning VM 一致由 move-not-share 句柄纪律保证，同 `set_globals`）。
    unsafe {
      match metatable {
        Some(mt) => mt.push_to_stack(),
        None => lua_pushnil(state),
      }
    }
    // For a non-table/non-userdata value, `lua_setmetatable` stores the
    // metatable in the VM's global per-type slot (`g->mt[type]`).
    // Safety: 此刻栈深≥2，`-2` 正指刚压入前的代表值（元表/nil 已被本调用弹走）；
    // 对非 table/userdata 基元类型按 C 约定写全局类型元表槽。
    unsafe { lua_setmetatable(state, -2) };
    // Pop the representative value left on the stack.
    // Safety: 栈顶即代表值，弹走它，净栈变化为零。
    unsafe { lua_pop(state, 1) };
  }

  /// The shared metatable for this type, if installed.
  fn type_metatable(lua: &Lua) -> Option<Table> {
    let state = lua.state();
    // 栈峰值：代表值 + `lua_getmetatable` 结果两层。
    ensure_stack_or_panic(state, 2);
    // Safety: 紧邻 `ensure_stack_or_panic(state, 2)` 预留两层，state 存活且
    // 由当前线程驱动，满足 `push_representative` 契约（push 恰一个代表值）。
    unsafe { Self::push_representative(state) };
    // Safety: 栈顶即刚压入的代表值，`-1` 指它；命中时 `lua_getmetatable` 恰再压
    // 一层（元表），返回非 0。
    let has = unsafe { lua_getmetatable(state, -1) };
    if has == 0 {
      // No metatable: pop the representative value.
      // Safety: 未命中不压新层，栈顶仍是代表值，弹走它，净栈变化为零。
      unsafe { lua_pop(state, 1) };
      return None;
    }
    // stack: [value, metatable]
    // `pop_ref`（safe fn）弹走栈顶元表并登记注册表引用（引用可达即元表存活）。
    let mt = Table::from_ref(lua.pop_ref());
    // Safety: 栈顶现为代表值，弹走它，净栈变化为零。
    unsafe { lua_pop(state, 1) };
    Some(mt)
  }
}

mod private {
  use super::*;

  pub trait Sealed {}
  impl Sealed for Vector {}
  impl Sealed for bool {}
  impl Sealed for f64 {}
  impl Sealed for LuaString {}
  impl Sealed for Function {}
  impl Sealed for Thread {}
  impl Sealed for LightUserData {}
}

impl TypeMetatable for Vector {
  /// # Safety
  /// Upholds the trait contract: pushes exactly one vector onto a live state.
  unsafe fn push_representative(state: *mut LuaState) {
    // Safety: 契约保证 state 存活、由当前线程驱动且有 ≥1 空位；push 恰一个
    // vector 值，四个分量均为常量 0.0，不涉及任何借用指针。
    unsafe {
      lua_pushvector_lua_state_f32_f32_f32_f32(state, 0.0, 0.0, 0.0, 0.0);
    }
  }
}

impl TypeMetatable for bool {
  /// # Safety
  /// Upholds the trait contract: pushes exactly one boolean onto a live state.
  unsafe fn push_representative(state: *mut LuaState) {
    // Safety: 契约保证 state 存活且有 ≥1 空位；`lua_pushboolean` 压恰一个
    // 布尔值，参数是常量。
    unsafe { lua_pushboolean(state, 0) }
  }
}

impl TypeMetatable for f64 {
  /// # Safety
  /// Upholds the trait contract: pushes exactly one number onto a live state.
  unsafe fn push_representative(state: *mut LuaState) {
    // Safety: 契约保证 state 存活且有 ≥1 空位；`lua_pushnumber` 压恰一个
    // 数值，参数是常量。
    unsafe { lua_pushnumber(state, 0.0) }
  }
}

impl TypeMetatable for LuaString {
  /// # Safety
  /// Upholds the trait contract: pushes exactly one string onto a live state.
  unsafe fn push_representative(state: *mut LuaState) {
    // Safety: 契约保证 state 存活且有 ≥1 空位。空串代表值：`EMPTY_NUL` 是静态
    // NUL 结尾字节串，`.as_ptr().cast()` 交出 `*const c_char`；指针指向静态存储
    // （len=0 也不读字节，指针仍有效）；`lua_pushlstring` 把内容拷成内部驻留
    // TString 后压恰一个字符串值，不持有入参指针。
    unsafe {
      lua_pushlstring(state, EMPTY_NUL.as_ptr().cast(), 0);
    }
  }
}

impl TypeMetatable for Function {
  /// # Safety
  /// Upholds the trait contract: pushes exactly one closure onto a live state
  /// (`noop_cfn` is a total C-ABI trampoline returning 0, used nowhere else).
  unsafe fn push_representative(state: *mut LuaState) {
    // Push a throwaway C function so `lua_setmetatable` targets the global
    // function-type slot.
    // Safety: 契约保证 state 存活且有 ≥1 空位。`noop_cfn` 是
    // `extern "C-unwind"` 全函数（恒返回 0），与 `LuaCFunction` 的
    // C ABI 约定兼容；`nup=0` 不消费栈、`cont=None` 为合法空续体。关键是
    // `debugname` 指针会被闭包长期持有并在调试路径解引用——这里传的
    // `EMPTY_NUL` 是 `'static` 静态 NUL 结尾字节串，永不失效。压恰一个闭包值。
    unsafe {
      lua_pushcclosurek(state, Some(noop_cfn), EMPTY_NUL.as_ptr().cast(), 0, None);
    }
  }
}

impl TypeMetatable for Thread {
  /// # Safety
  /// Upholds the trait contract: pushes exactly one fresh thread onto a live
  /// state (the stack slot itself keeps it reachable until the caller pops it).
  unsafe fn push_representative(state: *mut LuaState) {
    // A fresh thread targets the global thread-type slot.
    // Safety: 契约保证 state 存活且有 ≥1 空位；`lua_newthread` 成功时压恰
    // 一个线程值（新协程由栈槽可达，直到调用方按契约弹走），分配失败在 VM
    // 侧以错误路径收敛而非返回悬垂指针。
    unsafe {
      lua_newthread(state);
    }
  }
}

impl TypeMetatable for LightUserData {
  /// # Safety
  /// Upholds the trait contract: pushes exactly one (null) light userdata onto
  /// a live state.
  unsafe fn push_representative(state: *mut LuaState) {
    // Safety: 契约保证 state 存活且有 ≥1 空位；light userdata 只按值存储
    // 指针本身（`setpvalue`），VM 从不解引用它，null + tag 0 合法且不表达
    // 任何所有权；压恰一个 lightuserdata 值。
    unsafe {
      lua_pushlightuserdatatagged(state, null_mut(), 0);
    }
  }
}

/// A do-nothing C function used as the representative value for the
/// function-type metatable slot. 仅作为 `lua_CFunction` 句柄使用（比较身份，
/// 不实际执行）；即便被调用也只返回 0，不触碰 `state`，无前置条件。
extern "C-unwind" fn noop_cfn(_state: *mut LuaState) -> c_int {
  0
}
