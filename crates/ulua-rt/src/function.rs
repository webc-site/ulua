//! The [`Function`] handle. Mirrors `mlua::Function`.

#[cfg(feature = "async")]
use core::future::Future;
use core::{
  fmt::{self, Formatter},
  marker::PhantomData,
  mem::zeroed,
  result::Result as StdResult,
};

use ulua_common::functions::c_str::cstr_bytes;

#[cfg(feature = "async")]
use crate::async_support::{
  AsyncThread, LuaNativeAsyncFn, WrappedAsync, register_implicit_thread, unregister_implicit_thread,
};
use crate::{
  debug::debug_cstr,
  error::{ExternalError, Result},
  multi::MultiValue,
  registry::RegHandle,
  state::{Lua, LuaRef, ensure_stack},
  sync::{MaybeSend, NOT_SYNC, NotSync, XRc},
  sys::*,
  table::Table,
  traits::{FromLuaMulti, IntoLua, IntoLuaMulti},
  value::Value,
};

/// `lua_getinfo` 选项串：仅 `s`（函数类型），供 `is_lua_closure` 判定用。
/// 静态 NUL 结尾字节串，收口点交给 `*const c_char` 契约 API。
const GETINFO_S: &[u8] = b"s\0";
/// `lua_getinfo` 选项串：`n`（名字）+ `s`（源/类型）+ `a`（参数）+ `u`（upvalue）。
/// 供 `Function::info` 用，静态 NUL 结尾字节串。
const GETINFO_NSAU: &[u8] = b"nsau\0";

/// A handle to a callable Lua value (a Lua closure or a Rust function).
///
/// Mirrors `mlua::Function`. Under the `send` feature it is `Send` but never
/// `Sync` — see `crate::sync::NotSync`.
#[derive(Clone)]
pub struct Function {
  pub(crate) reference: XRc<LuaRef>,
  pub(crate) _not_sync: NotSync,
}

impl Function {
  pub(crate) fn from_ref(reference: LuaRef) -> Function {
    Function {
      reference: XRc::new(reference),
      _not_sync: NOT_SYNC,
    }
  }

  /// The owning [`Lua`].
  pub fn lua(&self) -> Lua {
    self.reference.lua()
  }

  /// Call the function with `args`, converting the results to `R`.
  ///
  /// Mirrors `mlua::Function::call`. Runs under `lua_pcall`, so a Lua runtime
  /// error (or a Rust callback returning `Err`) becomes `Err(Error)` rather
  /// than unwinding.
  pub fn call<R: FromLuaMulti>(&self, args: impl IntoLuaMulti) -> Result<R> {
    let lua = self.lua();
    let state = lua.state();
    let args: MultiValue = args.into_lua_multi(&lua)?;
    let nargs = args.len() as c_int;
    // Guard against pushing more values than the Lua stack can hold:
    // an unprotected overflow would abort the VM. We need room for the
    // function + all arguments (+1 slack for the call machinery).
    // `ensure_stack`（safe fn，只报告头寸）预留函数 1 层 + 参数 nargs 层 + 1 层
    // 调用机构余量；它只可能扩容栈、不改变当前栈深，故随后记录的 `base` 仍是
    // 压入前的合法深度。
    ensure_stack(state, nargs.saturating_add(2))?;
    // Safety: `state` 存活（句柄 `XRc<LuaInner>`），`lua_gettop` 只读当前栈深。
    let base = unsafe { lua_gettop(state) };
    // `reference.push()`（safe fn）的注册表 id 在 `Drop::lua_unref` 前恒指实槽位，
    // VM 侧 `lua_rawgeti` 自带栈预留；逐层 `push_value` 满栈时以 `?` 提前返回。
    self.reference.push();
    for v in &args {
      lua.push_value(v)?;
    }
    // Safety: `state` 存活且栈布局为 [.., func, args..]，`nargs` 与实际压入的
    // 参数数一致（同一次迭代）。`lua_pcall` 在受保护帧内执行：脚本错误/回调 Err
    // 一律折成非零 status 并把错误对象留在栈顶（正是 `pop_error` 的前提），不会
    // 裸展开到本帧；-1 即 LUA_MULTRET，保留全部结果。
    let status = unsafe { lua_pcall(state, nargs, -1, 0) };
    if status != 0 {
      return Err(lua.pop_error(status));
    }
    // Collect every value pushed above `base` as the results. The multi-ret
    // 复制头寸检查与不足时的截断由 `collect_results_above` 统一负责；
    // `from_lua_multi` 只做转换。
    let results = lua.collect_results_above(base)?;
    R::from_lua_multi(results, &lua)
  }

  /// Return a new function that, when called, prepends `args` to its own
  /// arguments and forwards to `self`.
  ///
  /// Mirrors `mlua::Function::bind`. Implemented as a Rust closure that
  /// captures the bound prefix and the target function.
  #[cfg(not(feature = "async"))]
  pub fn bind(&self, args: impl IntoLuaMulti) -> Result<Function> {
    let lua = self.lua();
    let bound: MultiValue = args.into_lua_multi(&lua)?;
    let target = self.clone();
    let bound_vec: Vec<Value> = bound.into_vec();
    lua.create_function(move |_, extra: MultiValue| {
      target.call::<MultiValue>(concat_bound(&bound_vec, extra))
    })
  }

  /// Return a new function that, when called, prepends `args` to its own
  /// arguments and forwards to `self`.
  ///
  /// Mirrors `mlua::Function::bind`. Under the `async` feature this is built as
  /// a **pure-Lua closure** (`function(...) return func(prepend(...)) end`)
  /// rather than a Rust trampoline, so the forwarded call is a Lua-level call
  /// and remains **yield-transparent**: a bound async function can still yield
  /// while its future is pending. (The `prepend` helper only rearranges
  /// arguments and returns, so it never yields across a C boundary.) Behavior
  /// is identical to the non-async implementation for ordinary functions.
  #[cfg(feature = "async")]
  pub fn bind(&self, args: impl IntoLuaMulti) -> Result<Function> {
    let lua = self.lua();
    let bound: MultiValue = args.into_lua_multi(&lua)?;
    let bound_vec: Vec<Value> = bound.into_vec();

    // `prepend(...)` returns the bound prefix followed by the call args.
    let prepend =
      lua.create_function(move |_, extra: MultiValue| Ok(concat_bound(&bound_vec, extra)))?;

    // Build the wrapper closure in Lua so the inner `func(...)` is a Lua
    // call (yield-transparent), capturing `func` and `prepend` as upvalues.
    let builder: Function = lua
      .load(
        r#"
                local func, prepend = ...
                return function(...)
                    return func(prepend(...))
                end
                "#,
      )
      .set_name("__ulua_bind")
      .into_function()?;
    builder.call::<Function>((self.clone(), prepend))
  }

  /// Call the function asynchronously: run it on a fresh coroutine and drive
  /// that coroutine to completion as a Rust [`Future`](std::future::Future).
  ///
  /// Mirrors `mlua::Function::call_async`. Works for both async functions
  /// (created via [`Lua::create_async_function`](crate::Lua::create_async_function))
  /// — which yield while their inner future is pending — and ordinary
  /// functions (which simply run to completion). Calling an async function
  /// with the *synchronous* [`Function::call`] instead raises a runtime error,
  /// matching mlua.
  #[cfg(feature = "async")]
  #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
  pub fn call_async<R>(&self, args: impl IntoLuaMulti) -> impl Future<Output = Result<R>>
  where
    R: FromLuaMulti,
  {
    let lua = self.lua();
    // Build the driver eagerly so argument-conversion / thread-creation
    // errors surface when awaited (wrapped in a ready future).
    let setup: Result<AsyncThread<R>> = (|| {
      let thread = lua.create_thread(self.clone())?;
      // The coroutine is *implicit* (created by `call_async`): register it
      // so `Lua::current_thread` running on it resolves to the owner (the
      // thread that issued this call). Mirrors mlua's thread-ownership map.
      register_implicit_thread(thread.state(), lua.state());
      // 注册后任何失败都必须撤销登记：否则 ownership 表项泄漏到
      // `LuaInner::drop`，期间同地址新协程会被 `current_thread` 误判。
      let co_state = thread.state();
      let setup_result = thread.into_async(args);
      match setup_result {
        Ok(mut th) => {
          th.set_implicit(true);
          Ok(th)
        }
        Err(e) => {
          unregister_implicit_thread(co_state);
          Err(e)
        }
      }
    })();
    async move { setup?.await }
  }

  /// Wrap a Rust async function/closure as a value convertible into a Lua
  /// function.
  ///
  /// Mirrors `mlua::Function::wrap_async`. Unlike
  /// [`Lua::create_async_function`](crate::Lua::create_async_function) the
  /// closure does not receive the [`Lua`] and its arity is free (0, 1, … args
  /// mapped from the Lua call arguments). The returned value is
  /// [`IntoLua`](crate::IntoLua) so it can be stored directly (e.g.
  /// `globals().set("f", Function::wrap_async(..))`). A returned `Err` is
  /// raised as a Lua error.
  #[cfg(feature = "async")]
  #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
  pub fn wrap_async<F, A, R, E>(func: F) -> impl IntoLua
  where
    F: LuaNativeAsyncFn<A, Output = StdResult<R, E>> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti + 'static,
    E: ExternalError + 'static,
  {
    WrappedAsync::new(move |_lua: Lua, a: A| {
      let fut = func.call(a);
      async move { fut.await.map_err(ExternalError::into_lua_err) }
    })
  }

  /// Like [`Function::wrap_async`] but the closure's output is passed through
  /// to Lua as-is (a returned `Result` becomes an `(ok, err)`-style multi
  /// value rather than being raised).
  ///
  /// Mirrors `mlua::Function::wrap_raw_async`.
  #[cfg(feature = "async")]
  #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
  pub fn wrap_raw_async<F, A>(func: F) -> impl IntoLua
  where
    F: LuaNativeAsyncFn<A> + MaybeSend + 'static,
    F::Output: IntoLuaMulti + 'static,
    A: FromLuaMulti,
  {
    WrappedAsync::new(move |_lua: Lua, a: A| {
      let fut = func.call(a);
      async move { Ok(fut.await) }
    })
  }

  /// Wrap a plain Rust closure as a value convertible into a Lua function.
  ///
  /// Mirrors `mlua::Function::wrap`. Unlike
  /// [`Lua::create_function`](crate::Lua::create_function), the closure does
  /// **not** receive the [`Lua`] and its arity is free (`||`, `|a|`, `|a, b|`,
  /// … mapped from the Lua call arguments). The returned value is
  /// [`IntoLua`](crate::IntoLua) so it can be stored directly (e.g.
  /// `table.set("f", Function::wrap(|a, b| Ok::<_, Error>(a + b)))`). A
  /// returned `Err` is raised as a Lua error.
  pub fn wrap<F, A, R, E>(func: F) -> impl IntoLua
  where
    F: LuaNativeFn<A, Output = StdResult<R, E>> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti,
    E: ExternalError,
  {
    WrappedFunction {
      func,
      _marker: PhantomData,
    }
  }

  /// A raw pointer identifying this function (for identity comparison).
  /// Mirrors `mlua::Function::to_pointer`.
  pub fn to_pointer(&self) -> *const c_void {
    self.reference.to_pointer()
  }

  /// The function's environment table (its globals), or `None` for a Rust
  /// (C) function. Mirrors `mlua::Function::environment`.
  pub fn environment(&self) -> Option<Table> {
    let lua = self.lua();
    let state = lua.state();
    // Safety: `state` 存活（句柄链 `XRc<LuaInner>`）；`lua_gettop` 只读当前栈深。
    // 此刻记录的 `base` 是本函数压入前的深度，末尾据此截回，净栈变化为零。
    let base = unsafe { lua_gettop(state) };
    // `reference.push()`（safe fn）落 VM 侧 `lua_rawgeti` 自带栈预留、注册表 id 指
    // 实槽位；压入后 -1 即本函数值。
    self.reference.push();
    // `lua_getfenv` only applies to Lua closures; a C function has no accessible
    // environment. [`Function::is_lua_closure`] 自行压/弹一层，返回后栈顶仍是本函数。
    let env = if self.is_lua_closure() {
      // Safety: `state` 存活、由当前线程驱动，-1 是刚压入的 Lua 闭包；
      // `lua_getfenv(state, -1)` 压一层该闭包的环境表。
      unsafe { lua_getfenv(state, -1) };
      // Safety: 栈顶是刚压入的环境值，`lua_type` 只读其类型标记、不动栈。
      let is_table = unsafe { lua_type(state, -1) == LuaType::Table as c_int };
      // 命中表时 `pop_ref`（safe fn）登记引用并弹掉这一层；否则留待下面 settop 收口。
      if is_table {
        Some(Table::from_ref(lua.pop_ref()))
      } else {
        None
      }
    } else {
      None
    };
    // Safety: 统一收口——本函数自己压的一切（含未被 `pop_ref` 消费的
    // `lua_getfenv` 结果）都在这里弹回；`base` 是压入前的合法栈深。
    unsafe { lua_settop(state, base) };
    env
  }

  /// Set the function's environment table. Returns `Ok(false)` for a Rust
  /// (C) function (which has no settable environment) and `Ok(true)` for a
  /// Lua closure. Mirrors `mlua::Function::set_environment`.
  pub fn set_environment(&self, env: Table) -> Result<bool> {
    let lua = self.lua();
    let state = lua.state();
    // 需要 [func, env] 两个槽位：`ensure_stack`（safe fn）只报告头寸，
    // 下面 `push_to_stack` 与 `is_lua_closure` 的临时压栈都以它为前提。
    ensure_stack(state, 2)?;
    // Safety: `state` 存活；`lua_gettop` 只读栈深，`base` 是压入前深度。
    let base = unsafe { lua_gettop(state) };
    // `reference.push()`（safe fn）：注册表引用锚定函数值在世，VM 侧自保栈位。
    self.reference.push();
    // lua_setfenv 会弹掉环境表；本函数压的函数层由末尾 settop 收口。
    // `is_lua_closure` 在函数刚压栈顶后调用，自行压/弹一层复用上面预留的头寸。
    let ok = if self.is_lua_closure() {
      // `env.push_to_stack()` 是 safe 封装（`lua_rawgeti` 自带栈预留）；`env` 与本
      // 句柄同 VM 由 move-not-share 句柄纪律约束（无运行时校验，同 `set_globals`）。
      env.push_to_stack();
      // Safety: 栈布局 [func, env]，-2 正指刚压入的本 Lua 闭包；`lua_setfenv` 按
      // C API 约定消费栈顶环境表写入函数原型，返回是否成功。
      unsafe { lua_setfenv(state, -2) != 0 }
    } else {
      false
    };
    // Safety: `base` 合法，`lua_settop` 把本函数压的层（含任何中间压入）截回，
    // 净栈变化为零。
    unsafe { lua_settop(state, base) };
    Ok(ok)
  }

  /// Whether this function is a Lua closure (vs a C function). Determined via
  /// the debug `what` field.
  ///
  /// 自行把句柄值压栈、读完即弹，**调用方栈深不变**：因此不要求「栈顶必须是本
  /// 函数」这类脆弱前置条件，调用点也无需 `unsafe`。
  ///
  /// **不改变栈**：本仓库/上游 Luau 的 `lua_getinfo(L, level, what, ar)` 没有
  /// PUC-Rio Lua 5.x 的 `">"`（「读信息并弹栈」）约定——`>` 落到 `auxgetinfo`
  /// 的 `default:;` 被忽略，只有 `f` 选项会压栈（cpp/VM/src/ldebug.cpp:190-247
  /// 的 `lua_getinfo`/`auxgetinfo`，移植见
  /// `crates/ulua-vm/src/functions/{lua_getinfo,auxgetinfo}.rs`）。所以这里用
  /// `level = -1` 直接指栈顶，弹栈由本函数末尾负责。
  fn is_lua_closure(&self) -> bool {
    let state = self.reference.state();
    // `reference.push()`（safe fn）落 VM 侧 `lua_rawgeti` 自带栈预留，压入后 -1
    // 即本函数值——正满足下一行 `level = -1` 的解读。
    self.reference.push();
    // Safety: `state` 存活且由当前线程驱动（句柄 `XRc<LuaInner>` + `NotSync` 纪律），
    // 栈顶是上一行刚压入的本函数值，故 `level = -1` 解到它。`LuaDebug` 全部由
    // 整数/裸指针/定长数组字段构成，`zeroed` 是合法初值，`s` 选项命中时 `what`
    // 字段必被覆写。`GETINFO_S` 为静态 NUL 结尾选项串；`lua_getinfo` 不带 `f` 选项
    // 不压值。`is_lua_what_cstr` 是 safe 门面，自带 null 归一，无需此处判空。
    let closure = unsafe {
      let mut ar: LuaDebug = zeroed();
      let ok = lua_getinfo(state, -1, GETINFO_S.as_ptr().cast(), &mut ar);
      ok != 0 && is_lua_what_cstr(ar.what)
    };
    // Safety: `getinfo` 不压不弹，栈顶仍是 push 压入的本函数值；`state` 存活且
    // top>base，`lua_pop(state, 1)` 弹回这一层，恢复调用方栈深。
    unsafe { lua_pop(state, 1) };
    closure
  }

  /// Debug information about this function. Mirrors `mlua::Function::info`.
  pub fn info(&self) -> FunctionInfo {
    let lua = self.lua();
    let state = lua.state();
    // Safety: `state` 存活（句柄链 `XRc`）；`lua_gettop` 只读当前栈深，`base` 是
    // 本函数压入前的深度，末尾据此截回。
    let base = unsafe { lua_gettop(state) };
    // `reference.push()`（safe fn）的栈头寸与 id 有效性论证同 `environment`，
    // 压入后 `level = -1` 解到刚压入的本函数。
    self.reference.push();
    // Safety: `LuaDebug` 是整数/裸指针/定长数组的 POD，全零是合法初值（裸指针
    // 字段为 null，由 `debug_cstr` 退化为 `None`）。
    let mut ar: LuaDebug = unsafe { zeroed() };
    // Safety: `state` 存活、由当前线程驱动，栈顶是本函数值；选项串 `GETINFO_NSAU` 是
    // 静态 NUL 串且不含 `f`，不额外压栈。回填的 `what` 指 VM 静态字面量，
    // `source`/`name` 指经闭包→proto 可达的 TString（函数值在栈上即存活、GC 不
    // 移动），`short_src` 指 `ar` 内嵌缓冲——全部在下面的 `info` 构造里当场拷成
    // owned `String`，即在本帧内消费完毕，不跨 `lua_settop` 存活。
    let ok = unsafe { lua_getinfo(state, -1, GETINFO_NSAU.as_ptr().cast(), &mut ar) };
    // 纯 Rust 装配：把 `ar` 的 C 字段搬成 owned [`FunctionInfo`]。
    let info = if ok == 0 {
      FunctionInfo::default()
    } else {
      let what = debug_cstr(ar.what).unwrap_or_default();
      let line_defined = if ar.linedefined > 0 {
        Some(ar.linedefined as i64)
      } else {
        None
      };
      // Lua chunks are loaded with a `=<name>` chunkname marker; mlua
      // reports the bare name in `source`, so strip a single leading
      // `=`/`@` for Lua/main functions. C functions keep their VM-reported
      // source verbatim (e.g. `=[C]`), matching mlua.
      let source = debug_cstr(ar.source).map(|s| {
        if is_lua_what(&what) && (s.starts_with('=') || s.starts_with('@')) {
          s[1..].to_string()
        } else {
          s
        }
      });
      FunctionInfo {
        name: debug_cstr(ar.name),
        source,
        short_src: debug_cstr(ar.short_src),
        line_defined,
        last_line_defined: None, // Luau does not report it.
        what,
        num_upvalues: ar.nupvals,
        num_params: ar.nparams,
        is_vararg: ar.isvararg != 0,
      }
    };
    // Safety: `base` 是压入前深度，`lua_settop` 弹回 push 的一层（含任何中间压入），
    // 截至压入前深度，净栈变化为零。
    unsafe { lua_settop(state, base) };
    info
  }
}

/// `"Lua"` 和 `"main"` 是 Lua 闭包的 what 标记；`"C"` 是原生函数。
/// [`Function::is_lua_closure`] 与 [`Function::info`] 共用。
fn is_lua_what(what: &str) -> bool {
  what == "Lua" || what == "main"
}

/// 直接按字节判定 VM 内部的 `what` C 字符串（`"Lua"`/`"main"` 均为 ASCII，
/// 免去 UTF-8 校验与 `String` 分配）。safe 门面：null 由 `cstr_bytes` 收口
/// 归一为空切片（与任何目标都不等 ⇒ `false`），调用方不再自带判空哨兵。
fn is_lua_what_cstr(what: *const c_char) -> bool {
  // Safety: `lua_Debug.what` 的字段契约——被 `s` 选项回填时必指 VM 静态字面量
  // （"C"/"Lua"/"main"）的 NUL 结尾串，未被回填则保持 null；`cstr_bytes` 的
  // 前置条件（null 或 NUL 结尾）恰覆盖此域，扫描止于自带 NUL，比较只读字节。
  let bytes = unsafe { cstr_bytes(what) };
  bytes == b"Lua" || bytes == b"main"
}

/// Concatenate the bound prefix with `extra` (both `bind` variants share it).
fn concat_bound(bound: &[Value], extra: MultiValue) -> MultiValue {
  bound.iter().cloned().chain(extra).collect::<MultiValue>()
}

/// Debug information about a [`Function`]. Mirrors `mlua::debug::FunctionInfo`
/// (the subset Luau reports).
#[derive(Clone, Debug, Default)]
pub struct FunctionInfo {
  /// The function's name, if known (Luau records the call-site name).
  pub name: Option<String>,
  /// The chunk source name (e.g. `"=[C]"` for native functions).
  pub source: Option<String>,
  /// A short, human-readable source description.
  pub short_src: Option<String>,
  /// The line where the function was defined, if it is a Lua function.
  pub line_defined: Option<i64>,
  /// The last line of the function's definition. Always `None` in Luau.
  pub last_line_defined: Option<i64>,
  /// `"Lua"`, `"C"`, or `"main"`.
  pub what: String,
  /// The number of upvalues.
  pub num_upvalues: u8,
  /// The number of fixed parameters.
  pub num_params: u8,
  /// Whether the function is variadic.
  pub is_vararg: bool,
}

impl RegHandle for Function {
  fn reference(&self) -> &XRc<LuaRef> {
    &self.reference
  }
}

impl fmt::Debug for Function {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "Function")
  }
}

impl PartialEq for Function {
  fn eq(&self, other: &Self) -> bool {
    // Pointer identity (matches mlua): same underlying function object.
    self.to_pointer() == other.to_pointer()
  }
}

// ---------------------------------------------------------------------------
// LuaNativeFn: arity-abstracting sync closure trait (mirrors mlua)
// ---------------------------------------------------------------------------

/// A function/closure callable with a tuple of `FromLuaMulti` arguments,
/// abstracting over arity. Mirrors `mlua::LuaNativeFn`. Lets
/// [`Function::wrap`] accept `||`, `|a|`, `|a, b|`, … closures uniformly (the
/// closure receives the converted args directly, not the [`Lua`]).
pub trait LuaNativeFn<A: FromLuaMulti> {
  /// The closure's return type (typically `Result<R, E>`).
  type Output;

  /// Invoke the closure with the converted arguments.
  fn call(&self, args: A) -> Self::Output;
}

macro_rules! impl_lua_native_fn {
    ($($a:ident : $A:ident),*) => {
        impl<FN, $($A,)* R> LuaNativeFn<($($A,)*)> for FN
        where
            FN: Fn($($A,)*) -> R,
            ($($A,)*): FromLuaMulti,
        {
            type Output = R;

            fn call(&self, args: ($($A,)*)) -> R {
                let ($($a,)*) = args;
                self($($a,)*)
            }
        }
    };
}

impl_lua_native_fn!();
impl_lua_native_fn!(a: A);
impl_lua_native_fn!(a: A, b: B);
impl_lua_native_fn!(a: A, b: B, c: C);
impl_lua_native_fn!(a: A, b: B, c: C, d: D);
impl_lua_native_fn!(a: A, b: B, c: C, d: D, e: E);
impl_lua_native_fn!(a: A, b: B, c: C, d: D, e: E, f: F);
impl_lua_native_fn!(a: A, b: B, c: C, d: D, e: E, f: F, g: G);
impl_lua_native_fn!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);

/// A plain closure not yet bound to a [`Lua`]. Becomes a Lua function (via
/// [`Lua::create_function`]) when converted with [`IntoLua`]. Backs
/// [`Function::wrap`].
struct WrappedFunction<F, A, R, E> {
  func: F,
  _marker: PhantomData<fn(A) -> (R, E)>,
}

impl<F, A, R, E> IntoLua for WrappedFunction<F, A, R, E>
where
  F: LuaNativeFn<A, Output = StdResult<R, E>> + MaybeSend + 'static,
  A: FromLuaMulti,
  R: IntoLuaMulti,
  E: ExternalError,
{
  fn into_lua(self, lua: &Lua) -> Result<Value> {
    let func = self.func;
    let f = lua
      .create_function(move |_lua, args: A| func.call(args).map_err(ExternalError::into_lua_err))?;
    Ok(Value::Function(f))
  }
}
