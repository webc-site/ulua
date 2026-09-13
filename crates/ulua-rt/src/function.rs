//! The [`Function`] handle. Mirrors `mlua::Function`.

#[cfg(feature = "async")]
use core::future::Future;
use core::{
  ffi::CStr,
  fmt::{self, Formatter},
  marker::PhantomData,
  mem::zeroed,
  result::Result as StdResult,
  str::from_utf8,
};

#[cfg(feature = "async")]
use crate::async_support::{AsyncThread, LuaNativeAsyncFn, WrappedAsync, register_implicit_thread};
use crate::{
  debug::debug_cstr,
  error::{Error, ExternalError, Result},
  multi::MultiValue,
  state::{Lua, LuaRef},
  sync::{MaybeSend, NOT_SYNC, NotSync, XRc},
  sys::*,
  table::Table,
  traits::{FromLuaMulti, IntoLua, IntoLuaMulti},
  value::Value,
};

/// A handle to a callable Lua value (a Lua closure or a Rust function).
///
/// Mirrors `mlua::Function`. Under the `send` feature it is `Send` but never
/// `Sync` — see [`crate::sync::NotSync`].
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

  pub(crate) unsafe fn push_to_stack(&self) {
    self.reference.push();
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

    unsafe {
      let base = lua_gettop(state);
      let nargs = args.len() as c_int;
      // Guard against pushing more values than the Lua stack can hold:
      // an unprotected overflow would abort the VM. We need room for the
      // function + all arguments (+1 slack for the call machinery).
      if lua_checkstack(state, nargs.saturating_add(2)) == 0 {
        return Err(Error::RuntimeError(
          "stack overflow: too many arguments to function call".to_string(),
        ));
      }
      // Push the function, then the arguments.
      self.reference.push();
      for v in args.iter() {
        lua.push_value(v)?;
      }
      // LUA_MULTRET == -1: keep every result.
      let status = lua_pcall(state, nargs, -1, 0);
      if status != 0 {
        return Err(lua.pop_error(status));
      }
      // The result-collection loop below reads each reference-typed result
      // via `value_from_stack`, which DUPLICATES it onto the stack
      // (`lua_pushvalue`) before popping it into a registry ref. After a
      // LUA_MULTRET call the results can fill the C frame's stack exactly to
      // `ci->top` (LUA_MINSTACK), so that duplicating push would overrun the
      // frame — the `api_incr_top` assert in `lua_pushvalue`. Reserve a slot
      // of headroom for it. (Found by the `run` fuzzer:
      // `local t={a=1}; return <~20 values including t>`.)
      if lua_checkstack(state, 2) == 0 {
        lua_settop(state, base);
        return Err(Error::RuntimeError(
          "stack overflow: too many return values".to_string(),
        ));
      }
      // Collect every value pushed above `base` as the results.
      let results = lua.collect_results_above(state, base)?;
      R::from_lua_multi(results, &lua)
    }
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
      let mut th = thread.into_async(args)?;
      th.set_implicit(true);
      Ok(th)
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
    let state = self.reference.state();
    unsafe {
      self.reference.push();
      let p = lua_topointer(state, -1);
      lua_pop(state, 1);
      p
    }
  }

  /// The function's environment table (its globals), or `None` for a Rust
  /// (C) function. Mirrors `mlua::Function::environment`.
  pub fn environment(&self) -> Option<Table> {
    let lua = self.lua();
    let state = lua.state();
    unsafe {
      self.reference.push();
      // `lua_getfenv` only applies to Lua closures; a C function has no
      // accessible environment.
      if !self.is_lua_closure() {
        lua_pop(state, 1);
        return None;
      }
      lua_getfenv(state, -1);
      // stack: [func, env]
      if lua_type(state, -1) != ttype::TABLE {
        lua_pop(state, 2);
        return None;
      }
      let env = Table::from_ref(lua.pop_ref());
      lua_pop(state, 1); // pop func
      Some(env)
    }
  }

  /// Set the function's environment table. Returns `Ok(false)` for a Rust
  /// (C) function (which has no settable environment) and `Ok(true)` for a
  /// Lua closure. Mirrors `mlua::Function::set_environment`.
  pub fn set_environment(&self, env: Table) -> Result<bool> {
    let lua = self.lua();
    let state = lua.state();
    unsafe {
      self.reference.push();
      if !self.is_lua_closure() {
        lua_pop(state, 1);
        return Ok(false);
      }
      // stack: [func]; push env, then lua_setfenv(func_index).
      env.push_to_stack();
      let ok = lua_setfenv(state, -2);
      // lua_setfenv pops the env table; pop the function too.
      lua_pop(state, 1);
      Ok(ok != 0)
    }
  }

  /// Whether the value on top of the stack (this function, just pushed) is a
  /// Lua closure (vs a C function). Determined via the debug `what` field.
  unsafe fn is_lua_closure(&self) -> bool {
    let state = self.reference.state();
    unsafe {
      // The function is on top of the stack (index -1). Ask lua_getinfo
      // about it via the ">" level convention: push the function and use
      // option ">" so it pops the function and reads its info.
      lua_pushvalue(state, -1);
      let mut ar: LuaDebug = zeroed();
      let opt = c">s";
      let ok = lua_getinfo(state, -1, opt.as_ptr() as *const c_char, &mut ar);
      if ok == 0 {
        return false;
      }
      if ar.what.is_null() {
        return false;
      }
      let what = CStr::from_ptr(ar.what).to_bytes();
      from_utf8(what).map(is_lua_what).unwrap_or(false)
    }
  }

  /// Debug information about this function. Mirrors `mlua::Function::info`.
  pub fn info(&self) -> FunctionInfo {
    let lua = self.lua();
    let state = lua.state();
    unsafe {
      self.reference.push();
      let mut ar: LuaDebug = zeroed();
      // Options: n=name, s=source/what/linedefined, a=params/vararg,
      // u=upvalues. The ">" prefix pops the function from the stack and
      // reads info about it.
      let opt = c">nsau";
      let ok = lua_getinfo(state, -1, opt.as_ptr() as *const c_char, &mut ar);
      if ok == 0 {
        return FunctionInfo::default();
      }
      let cstr = debug_cstr;
      let what = cstr(ar.what).unwrap_or_default();
      let line_defined = if ar.linedefined > 0 {
        Some(ar.linedefined as i64)
      } else {
        None
      };
      // Lua chunks are loaded with a `=<name>` chunkname marker; mlua
      // reports the bare name in `source`, so strip a single leading
      // `=`/`@` for Lua/main functions. C functions keep their VM-reported
      // source verbatim (e.g. `=[C]`), matching mlua.
      let source = cstr(ar.source).map(|s| {
        if is_lua_what(&what) && (s.starts_with('=') || s.starts_with('@')) {
          s[1..].to_string()
        } else {
          s
        }
      });
      FunctionInfo {
        name: cstr(ar.name),
        source,
        short_src: cstr(ar.short_src),
        line_defined,
        last_line_defined: None, // Luau does not report it.
        what,
        num_upvalues: ar.nupvals,
        num_params: ar.nparams,
        is_vararg: ar.isvararg != 0,
      }
    }
  }
}

/// `"Lua"` 和 `"main"` 是 Lua 闭包的 what 标记；`"C"` 是原生函数。
/// [`Function::is_lua_closure`] 与 [`Function::info`] 共用。
fn is_lua_what(what: &str) -> bool {
  what == "Lua" || what == "main"
}

/// Concatenate the bound prefix with `extra` (both `bind` variants share it).
fn concat_bound(bound: &[Value], extra: MultiValue) -> MultiValue {
  let mut all = MultiValue::with_capacity(bound.len() + extra.len());
  for v in bound {
    all.push_back(v.clone());
  }
  for v in extra {
    all.push_back(v);
  }
  all
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
