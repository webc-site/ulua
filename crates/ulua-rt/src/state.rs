//! The [`Lua`] handle and the shared inner state.
//!
//! ## Lifetime model (mirrors mlua's `Rc<inner> + registry-key` design)
//!
//! [`Lua`] owns the VM state as a `NonNull<LuaState>` handle (type-level
//! non-null; the FFI boundary converts). The state is wrapped in an
//! [`std::rc::Rc`] (`LuaInner`) so that long-lived handles ([`Table`], [`Function`],
//! [`LuaString`], the corresponding [`Value`] variants, userdata) can hold a
//! clone of that `Rc` and keep the state alive for as long as they exist.
//!
//! Each such handle additionally holds a **registry reference** obtained via
//! [`lua_ref`] (ulua's `lua_ref`/`lua_unref`). That keeps the underlying Lua
//! value reachable by the GC, and lets the handle re-push the value onto the
//! stack on demand. On `Drop` the handle releases its registry slot with
//! [`lua_unref`] — but only if the state is still alive (the `Rc` keeps it so).
//!
//! `Lua` is single-threaded (`Rc`, so `!Send`/`!Sync`), matching mlua's
//! non-`Send` default.
//!
//! ## The `send` feature
//!
//! Under the `send` feature (mirroring mlua) the shared interior uses
//! `XRc` = `Arc` instead of `Rc`, and `LuaInner` / `LuaRef` carry a
//! documented `unsafe impl Send`. That makes [`Lua`] and every handle `Send` so
//! the whole VM can be **moved** to another thread. It is *not* made `Sync`: the
//! VM is still single-threaded, the user must serialize all access, and only the
//! ownership *transfer* crosses threads (exactly mlua's `send` contract).

#[cfg(feature = "typecheck")]
use core::cell::RefCell;
#[cfg(feature = "async")]
use core::future::Future;
use core::{
  ffi::{c_char, c_void},
  ptr::NonNull,
};
use std::{cell::Cell, sync::Arc};

#[cfg(feature = "jit")]
use ulua_code_gen::functions::{
  is_supported::is_supported, luau_codegen_create::luau_codegen_create,
};
use ulua_common::{functions::c_str::with_c_str, set_luau_bool_flags};

#[cfg(feature = "async")]
use crate::async_support::{clear_async_state, create_async_callback};
// Re-export the GC-control types here so they live at `ulua_rt::state::{..}`,
// matching mlua's `mlua::state::{GcMode, GcIncParams, GcGenParams}` path.
pub use crate::gc::{GcGenParams, GcIncParams, GcMode};
#[cfg(feature = "serde")]
use crate::serde::clear_sentinels;
#[cfg(feature = "typecheck")]
use crate::{TypeDiagnostic, typecheck, typecheck::check_with_definitions};
use crate::{
  app_data::clear_app_data,
  buffer::{self, Buffer},
  callback::{create_callback_function, recover_wrapped_error, wrap_mut_closure},
  chunk::{Chunk, ChunkMode, ChunkSource, DEFAULT_CHUNK_NAME},
  error::{Error, Result},
  function::Function,
  interrupt::clear_interrupt,
  luau_ext::clear_vm_state,
  memory::{MemoryControls, clear_memory, clear_memory_categories},
  multi::MultiValue,
  options::{LuaOptions, StdLib},
  registry::RegHandle,
  string::{self, LuaString},
  sync::{MaybeSend, MaybeSync, NOT_SYNC, NotSync, XRc, XWeak},
  sys::*,
  table::{self, Table, number_at},
  traits::{FromLua, FromLuaMulti, IntoLua, IntoLuaMulti},
  userdata::{self, AnyUserData, UserData},
  value::{self, Integer, Number, Value},
  vector::Vector,
  vm_store::vm_key,
};

/// 内存类别 0：VM 隐式的 "main" 类别（`LuaInner` 关闭前重置用）。
/// 与 `memory.rs` 的类别 id 同域（u8），`as c_int` 留给 `lua_setmemcat` 调用点。
const MEMCAT_MAIN: u8 = 0;

/// 校验 Rust 名字可作 C 字符串（不含内部 NUL）后，
/// 经 `ulua-common` 的 `with_c_str` 门面向闭包提供瞬时 `*const c_char`（补尾
/// NUL、闭包调用期内存活），供 callee 当场复制/驻留——不散落 `CString`。
/// §10 评估（保留）：唯一消费者 `registry.rs` 走的是 ulua-vm 的 `lua_setfield` /
/// `lua_getfield`，形参即 `*const c_char`（C ABI 实现层，§10 豁免），callee 以
/// NUL 扫描经 `lua_s_new` 当场把名字驻留进 intern 表——无 Rust 原生名字形参可
/// 直传，本收口在门面收敛前维持。
/// 名字含内部 NUL 时返回 `Err`——[`Lua::set_named_registry_value`] 一类的注册表名
/// 直接 `?` 向上抛（见 `registry.rs`；chunk 名的 NUL 回退策略不同，在 `chunk.rs`
/// 内自行构造）。
pub(crate) fn with_name_cstr<R>(name: &str, f: impl FnOnce(*const c_char) -> R) -> Result<R> {
  if name.as_bytes().contains(&0) {
    return Err(Error::runtime("name contains a NUL byte"));
  }
  Ok(with_c_str(name.as_bytes(), f))
}

/// 为一次栈操作预留 `slots` 个空位；不足时返回可捕获的 `RuntimeError`
/// （而不是让后续 push 触发 VM 断言 abort）。
/// `Table` 系列原始访问、`Function::call`、`Thread::resume`、`exec_raw` 共用。
pub(crate) fn ensure_stack(state: *mut LuaState, slots: c_int) -> Result<()> {
  // Safety: 所有调用点传入的 `state` 要么是 `self.state()`（`XRc<LuaInner>`
  // 保活），要么是构造路径上刚通过 null 检查的新 state。`lua_checkstack` 对
  // 存活 state + 任意 `slots` 都有定义：返回 0 表示扩不动，非 0 表示头寸
  // 就位，本函数不越界读写、只把 c_int 结果转成 `Result`。
  if unsafe { lua_checkstack(state, slots) } == 0 {
    return Err(Error::runtime("stack overflow: not enough Lua stack space"));
  }
  Ok(())
}

/// [`ensure_stack`] 的 infallible 版：供签名固定、无法向上抛错的公开安全入口
/// （如 [`Table::raw_len`]、[`LuaString::as_bytes`]、[`Buffer::as_slice`]、
/// [`Thread::from_ref`]）使用。这些入口若栈头寸不足，其内部句柄 push 在裁剪
/// 断言后会越栈写（UB）；相比静默返回错误数据或跳过写操作，panic 是仓库既有
/// 约定（`as_raw_parts`/`write_bytes` 即以 `assert!` 收口）下唯一可观察的失败
/// 方式，故与可抛错入口共用同一道 `lua_checkstack` 闸门，只是把 `Err` 转成
/// panic。`#[track_caller]` 让 panic 定位到调用点而非本函数。
#[track_caller]
pub(crate) fn ensure_stack_or_panic(state: *mut LuaState, slots: c_int) {
  if let Err(e) = ensure_stack(state, slots) {
    panic!("{e}");
  }
}

/// 读栈深（`lua_gettop`）的 safe 门面，与 [`ensure_stack`] 同族：`unsafe` 只在此处一次
/// C-ABI 边界，各调用点不再重复「state 存活 + 只读查询」的 `// Safety` 论证。
#[inline]
pub(crate) fn stack_top(state: *mut LuaState) -> c_int {
  // Safety: 所有调用点传入的 `state` 要么是 `self.state()`（`XRc<LuaInner>` 保活）、
  // 要么是构造路径上刚过 null 检查的新 state；`lua_gettop` 只读当前栈深，不触栈不抛错。
  unsafe { lua_gettop(state) }
}

/// 把栈截断/填充到绝对深度 `top`（`lua_settop` 的收口点，等价 `lua_pop` 的通用形态）。
///
/// 调用序契约（正确性，非内存安全）：`state` 存活；`top` 是合法深度——截回不剥活寄存器、
/// 填充时目标深度在已预留头寸内。由 `collect_results_above` 等调用点维持（各自刚记录/预留
/// 对应深度）。
#[inline]
pub(crate) fn set_stack_top(state: *mut LuaState, top: c_int) {
  // Safety: `state` 存活（调用点传自己驱动的 state）；`top` 的合法性是本函数文档所述
  // 调用序前提，由 `collect_results_above`（截回搬运前记录的 `base`）等维持。
  unsafe { lua_settop(state, top) }
}

/// 弹出栈顶 `n` 个值（`lua_pop` 的收口点，即 [`set_stack_top`] 的 `top = gettop - n` 特化）。
///
/// 调用序契约（正确性，非内存安全）：`state` 存活且 `n` ≤ 当前栈深——各调用点都是刚压入
/// 对应层数随即弹回，配平栈。
#[inline]
pub(crate) fn pop_stack(state: *mut LuaState, n: c_int) {
  // Safety: `state` 存活（`self.state()`/`reference.state()`，均由 `XRc<LuaInner>` 保活）；
  // `n` 恒 ≤ 当前栈深（调用点紧邻压栈后弹回其压入量），`lua_pop` 只把 top 下移、不越界读写。
  unsafe { lua_pop(state, n) }
}

/// 向栈压入一个整数（`lua_pushinteger` 的收口点），与 [`ensure_stack`] 同族：`unsafe`
/// 只在此处一次 C-ABI 边界，各调用点不再重复「state 存活 + 预留 1 槽」的 `// Safety` 论证。
///
/// 调用序契约（正确性，非内存安全）：`state` 存活，压栈净增一层，由调用点维持栈配平。
#[cfg(feature = "async")]
#[inline]
pub(crate) fn push_int(state: *mut LuaState, n: c_int) {
  // Safety: 所有调用点传入的 `state` 均存活（self.state()/刚构造的驱动 state），
  // `lua_pushinteger` 内部 `ensure_stack` 自保 1 槽后写 top，净压一层。
  unsafe { lua_pushinteger(state, n) }
}

/// 把栈索引 `idx` 处的值在注册表登记，返回其正槽位 id（`lua_ref` 的收口点，与
/// [`ensure_stack`] 同族的 safe 门面）。
///
/// 调用序契约（正确性，非内存安全）：`state` 存活、`idx` 是有效栈槽。`lua_ref` 只读栈槽、
/// 在注册表落一份引用并返回正 id，**不弹栈**（弹出由调用方 [`pop_stack`] 或值本身生命周期决定）。
#[inline]
pub(crate) fn register_slot(state: *mut LuaState, idx: c_int) -> c_int {
  // Safety: 调用点（`register_ref`、`Clone`，及其下游全部句柄构造路径）传入存活 state 与
  // 刚压入值的有效索引；`lua_ref` 对存活 state + 有效索引登记并返回正 id，不越界读写、不动栈深。
  unsafe { lua_ref(state, idx) }
}

/// `LUA_MULTRET` 收集路径（`Function::call`、`Lua::exec_raw`、协程 resume）
/// 栈头寸不足时的统一错误文案。
pub(crate) const TOO_MANY_RESULTS_MSG: &str = "stack overflow: too many return values";

/// VM OOM 时错误对象的固定文案：`lua_pcall` 的 `LUA_ERRMEM` 与 `luau_load`
/// 的 OOM 均以它作错误对象，[`Lua::pop_error`] 据此识别内存错误。
const OOM_MSG: &str = "not enough memory";

/// 错误对象无法转成字符串时 [`Lua::pop_error`] 的退化文案（mlua 同字面量）。
const NON_STRING_ERROR_MSG: &str = "<non-string error>";

/// The reference-counted, shared interior of a [`Lua`] instance.
///
/// Held by [`Lua`] and cloned into every long-lived handle. When the last
/// `XRc<LuaInner>` is dropped, [`Drop`] closes the `LuaState`.
pub(crate) struct LuaInner {
  /// The owned VM state handle. Non-null by construction (`NonNull` encodes the
  /// invariant; the null check happens once where `lua_*` hands the pointer over).
  pub(crate) state: NonNull<LuaState>,
  /// Whether this `LuaInner` is responsible for closing the state. The
  /// trampoline builds a *borrowed* [`Lua`] around the calling thread's
  /// state and must not close it.
  owned: bool,
  /// Host type definitions accumulated via [`Lua::add_definitions`] (the
  /// `typecheck` feature), in Luau definition-file syntax. Each registration
  /// is appended separated by a newline; the whole buffer is fed to the
  /// type-checker by [`Lua::check`] / [`Chunk::check`]. Uses the crate's
  /// `RefCell` interior-mutability idiom (the VM is single-threaded).
  #[cfg(feature = "typecheck")]
  typecheck_defs: RefCell<String>,
  #[cfg(feature = "jit")]
  pub(crate) jit_enabled: Cell<bool>,
}

impl LuaInner {
  /// Build a fresh `LuaInner`, initializing every field (including the
  /// feature-gated `typecheck_defs` store). Used by all `Lua` constructors so
  /// the field set stays in one place.
  fn new(state: NonNull<LuaState>, owned: bool) -> LuaInner {
    LuaInner {
      state,
      owned,
      #[cfg(feature = "typecheck")]
      typecheck_defs: RefCell::new(String::new()),
      #[cfg(feature = "jit")]
      jit_enabled: Cell::new(false),
    }
  }
}

impl Drop for LuaInner {
  fn drop(&mut self) {
    if self.owned {
      let state = self.state.as_ptr();
      // Evict every per-VM side-table entry keyed by this state before closing
      // it, so none of them leaks one slot per state created — and so the next
      // VM that reuses this `global_State` address does not inherit them. All
      // are keyed by the still-valid state/global pointer here. (The sandbox
      // saved-globals live in the state's REGISTRY, freed by `lua_close` below;
      // here we only drop their pointer/flag bookkeeping.) These maps hold no
      // Lua handles, so the state actually reaches this Drop — that is the whole
      // point of not caching handles. See [`crate::vm_store`].
      clear_app_data(state);
      #[cfg(feature = "async")]
      clear_async_state(state);
      #[cfg(feature = "serde")]
      clear_sentinels(state);
      clear_interrupt(state);
      clear_vm_state(state);
      // The memory map is keyed by the global-state pointer and must be
      // dropped AFTER `lua_close` (the allocator `MemoryControl` handed to
      // the VM as `ud` is used throughout close to free every object), so
      // capture the key — and our control block's identity token, while the
      // entry is provably still ours — now, while the state is still valid.
      // The category table has no such tie to close, so it goes with the
      // other pre-close stores.
      // Safety: `state` 此刻仍存活（`lua_close` 尚未运行，上面的 clear_*
      // 序列也都以存活 state 为前提），`global` 非空，`vm_key` 契约成立。
      let mem_key = unsafe { vm_key(state) };
      let mem_token = MemoryControls::with(|m| m.get(&mem_key).map_or(0, |ctrl| ctrl.token));
      clear_memory_categories(mem_key);
      // Safety: `state` 存活且本 `LuaInner` 是其唯一拥有者（`owned:true`
      // 且 `Rc` 强计数归零才进到这里）——`lua_setmemcat` 只写 `activememcat`
      // 一字段，`lua_close` 释放整个 VM 并把 state 变为 dangling；此后本帧
      // 不再触碰该指针（`clear_memory` 只用已捕获的整型 key）。二者是
      // `lua_*` C ABI 对主状态的合法调用点序（close 前重置类别，close 收口），
      // 不存在并发借用——`Rc` 归零即独占。
      unsafe {
        // Reset the active memory category to "main" before closing.
        // `Lua::set_memory_category` may have left a non-main category
        // active; allocations made during teardown would otherwise be
        // accounted to it, tripping `close_state`'s debug invariant that
        // only category 0 is non-empty at shutdown.
        lua_setmemcat(state, MEMCAT_MAIN as c_int);
        lua_close(state)
      }
      // Now the allocator is no longer needed: drop its control block (only
      // if the token still identifies ours — see `clear_memory`).
      clear_memory(mem_key, mem_token);
    }
  }
}

// Under the `send` feature, allow a `Lua` (and every handle, transitively) to be
// **moved** across threads. The raw `*mut LuaState` is `!Send`/`!Sync` by
// default; these impls encode ulua-rt's documented contract — single-threaded
// *use*, only *ownership transfer* across threads, never concurrent access.
//
// `Send` is the property we actually expose. `Sync` is needed only as an
// internal obligation: `XRc<LuaInner>` is `Arc<LuaInner>` under the feature, and
// `Arc<T>: Send` requires `T: Send + Sync`. We therefore mark `LuaInner` (the
// non-public interior) `Sync`, and then keep the *public* `Lua`/handle types
// `!Sync` with a `NotSync` phantom marker (see [`NotSync`]). Net effect: the VM
// can be moved across threads but never shared/accessed concurrently — exactly
// mlua's `send` contract, minus mlua's extra `Sync` (ulua-rt stays `!Sync`).
//
// Safety: `LuaInner` 内所有可变状态（`NonNull<LuaState>` 句柄、`Registry`、`AppData`）
// 只允许随句柄 *整体移交* 线程，不允许并发访问； public 的 `Lua` 由 `NotSync`
// 标记保持 `!Sync`，故下面的 `Sync` 只为满足 `Arc<T>: Send` 的内部义务而存在，
// 不构成任何并发读写许可——违反该移交契约即数据竞争，责任在调用方。
#[cfg(feature = "send")]
unsafe impl Send for LuaInner {}
#[cfg(feature = "send")]
unsafe impl Sync for LuaInner {}

/// A handle to a Lua interpreter.
///
/// Mirrors `mlua::Lua`. Cloning produces another handle to the **same** VM
/// (the inner state is shared via `Rc`), exactly like mlua.
#[derive(Clone)]
pub struct Lua {
  pub(crate) inner: XRc<LuaInner>,
  /// Keeps `Lua` `!Sync` under the `send` feature (the VM is move-only, never
  /// shareable). A zero-sized `()` under the default build. See [`NotSync`].
  pub(crate) _not_sync: NotSync,
}

/// 创建一个 VM 并按需打开标准库，**分配失败时返回 `None`**。
///
/// `create` 是 state 的产生者（生产路径为 [`lua_l_newstate`]，测试可注入返回 null
/// 的分配器以覆盖 OOM 分支）。
///
/// 关键顺序：`create` 的返回值先经 `NonNull::new` 的非空收口（null → `None`
/// 提前返回），**之后**才允许被任何 `lua_*` 入口使用（含 [`lua_l_openlibs`]）——
/// 对空 state 调用任何 `lua_*` 入口都是空指针解引用，检查必须在使用之前。
fn build_lua(create: impl FnOnce() -> *mut LuaState, openlibs: bool) -> Option<Lua> {
  let state = NonNull::new(create())?;
  if openlibs {
    // Safety: `state` 刚过上一行的非空收口，是完整可用的新 state。
    unsafe { lua_l_openlibs(state.as_ptr()) };
  }
  Some(Lua::from_inner(XRc::new(LuaInner::new(state, true))))
}

/// OOM 创建 VM 时的统一错误文案（[`Lua::new_with`]）。
const STATE_ALLOC_MSG: &str = "failed to create Lua state: memory allocation failure";

impl Lua {
  /// Create a new Lua state with the standard library opened.
  ///
  /// Mirrors `mlua::Lua::new`.
  ///
  /// # Panics
  /// Panics with `"lua_l_newstate returned null"` if the VM cannot be allocated
  /// (the state is checked for null **before** the libraries are opened, so a
  /// failed allocation panics instead of dereferencing a null state).
  pub fn new() -> Lua {
    // ulua's v11+ bytecode needs the default Luau flags on (see the
    // umbrella crate's `eval`).
    set_luau_bool_flags(true);
    build_lua(lua_l_newstate, true).expect("lua_l_newstate returned null")
  }

  /// Create a new Lua state **without** opening the standard library.
  ///
  /// A deliberate deviation from mlua (which exposes `StdLib` flags); a
  /// minimal convenience for embedders who want a clean global table.
  ///
  /// # Panics
  /// Panics (before any use of the state) if the VM cannot be allocated; see
  /// [`Lua::new`].
  pub fn new_empty() -> Lua {
    set_luau_bool_flags(true);
    build_lua(lua_l_newstate, false).expect("lua_l_newstate returned null")
  }

  /// Create a new Lua state with the standard library opened, **without** the
  /// extra safety restrictions a safe `Lua::new` would impose.
  ///
  /// Mirrors `mlua::Lua::unsafe_new`. In Luau there is no separate set of
  /// "unsafe" base libraries (the `debug`/`ffi`/`package` distinction is a
  /// Lua-5.x concept), so this is equivalent to [`Lua::new`]; the name is
  /// kept for mlua signature parity. mlua's counterpart is `unsafe fn`
  /// because its default libraries can load native code; ulua's cannot, so
  /// this entry point needs no preconditions and is fully safe.
  pub fn unsafe_new() -> Lua {
    Lua::new()
  }

  /// Create a new Lua state opening the libraries selected by `libs`, with the
  /// behavioral `options`. Mirrors `mlua::Lua::new_with`.
  ///
  /// **DEVIATION:** ulua opens the Luau base libraries as a unit, so any
  /// non-empty `libs` opens the full standard library and [`StdLib::NONE`]
  /// opens nothing (see [`StdLib`]). `options` is recorded on the VM (currently
  /// only `catch_rust_panics` is observable).
  ///
  /// # Errors
  /// [`Error::MemoryError`] if the state itself cannot be allocated — the null
  /// return of `lua_newstate` is detected **before** `lua_l_openlibs` runs.
  pub fn new_with(libs: StdLib, options: LuaOptions) -> Result<Lua> {
    set_luau_bool_flags(true);
    let lua = build_lua(lua_l_newstate, !libs.is_none())
      .ok_or_else(|| Error::MemoryError(STATE_ALLOC_MSG.to_string()))?;
    lua.set_catch_rust_panics(options.catch_rust_panics);
    Ok(lua)
  }

  /// Enables or disables Luau native code generation (JIT).
  ///
  /// Mirrors `mlua::Lua::enable_jit`.
  #[cfg(feature = "jit")]
  pub fn enable_jit(&self, enabled: bool) -> Result<()> {
    if enabled {
      if !is_supported() {
        return Err(Error::runtime(
          "Luau native CodeGen is not supported on this platform",
        ));
      }
      // Safety: self.state() 为存活有效的 LuaState 指针
      unsafe {
        luau_codegen_create(self.state());
      }
      self.inner.jit_enabled.set(true);
    } else {
      self.inner.jit_enabled.set(false);
    }
    Ok(())
  }

  /// Enables or disables Luau native code generation (JIT).
  ///
  /// Returns an error if the crate is built without the `jit` feature.
  #[cfg(not(feature = "jit"))]
  pub fn enable_jit(&self, _enabled: bool) -> Result<()> {
    Err(Error::runtime(
      "Luau JIT support is not enabled in this build (requires feature 'jit')",
    ))
  }

  /// Returns whether Luau native code generation (JIT) is currently enabled.
  pub fn is_jit_enabled(&self) -> bool {
    #[cfg(feature = "jit")]
    {
      self.inner.jit_enabled.get()
    }
    #[cfg(not(feature = "jit"))]
    {
      false
    }
  }

  /// The raw state pointer. Internal use only — this is the crate's single FFI
  /// 收口点 accessor: wrapper handles carry the state as `NonNull`/behind `Rc`,
  /// and only call sites about to invoke a `lua_*` C-ABI entry read it out here.
  #[inline]
  pub(crate) fn state(&self) -> *mut LuaState {
    self.inner.state.as_ptr()
  }

  /// Wrap an *already-existing* state (e.g. the thread passed into a C
  /// trampoline) in a borrowed [`Lua`] that will **not** close it on drop.
  ///
  /// 内部边界封装（原 `pub(crate) unsafe fn`，去 unsafe 化后契约前移到调用点）：
  /// `state` 必须是存活的非空 `LuaState`，且比返回句柄及其所有克隆活得久。本函数
  /// 只把指针经 `NonNull` 收口存入 `LuaInner`（不解引用），`owned:false` 使句柄
  /// drop 不关 VM。全部调用点都在 VM 驱动的 C trampoline 内（`callback.rs`/
  /// `async.rs`/`interrupt.rs`）：那里的 `state` 由 VM 在受保护边界实时传入，天然
  /// 满足契约；null 输入是调用方违约，当场 panic（原语义下是后续空指针解引用
  /// UB——panic 是更响亮的等价失败）。
  pub(crate) fn from_borrowed(state: *mut LuaState) -> Lua {
    let state = NonNull::new(state).expect("borrowed LuaState must not be null");
    Lua::from_inner(XRc::new(LuaInner::new(state, false)))
  }

  /// Register a value sitting at stack index `idx` in the registry and return
  /// a [`LuaRef`] that owns the slot. Does not pop the value.
  pub(crate) fn register_ref(&self, idx: c_int) -> LuaRef {
    // `register_slot` 是 safe 门面：`idx` 的合法性由调用方承担——`pop_ref` 传 -1 且
    // 调用点栈顶必有值（先 push 后登记），其余经 `Lua::register_ref` 的句柄构造点同样
    // 刚压入对象；owning VM 由本 `Lua` 的 `XRc<LuaInner>` 保活。
    let id = register_slot(self.state(), idx);
    LuaRef {
      inner: self.inner.clone(),
      id: Cell::new(id),
    }
  }

  /// Pop the top stack value and register it, returning a [`LuaRef`].
  pub(crate) fn pop_ref(&self) -> LuaRef {
    let r = self.register_ref(-1);
    // `pop_stack` 是 safe 门面：上一行 `register_ref(-1)` 登记时不弹栈（ref 保留栈顶
    // 值），故弹的正是刚登记的槽位，栈顶非空、`state` 存活。
    pop_stack(self.state(), 1);
    r
  }
}

impl Default for Lua {
  fn default() -> Self {
    Lua::new()
  }
}

impl Lua {
  /// 从共享内部状态封装 [`Lua`]（补上 `!Sync` 标记）。全部构造点共用，
  /// 保证字段集只有这一处。
  fn from_inner(inner: XRc<LuaInner>) -> Lua {
    Lua {
      inner,
      _not_sync: NOT_SYNC,
    }
  }

  /// A non-owning, weak handle to this VM. Mirrors `mlua::Lua::weak`.
  ///
  /// The [`WeakLua`] does not keep the VM alive; it can be upgraded back to a
  /// strong [`Lua`] only while at least one strong handle still exists.
  pub fn weak(&self) -> WeakLua {
    WeakLua(XRc::downgrade(&self.inner))
  }
}

/// A weak handle to a [`Lua`] instance. Mirrors `mlua::WeakLua`.
///
/// Holds a non-owning reference to the shared VM interior; upgrade it to a
/// strong [`Lua`] with [`WeakLua::try_upgrade`] / [`WeakLua::upgrade`].
#[derive(Clone)]
pub struct WeakLua(pub(crate) XWeak<LuaInner>);

impl WeakLua {
  /// Try to obtain a strong [`Lua`] handle. Returns `None` if the VM has
  /// already been destroyed. Mirrors `mlua::WeakLua::try_upgrade`.
  pub fn try_upgrade(&self) -> Option<Lua> {
    self.0.upgrade().map(Lua::from_inner)
  }

  /// Obtain a strong [`Lua`] handle, panicking if the VM has been destroyed.
  /// Mirrors `mlua::WeakLua::upgrade`.
  pub fn upgrade(&self) -> Lua {
    // mlua 对等的文档化 panic：upgrade 失败仅代表 VM 已销毁，属公开契约（try_upgrade 为可抛错版）。
    self.try_upgrade().expect("Lua instance is destroyed")
  }
}

// ---------------------------------------------------------------------------
// Public, mlua-style construction API.
// ---------------------------------------------------------------------------

impl Lua {
  /// The globals table.
  ///
  /// Mirrors `mlua::Lua::globals`. Returns a [`Table`] handle to the global
  /// environment (the table reachable at `LUA_GLOBALSINDEX`).
  pub fn globals(&self) -> Table {
    let state = self.state();
    // 压入全局表一层（`pop_ref` 随即弹掉）：先预留头寸。
    ensure_stack_or_panic(state, 1);
    // Safety: `state` 存活且刚预留 1 层头寸；`LUA_GLOBALSINDEX` 是 Luau 的
    // 注册表伪索引，`lua_pushvalue` 对伪索引恒成功地把全局表副本压顶。
    unsafe { lua_pushvalue(state, LUA_GLOBALSINDEX) };
    // `pop_ref` 消费刚压入的一层登记引用，净栈变化为零。
    Table::from_ref(self.pop_ref())
  }

  /// Create a new, empty table.
  ///
  /// Mirrors `mlua::Lua::create_table` (infallible here, so no `Result`
  /// wrapper is strictly needed — but we also provide the `_result` variant
  /// for signature parity below).
  pub fn create_table(&self) -> Table {
    table::create_table(self)
  }

  /// Create a new, empty table with preallocated array (`narr`) and record (`nrec`) capacities.
  pub fn create_table_with_capacity(&self, narr: usize, nrec: usize) -> Table {
    table::create_table_with_capacity(self, narr, nrec)
  }

  /// `Result`-returning alias of [`Lua::create_table`] for mlua signature
  /// parity.
  pub fn create_table_result(&self) -> Result<Table> {
    Ok(self.create_table())
  }

  /// Create a Lua string from bytes/str.
  ///
  /// Mirrors `mlua::Lua::create_string`.
  pub fn create_string(&self, s: impl AsRef<[u8]>) -> LuaString {
    string::create_string(self, s.as_ref())
  }

  /// Create a table and populate it from an iterator of key/value pairs.
  ///
  /// Mirrors `mlua::Lua::create_table_from`.
  pub fn create_table_from<K, V, I>(&self, iter: I) -> Result<Table>
  where
    K: IntoLua,
    V: IntoLua,
    I: IntoIterator<Item = (K, V)>,
  {
    let t = self.create_table();
    for (k, v) in iter {
      t.raw_set(k, v)?;
    }
    Ok(t)
  }

  /// Create a sequence (1-based array) table from an iterator of values.
  ///
  /// Mirrors `mlua::Lua::create_sequence_from`.
  pub fn create_sequence_from<V, I>(&self, iter: I) -> Result<Table>
  where
    V: IntoLua,
    I: IntoIterator<Item = V>,
  {
    let t = self.create_table();
    t.fill_sequence(iter)?;
    Ok(t)
  }

  /// Run a full garbage-collection cycle.
  ///
  /// Mirrors `mlua::Lua::gc_collect` (infallible here — ulua's `lua_gc`
  /// cannot fail for `collect`).
  pub fn gc_collect(&self) -> Result<()> {
    // Safety: `state` 存活，`Collect as c_int` 是 VM 认识的完整周期 op；
    // 宿主调用点不在 GC 步进中途，收集只回收不可达对象——所有存活句柄都
    // 持注册表引用（GC 可达），不会被误收。
    unsafe { lua_gc(self.state(), LuaGcOp::Collect as c_int, 0) };
    Ok(())
  }

  /// Create a Lua function from a Rust closure.
  ///
  /// Mirrors `mlua::Lua::create_function`. The closure receives `&Lua` and
  /// the arguments converted via [`FromLuaMulti`]; its `Ok` return is
  /// converted via [`IntoLuaMulti`]. Returning `Err` (or panicking) surfaces
  /// as a catchable Lua error.
  pub fn create_function<F, A, R>(&self, func: F) -> Result<Function>
  where
    F: Fn(&Lua, A) -> Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti,
  {
    create_callback_function(self, func)
  }

  /// Create a Lua function from a Rust **mutable** closure.
  ///
  /// Mirrors `mlua::Lua::create_function_mut`. The closure is guarded by a
  /// [`std::cell::RefCell`]; a re-entrant call (the callback running
  /// Lua that calls the same callback again) surfaces as
  /// [`Error::RecursiveMutCallback`]
  /// rather than allowing mutable aliasing.
  pub fn create_function_mut<F, A, R>(&self, func: F) -> Result<Function>
  where
    F: FnMut(&Lua, A) -> Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti,
  {
    self.create_function(wrap_mut_closure!(func))
  }

  /// Create userdata wrapping a `T: UserData` value.
  ///
  /// Mirrors `mlua::Lua::create_userdata`.
  pub fn create_userdata<T: UserData + MaybeSend + MaybeSync + 'static>(
    &self,
    data: T,
  ) -> Result<AnyUserData> {
    userdata::create_userdata(self, data)
  }

  /// Create a Lua function from a Rust **async** closure (the `async`
  /// feature).
  ///
  /// Mirrors `mlua::Lua::create_async_function`. The closure receives an owned
  /// [`Lua`] and the converted arguments, and returns a `Future`. When the
  /// resulting Lua function is called, it runs on a coroutine that **yields**
  /// while the future is pending; a driver such as
  /// [`Function::call_async`](crate::Function::call_async) /
  /// [`Chunk::eval_async`](crate::Chunk::eval_async) resumes the coroutine,
  /// polls the future, and resumes it with the result when ready.
  ///
  /// The executor is provided by the caller (ulua-rt is executor-agnostic,
  /// exactly like mlua): the returned futures must be `.await`ed / polled on
  /// the caller's runtime (e.g. tokio).
  #[cfg(feature = "async")]
  #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
  pub fn create_async_function<F, A, FR, R>(&self, func: F) -> Result<Function>
  where
    F: Fn(Lua, A) -> FR + MaybeSend + 'static,
    A: FromLuaMulti,
    FR: Future<Output = Result<R>> + MaybeSend + 'static,
    R: IntoLuaMulti,
  {
    // The closure and its future stay at their concrete types; the poller's
    // `get_future`/`poll` C closures are monomorphized over `(F, A, FR, R)`.
    // Argument conversion (`A::from_lua_multi`) and the `R -> MultiValue`
    // conversion are performed inside those closures, with a conversion error
    // deferred to the first poll (see `async_support`), preserving the previous
    // boxed-future behavior.
    create_async_callback(self, func)
  }

  /// Creates and returns a Luau [buffer] object from a byte slice of data.
  ///
  /// Mirrors `mlua::Lua::create_buffer`.
  ///
  /// [buffer]: https://luau.org/library#buffer-library
  pub fn create_buffer(&self, data: impl AsRef<[u8]>) -> Result<Buffer> {
    let data = data.as_ref();
    let mut buffer = self.create_buffer_with_capacity(data.len())?;
    if !data.is_empty() {
      buffer.write_bytes(0, data);
    }
    Ok(buffer)
  }

  /// Creates and returns a Luau [buffer] object with the specified size.
  ///
  /// Size limit is 1GB. All bytes are initialized to zero. Exceeding the
  /// limit returns a `RuntimeError` carrying a `"memory allocation error"`
  /// message (matching mlua).
  ///
  /// Mirrors `mlua::Lua::create_buffer_with_capacity`.
  ///
  /// [buffer]: https://luau.org/library#buffer-library
  pub fn create_buffer_with_capacity(&self, size: usize) -> Result<Buffer> {
    buffer::create_buffer_with_capacity(self, size)
  }

  /// Creates and returns a Luau [`Vector`] value.
  ///
  /// Mirrors `mlua::Lua::create_vector`. ulua is a 3-wide vector build.
  pub fn create_vector(&self, x: f32, y: f32, z: f32) -> Vector {
    Vector::new(x, y, z)
  }

  /// Load a chunk of Lua source for execution.
  ///
  /// Mirrors `mlua::Lua::load`. Returns a [`Chunk`]; finalize with
  /// [`Chunk::exec`] / [`Chunk::eval`] / [`Chunk::into_function`].
  pub fn load(&self, source: impl AsRef<str>) -> Chunk {
    Chunk {
      lua: self.clone(),
      source: ChunkSource::Text(source.as_ref().to_string()),
      name: DEFAULT_CHUNK_NAME.to_string(),
      environment: None,
      compiler: None,
      mode: ChunkMode::Text,
    }
  }

  /// Load precompiled Luau bytecode for execution.
  ///
  /// Returns a [`Chunk`] configured to load and execute the precompiled bytecode
  /// directly without re-invoking the compiler.
  pub fn load_bytecode(&self, bytecode: impl AsRef<[u8]>) -> Chunk {
    Chunk {
      lua: self.clone(),
      source: ChunkSource::Bytecode(bytecode.as_ref().to_vec()),
      name: DEFAULT_CHUNK_NAME.to_string(),
      environment: None,
      compiler: None,
      mode: ChunkMode::Binary,
    }
  }

  /// Convert a Rust value into a single Lua [`Value`].
  ///
  /// Mirrors `mlua::Lua::pack`-ish convenience. Provided so callers can build
  /// `Value`s without importing the trait.
  pub fn pack(&self, value: impl IntoLua) -> Result<Value> {
    value.into_lua(self)
  }

  /// Convert any `FromLuaMulti` from a packed [`MultiValue`]. Mirrors
  /// `mlua::Lua::unpack_multi` (and `unpack` for the single-value case).
  pub fn unpack_multi<T: FromLuaMulti>(&self, values: MultiValue) -> Result<T> {
    T::from_lua_multi(values, self)
  }

  /// Convert a single Lua [`Value`] to a Rust value. Mirrors `mlua::Lua::unpack`.
  pub fn unpack<T: FromLua>(&self, value: Value) -> Result<T> {
    T::from_lua(value, self)
  }

  /// Coerce a [`Value`] to an integer the way Lua's `tonumber`+integer check
  /// would (`"1"` -> `Some(1)`, `"1.5"` -> `None`, a non-numeric value ->
  /// `None`). Mirrors `mlua::Lua::coerce_integer`.
  pub fn coerce_integer(&self, value: Value) -> Result<Option<Integer>> {
    // An integral, in-range float coerces to an integer; otherwise None.
    Ok(
      self
        .coerce_number_value(&value)?
        .filter(|n| value::is_exact_integer(*n))
        .map(|n| n as i64),
    )
  }

  /// Coerce a [`Value`] to a float the way Lua's `tonumber` would. Mirrors
  /// `mlua::Lua::coerce_number`.
  pub fn coerce_number(&self, value: Value) -> Result<Option<Number>> {
    self.coerce_number_value(&value)
  }

  /// Shared core of `coerce_integer` / `coerce_number`: push the value, run
  /// the VM's `tonumber`, and report whether the coercion succeeded.
  fn coerce_number_value(&self, value: &Value) -> Result<Option<Number>> {
    let state = self.state();
    // 预留 `push_value` 压入的一层（读毕弹掉）。
    ensure_stack(state, 1)?;
    // Safety: 上一行闸门保证 `push_value` 的一层头寸；其各分支要么不压、
    // 要么恰压一层后返回 `Ok`（该函数唯一 `Err` 面是嵌套句柄 push 的 VM
    // 断言路径，正常返回即栈顶有值），故压入后 -1 恒为有效索引。
    self.push_value(value)?;
    // isnum 出参经 `table::number_at` 门面收口为 `Option`（rt 既定统一形态）。
    // Safety: `state` 存活且 -1 是刚压入值的有效索引（见上），number_at 只读
    // 该槽、不动栈深。
    let coerced = unsafe { number_at(state, -1) };
    // `pop_stack` 弹掉刚压入的值（`number_at` 不动栈）恢复平衡。
    pop_stack(state, 1);
    Ok(coerced)
  }

  /// Replace the global environment with `globals`. Mirrors
  /// `mlua::Lua::set_globals`.
  ///
  /// In a sandboxed Lua state the globals table is read-only and cannot be
  /// replaced; this returns a [`Error::RuntimeError`] in that case (matching
  /// mlua / Luau).
  pub fn set_globals(&self, globals: Table) -> Result<()> {
    if self.is_sandboxed() {
      return Err(Error::runtime(
        "cannot change globals in a sandboxed Lua state",
      ));
    }
    let state = self.state();
    // 预留 globals 压入的一层（随后 `lua_replace` 弹掉）：`push_to_stack` 落
    // `lua_rawgeti` 自带栈预留，这里的 `ensure_stack` 只是把接近栈上限时的
    // 越界收敛成一个可返回的 `Err` 而非 VM abort。
    ensure_stack(state, 1)?;
    // `globals.push_to_stack()` 是 safe 封装；owning VM 存活由本句柄的
    // `XRc<LuaInner>` 保证，线程一致性沿用 move-not-share 纪律（沙箱态已在入口挡住）。
    globals.push_to_stack();
    // Safety: `lua_replace(state, LUA_GLOBALSINDEX)` 按 C API 约定消费栈顶值
    // （上一行刚压入的 globals 表）写入全局表伪索引，压/消配平；它搬运的是
    // 已压入本栈的 GC 值，不产生悬垂读。
    unsafe { lua_replace(state, LUA_GLOBALSINDEX) };
    Ok(())
  }

  /// Build a stack traceback string for this VM. Mirrors `mlua::Lua::traceback`.
  ///
  /// `msg`, if present, is prepended to the traceback; `level` selects the
  /// starting stack level. The returned [`LuaString`] holds the traceback as
  /// produced by `luaL_traceback`.
  pub fn traceback(&self, msg: Option<&str>, level: usize) -> Result<LuaString> {
    let state = self.state();
    // 预留 `luaL_traceback` 压入结果字符串的一层（`pop_ref` 随即弹掉）。
    ensure_stack(state, 1)?;
    // Safety: `state` 存活且刚预留 1 层；`lua_l_traceback(L, L2=state, msg,
    // level)` 的 `msg` 是 `Option<&str>`，Some 时字节切片在本帧存活且
    // traceback 当场格式化成字符串（不寄存指针），None 走跳过分支；两个
    // state 实参同属存活 VM。该调用恒压一个字符串结果。
    unsafe { lua_l_traceback(state, state, msg, level as c_int) };
    // `pop_ref` 消费 traceback 压出的结果字符串。
    Ok(LuaString::from_ref(self.pop_ref()))
  }

  /// [`Lua::traceback`] 的内部错误路径版本：返回 `String`，且**任何失败都退化成
  /// 空串**（错误处理路径上不能再抛出「取回溯失败」这种次生错误；结果串的
  /// 栈位由 [`Lua::traceback`] 内建的 [`ensure_stack`] 预留）。
  fn traceback_at_level(&self, level: usize) -> String {
    match self.traceback(None, level) {
      Ok(text) => text.to_string_lossy(),
      Err(_) => String::new(),
    }
  }
}

// ---------------------------------------------------------------------------
// Static type-checking (the `typecheck` feature).
//
// ulua ships Luau's static type checker, so — unlike mlua — a script can be
// type-checked against the host surface *before* it runs. The host surface is
// described in Luau definition-file syntax and accumulated on the `Lua` via
// `add_definitions`; `check` / `Chunk::check` then validate source against it.
// ---------------------------------------------------------------------------
#[cfg(feature = "typecheck")]
#[cfg_attr(docsrs, doc(cfg(feature = "typecheck")))]
impl Lua {
  /// Register host type `definitions` (Luau definition-file syntax) so later
  /// [`Lua::check`] / [`Chunk::check`] calls type-check against them.
  ///
  /// `definitions` describes the host-provided globals — the Rust functions,
  /// values, and userdata you expose to the runtime (e.g. via
  /// [`Lua::create_function`] / [`UserData`]):
  ///
  /// ```text
  /// declare function add(a: number, b: number): number
  /// declare config: { name: string, retries: number }
  /// ```
  ///
  /// The definitions are validated before being recorded: if they are
  /// malformed, this returns [`Error::TypeError`]
  /// carrying the (`in_definitions`) diagnostics and records nothing. On
  /// success they are appended to this VM's accumulated definitions.
  pub fn add_definitions(&self, defs: &str) -> Result<()> {
    // Validate the new definitions in isolation by checking a trivial body.
    if let Err(diagnostics) = check_with_definitions("return nil", defs) {
      // Only the definition-side diagnostics are this call's fault; a
      // type error in the trivial body would be ours, not the caller's.
      let def_errors: Vec<TypeDiagnostic> = diagnostics
        .into_iter()
        .filter(|d| d.in_definitions)
        .collect();
      if !def_errors.is_empty() {
        return Err(Error::TypeError(def_errors));
      }
    }
    // Append, newline-separated, to the accumulated definitions.
    let mut store = self.inner.typecheck_defs.borrow_mut();
    if !store.is_empty() {
      store.push('\n');
    }
    store.push_str(defs);
    Ok(())
  }

  /// Type-check `source` against this VM's accumulated host definitions.
  ///
  /// Returns `Ok(())` if the source type-checks clean, or
  /// [`Error::TypeError`] carrying the structured
  /// diagnostics otherwise.
  ///
  /// The Luau VM is dynamically typed, so this is **advisory**: a script that
  /// fails the check can still be run (`exec`/`eval`). The value is catching
  /// host-API misuse statically, before running untrusted or generated code.
  pub fn check(&self, source: &str) -> Result<()> {
    let defs = self.inner.typecheck_defs.borrow();
    let result = if defs.is_empty() {
      typecheck::check(source)
    } else {
      typecheck::check_with_definitions(source, &defs)
    };
    result.map_err(Error::TypeError)
  }

  /// Type-check `source` against this VM's accumulated host definitions **plus**
  /// the extra `defs` (for a one-off check that does not persist `defs`).
  ///
  /// Same mapping as [`Lua::check`]: `Ok(())` when clean, otherwise
  /// [`Error::TypeError`].
  pub fn check_with_definitions(&self, source: &str, defs: &str) -> Result<()> {
    let accumulated = self.inner.typecheck_defs.borrow();
    let combined = if accumulated.is_empty() {
      defs.to_string()
    } else {
      format!("{accumulated}\n{defs}")
    };
    typecheck::check_with_definitions(source, &combined).map_err(Error::TypeError)
  }
}

/// An owned registry reference to a Lua value.
///
/// Keeps both the value reachable (registry slot) and the VM alive (the cloned
/// `XRc<LuaInner>`). On drop it releases the slot via [`lua_unref`].
pub(crate) struct LuaRef {
  inner: XRc<LuaInner>,
  id: Cell<c_int>,
}

// `LuaRef` is shared behind `XRc<LuaRef>` (`Arc<LuaRef>` under the feature) by
// every handle, so it must be `Send + Sync` for the handles to be `Send`. The
// `Cell<c_int>` slot is only ever mutated on the owning thread (the move-only
// contract); marking `LuaRef` `Sync` is sound under that contract. Handles stay
// `!Sync` via their own `NotSync` markers.
//
// Safety: 沿用 `LuaInner` 的同一条移交契约——`id` 只在持有者线程上变更，
// `XRc<LuaRef>` 的跨线程共享仅为指针复制，不构成对同一 `LuaRef` 的并发读写。
#[cfg(feature = "send")]
unsafe impl Send for LuaRef {}
#[cfg(feature = "send")]
unsafe impl Sync for LuaRef {}

impl LuaRef {
  /// The owning [`Lua`] handle (a fresh borrow sharing the same inner state).
  pub(crate) fn lua(&self) -> Lua {
    Lua::from_inner(self.inner.clone())
  }

  /// The raw state pointer this ref belongs to.
  #[inline]
  pub(crate) fn state(&self) -> *mut LuaState {
    self.inner.state.as_ptr()
  }

  /// Push the referenced value, read `lua_topointer`, pop. Shared by every
  /// handle's `to_pointer` (`Table` / `Function` / `LuaString` / `Buffer` /
  /// `Thread` / `AnyUserData`).
  pub(crate) fn to_pointer(&self) -> *const c_void {
    // `push` 不自动扩容：先预留本函数压入的一层（读毕即弹）。
    ensure_stack_or_panic(self.state(), 1);
    // `with_reference_pushed` 是 safe 门面；其给出的 `lua.state()` 只在 `lua_topointer`
    // 这一处 FFI 收口点读出——对任意值槽返回对象地址或 null，不移动值、不改栈深，
    // 读出裸指针后不再有栈依赖。
    with_reference_pushed(self, |lua, idx| unsafe { lua_topointer(lua.state(), idx) })
  }

  /// The registry id. (Retained for internal diagnostics; handle identity is
  /// established via `lua_topointer`, not the registry slot id.)
  #[inline]
  pub(crate) fn id(&self) -> c_int {
    self.id.get()
  }

  /// 写回槽位 id（`replace_registry_value` 原地换槽用；`Cell` 保证共享
  /// 句柄同见新值）。
  #[inline]
  pub(crate) fn set_id(&self, id: c_int) {
    self.id.set(id);
  }

  /// Push the referenced value back onto the stack.
  pub(crate) fn push(&self) {
    // The registry table lives at LUA_REGISTRYINDEX; `lua_ref` stores
    // values keyed by their integer id, so a `rawgeti` on the registry
    // recovers them. ulua exposes this through getfield on the registry
    // via the same mechanism `lua_getref` uses in upstream Luau:
    // `lua_rawgeti(l, LUA_REGISTRYINDEX, id)`.
    // Safety: `self.inner` 的 `XRc<LuaInner>` 保证调用时 state 存活（本方法
    // 只能在 `&self` 期间运行，而 `LuaRef::drop` 先于 inner 关闭）；id 是
    // `lua_ref` 登记的正槽位（`Drop` 才 unref），LUA_REGISTRYINDEX 为 VM
    // 伪索引常量。栈头寸：VM 的 `lua_rawgeti` 自带 `ensure_stack(l, 1)`
    // 自保，无需调用方预留。
    unsafe {
      lua_rawgeti(self.state(), LUA_REGISTRYINDEX, self.id.get());
    }
  }
}

/// 把 `reference` 锚定的注册表引用值压栈，以**存活 VM 句柄 + 该值的绝对索引**跑
/// `f`，随后弹回这一层。
///
/// 「push 登记引用 → 以绝对索引读 → pop」同构样板的公共门面，使栈配对契约只在此处
/// 出现一次。[`LuaRef::to_pointer`] 及 `userdata.rs` 的 `AnyUserData::cell`/
/// `type_id`/`recover_cell`/scope 中和闭包皆直接引本门面；`table.rs` 的
/// [`crate::table::Table::with_pushed`] 是本门面的安全薄适配（分工见其文档注释）。
///
/// 闭包拿到的上下文是带 `&self` 生命周期的 [`Lua`] 句柄（§2：上下文裸指针 →
/// 带生命周期引用）——句柄背后即 `reference` 的 `XRc<LuaInner>`，VM 存活由类型
/// 系统担保；闭包只在即将调用 `lua_*` C ABI 入口的瞬间经 [`Lua::state`] 收口点
/// 读出裸指针，各自用带 `// Safety` 注释的最小 `unsafe` 块完成调用。
///
/// 栈配对是**正确性**约定（非内存安全）：`f` 不得改变栈深（只读该槽、最多经绝对
/// 索引 `idx` 消费），否则收尾的 `lua_pop` 弹错槽位。`f` 若返回指向被引用对象
/// 内联数据的引用，其有效性由注册表引用钉住对象 + Luau GC 不移动对象保证，与弹出
/// 后的栈槽位无关（同 [`LuaRef::push`] 各 safe 门面的既有论证）。
pub(crate) fn with_reference_pushed<R>(reference: &LuaRef, f: impl FnOnce(&Lua, c_int) -> R) -> R {
  let state = reference.state();
  // `reference.push()`（safe fn）落 VM 侧 `lua_rawgeti`：自带一层栈位预留，注册表
  // id 在 `Drop::lua_unref` 前恒指实槽位，压入的必是登记时的原值。
  reference.push();
  // `stack_top`/`pop_stack` 是 safe 门面：`reference.push()` 净压登记值一层，`stack_top`
  // 读回的正是该值的绝对索引；`f` 按约定不动栈深，收尾 `pop_stack(state, 1)` 弹回这一层，
  // 净栈变化为零。`state` 全程存活（`&reference` 借用期）。
  let idx = stack_top(state);
  let out = f(&reference.lua(), idx);
  pop_stack(state, 1);
  out
}

impl Clone for LuaRef {
  fn clone(&self) -> Self {
    // 重压值并在当前栈顶登记新注册表槽位：每个克隆持有独立槽位。
    // 直接登记（而非借道临时 Lua 句柄）省去两次引用计数跳变。
    let state = self.state();
    // `push` 不自动扩容：先预留重压的一层（登记后即弹）。
    ensure_stack_or_panic(state, 1);
    self.push();
    // 直接经 `register_slot` 登记（而非借道临时 `Lua`/`pop_ref` 句柄）省去两次引用计数
    // 跳变：上两行刚把有效值压到栈顶，登记该槽位返回新正 id，紧随 `pop_stack` 弹掉这一层
    // （净栈变化为零）。`Drop` 对新 id 做 unref，克隆句柄各持独立槽位互不干扰。
    let id = register_slot(state, -1);
    pop_stack(state, 1);
    LuaRef {
      inner: self.inner.clone(),
      id: Cell::new(id),
    }
  }
}

impl Drop for LuaRef {
  fn drop(&mut self) {
    let id = self.id.get();
    // Only unref live, real slots. (`LuaInner::state` is `NonNull` — the
    // never-null guard it used to need is now encoded in the field type.)
    if id > 0 {
      // Safety: 此刻 `LuaInner` 尚未 drop（本 `Drop` 持着它唯一的
      // `XRc<LuaInner>` 强引用，`lua_close` 在 inner 的 drop 里、之后才发生），
      // 故 state 存活；守卫把掉出正 id 域的值挡在 unref 之外，
      // 进入分支的 id 必是 `lua_ref` 返回且尚未释放的本句柄槽位。
      unsafe { lua_unref(self.inner.state.as_ptr(), id) };
    }
  }
}

impl Lua {
  /// Convenience: convert a top-of-stack value (at `idx`) into a [`Value`],
  /// taking a registry ref for reference types. Does not pop.
  pub(crate) fn value_from_stack(&self, idx: c_int) -> Result<Value> {
    value::value_from_stack(self, idx)
  }

  /// Push a [`Value`] onto the stack.
  pub(crate) fn push_value(&self, value: &Value) -> Result<()> {
    value::push_value(self, value)
  }

  /// Metatable-aware `tostring` of a [`Value`] (honors `__tostring`),
  /// mirroring Lua's `tostring`/`luaL_tolstring`.
  pub(crate) fn value_to_string(&self, value: &Value) -> Result<String> {
    let state = self.state();
    // 预留值一层 + `luaL_tolstring` 结果一层（`lua_pop(state, 2)` 收口）。
    ensure_stack(state, 2)?;
    self.push_value(value)?;
    // Safety: 上一行 `push_value` 成功即栈顶有值，-1 是有效索引；
    // `lua_l_tolstring_ref` 对有效索引恒转换并按 `__tostring` 语义压结果串、以
    // 切片带出全字节（长度即切片长，内嵌 NUL 不截断）。
    let out = unsafe { lua_l_tolstring_ref(state, -1) }
      .map(|s| String::from_utf8_lossy(s).into_owned())
      .unwrap_or_default();
    // `pop_stack` 精确弹回值 + luaL_tolstring 结果两层（luaL_tolstring 会把结果串压栈）。
    pop_stack(state, 2);
    Ok(out)
  }

  /// Map a `lua_pcall`/`luau_load` status code plus the error object on the
  /// stack into an [`Error`]. Assumes a non-zero status and that the error
  /// object is on top of the stack; pops it.
  pub(crate) fn pop_error(&self, status: c_int) -> Error {
    let state = self.state();
    // Safety: 函数文档即栈前提——非零 status 的 `lua_pcall`/`luau_load` 失败
    // 后错误对象必在栈顶，故 -1 是有效索引，`recover_wrapped_error` 只读栈
    // 不越界；`state` 全程存活（&self 的 XRc 链）。
    // First, see if the error object is one of our *structured* error
    // userdata (raised for scope-destruction errors). If so, recover the
    // original `Error` and wrap it in `CallbackError`, mirroring mlua.
    let wrapped = unsafe { recover_wrapped_error(state, -1) };
    if let Some(cause) = wrapped {
      // 栈回溯要在 `lua_pop` **之前**取：`luaL_traceback` 走的就是此刻
      // 这份调用栈（mlua 的 `pop_error` 同样在弹出错误对象前取）。
      // 取不到（栈位不足等）时退化成空串，与旧行为一致。
      let traceback = self.traceback_at_level(1);
      // `pop_stack` 弹回栈顶错误对象（`traceback_at_level` 净栈变化为零）这一层。
      pop_stack(state, 1);
      return Error::CallbackError {
        traceback,
        cause: Arc::new(cause),
      };
    }
    // Otherwise, fall back to the flat string error path.
    // Safety: `state` 存活（&self 的 XRc 链）；-1 仍是有效索引（上一分支未
    // 弹栈）；`lua_tolstring_ref` 以切片带出栈顶串的全部字节。
    let msg = unsafe { lua_tolstring_ref(state, -1) }
      .map(|s| String::from_utf8_lossy(s).into_owned())
      // `None`（非字符串错误对象，旧 null 指针）退化为 [`NON_STRING_ERROR_MSG`]。
      .unwrap_or_else(|| NON_STRING_ERROR_MSG.to_string());
    // Safety: 弹掉的正是栈顶错误对象（`lua_tolstring` 若做了转换，替换后的
    // 字符串仍在 -1），栈平衡不变。
    unsafe { lua_pop(state, 1) };
    // `LUA_ERRMEM` (status 4) is an out-of-memory error (the VM set the
    // error object to "not enough memory"); surface it as `MemoryError`
    // so `set_memory_limit` callers can match it, mirroring mlua.
    // `luau_load` reports OOM with a generic non-zero rc but the same
    // "not enough memory" message, so we also detect it by message.
    if status == LuaStatus::ErrMem as c_int || msg == OOM_MSG {
      return Error::MemoryError(msg);
    }
    Error::RuntimeError(msg)
  }

  /// Collect every stack value above `base` into a [`MultiValue`], then
  /// truncate the stack back to `base`. Shared by `Function::call`,
  /// `Lua::exec_raw`, and the coroutine-resume paths — all of which collect on
  /// **this** handle's state (协程路径先把结果 xmove 回 parent 再收集)，故栈
  /// 句柄不再作为参数散传，统一取自 `self.state()`。
  ///
  /// 收集前统一预留头寸：`value_from_stack` 会先把引用型结果复制（`lua_pushvalue`）
  /// 到栈顶再弹出收进注册表引用，而 `LUA_MULTRET` 调用后结果可能已把 C 帧正好填到
  /// `ci->top`（LUA_MINSTACK），那次复制 push 会越出帧界——即 `lua_pushvalue` 的
  /// `api_incr_top` 断言（fuzzer 发现：`local t={a=1}; return <约 20 个含 t 的值>`）。
  /// 预留失败时截回 `base` 并报 [`TOO_MANY_RESULTS_MSG`]。
  pub(crate) fn collect_results_above(&self, base: c_int) -> Result<MultiValue> {
    let state = self.state();
    // Safety: `state` 存活（调用方传自己驱动的 state），`lua_checkstack` 只
    // 报告头寸：返回 0 表示扩不动，非 0 表示两层余量就位。
    if unsafe { lua_checkstack(state, 2) } == 0 {
      // 头寸不足：`set_stack_top` 把 MULTRET 结果区截回调用方记录的 `base`，不留残值。
      set_stack_top(state, base);
      return Err(Error::runtime(TOO_MANY_RESULTS_MSG));
    }
    // `stack_top` 是 safe 只读门面（`state` 存活，只读栈深）。成功路径上
    // `value_from_stack` 每次 pushvalue+pop_ref 净零、恒有两层余量覆盖最坏引用登记；
    // `base+1..=top` 即本次 MULTRET 结果区，索引全部有效。
    let top = stack_top(state);
    let nresults = top - base;
    let mut results = MultiValue::with_capacity(nresults.max(0) as usize);
    // 栈位区间 base+1..=top 迭代，等价原 `for i in 0..nresults`
    for index in base + 1..=top {
      match self.value_from_stack(index) {
        Ok(v) => results.push_back(v),
        // 失败路径同样把栈截断回 base，避免泄漏中间值。
        Err(e) => {
          set_stack_top(state, base);
          return Err(e);
        }
      }
    }
    // 三条出口（正常/收集失败/头寸不足）都以 `set_stack_top(state, base)` 收口，栈平衡不变。
    set_stack_top(state, base);
    Ok(results)
  }
}

// §8：测私有函数 `build_lua` 的 null-state 前置检查；ulua-rt 不暴露分配器注入口
// （`luaL_newstate` 无参数），pub API 无从构造 null state，迁 tests/ 需泄 pub，保留 src。
// 可观测的前提（失败分配器 → null）与两条 pub 构造路径见 `tests/state_alloc_failure.rs`。
#[cfg(test)]
mod tests {
  use core::{ffi::c_void, ptr::null_mut};
  use std::panic::catch_unwind;

  use ulua_vm::{functions::lua_newstate::lua_newstate, records::lua_state::LuaState};

  use super::build_lua;

  /// 一个总是分配失败的 VM 分配器（模拟 OOM）：任何请求都返回 null（VM 把
  /// null 同时当作「释放完成」与「分配失败」，故无需区分 `nsize == 0`）。
  /// 仅作 `lua_newstate` 的 `lua_Alloc` 回调：签名与 C 侧一致，忽略全部入参
  /// （含 `ud`）且无内部状态，恒返回 null——VM 契约允许分配器返回 null 表示
  /// 失败，因此任何调用都安全，无前置条件。
  extern "C-unwind" fn failing_alloc(
    _ud: *mut c_void,
    _ptr: *mut u8,
    _osize: usize,
    _nsize: usize,
  ) -> *mut u8 {
    null_mut()
  }

  /// R5：null 检查必须先于 `lua_l_openlibs`——注入 null 时只得到 `None`/panic，
  /// 不会把空 state 交给 openlibs（那是空指针解引用）。
  #[test]
  fn build_lua_rejects_null_state_before_openlibs() {
    // 1) 直接返回 null 的 create：走 openlibs 分支也只得到 None。
    assert!(build_lua(null_mut::<LuaState>, true).is_none());
    assert!(build_lua(null_mut::<LuaState>, false).is_none());

    // 2) 真实 OOM 路径（failing allocator）同样返回 None。
    // Safety: `lua_newstate` 对分配器无前置要求（首个请求即分配 global_State，
    // `failing_alloc` 恒 null → 函数返回 null state，不解引用）；`null_mut()`
    // 作 ud 与 C 参考一致（该分配器忽略 ud）。返回值先经 `build_lua` 的
    // null 检查，null 时直接 `None`，任何 `lua_*` 入口都不会拿到空 state。
    assert!(
      build_lua(
        || unsafe { lua_newstate(Some(failing_alloc), null_mut()) },
        true
      )
      .is_none()
    );

    // 3) `Lua::new` 的失败形态是「state 为空」panic，而不是 openlibs 崩溃。
    let err = match catch_unwind(|| {
      build_lua(null_mut::<LuaState>, true).expect("lua_l_newstate returned null")
    }) {
      Err(err) => err,
      Ok(_) => panic!("a null state must panic instead of being opened"),
    };
    let msg = err
      .downcast_ref::<String>()
      .cloned()
      .or_else(|| err.downcast_ref::<&str>().map(|s| (*s).to_string()))
      .unwrap_or_default();
    assert!(
      msg.contains("null"),
      "panic must report the null state, got {msg:?}"
    );
  }
}
