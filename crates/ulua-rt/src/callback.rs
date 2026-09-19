//! The Rust-closure-as-Lua-function trampoline.
//!
//! ## Design (see also the crate-level docs)
//!
//! A user-supplied Rust closure is type-erased to
//! [`BoxedCallback`] = `Box<dyn Fn(&Lua, MultiValue) -> Result<MultiValue>>`.
//! That `Box` is stored inside a Lua userdata created with
//! [`lua_newuserdatadtor`] — the destructor reconstitutes and drops the `Box`,
//! so the closure's captured environment is freed exactly when the GC collects
//! the function. The userdata is then captured as **upvalue 1** of a single
//! C trampoline function ([`trampoline`]) pushed via [`lua_pushcclosurek`]
//! with `nup = 1`.
//!
//! When Lua calls the function:
//!  1. The trampoline fetches upvalue 1 with [`lua_upvalueindex`] and recovers
//!     `&BoxedCallback` from the userdata pointer.
//!  2. It pops all on-stack arguments into a [`MultiValue`].
//!  3. It runs the closure **inside [`catch_unwind`]** — so a `panic!` in user
//!     code can never become a nested panic while we are about to call
//!     [`lua_error`].
//!  4. On success it pushes the results and returns the count.
//!  5. On a returned `Err`, or a caught panic, it pushes a message string and
//!     calls [`lua_error`]. `lua_error` raises the VM's normal longjmp-style
//!     error (a `panic_any(lua_exception)`), which unwinds this trampoline
//!     frame up to the VM's protected-call boundary — the VM's own mechanism.
//!
//! Because the user panic is caught *before* `lua_error` is called, there is
//! never a double-unwind, and a genuine Rust panic in user code surfaces as an
//! ordinary catchable Lua error, not a process abort.

use core::{
  any::Any,
  mem::size_of,
  ptr::{drop_in_place, replace, write},
};
use std::{
  any::TypeId,
  borrow::Cow,
  panic::{AssertUnwindSafe, catch_unwind},
  thread::Result as ThreadResult,
};

use crate::{
  error::{Error, Result},
  function::Function,
  multi::MultiValue,
  state::Lua,
  sync::MaybeSend,
  sys::*,
  traits::{FromLuaMulti, IntoLuaMulti},
};

// ---------------------------------------------------------------------------
// Structured error objects (for errors that must survive the Lua boundary)
// ---------------------------------------------------------------------------
//
// Most callback errors are raised as plain Lua strings via `lua_error`, and
// `Lua::pop_error` rebuilds a flat `Error::RuntimeError`. That keeps the simple,
// message-only error path that the rest of the crate (and its tests) rely on.
//
// A small set of errors, however, carry *structured* meaning that the caller
// must be able to pattern-match after the error has travelled up through a
// `lua_pcall` boundary — specifically `CallbackDestructed` and
// `UserDataDestructed`, raised when a scope-created callback/userdata is invoked
// after its `Lua::scope` has ended. For those we push a **userdata error
// object** holding a boxed `Error` (tagged with a magic `TypeId` header), call
// `lua_error`, and recover the structured error in `pop_error`, wrapping it in
// `Error::CallbackError { cause, .. }` exactly like mlua.

/// 用户回调 panic 转为 Lua 错误时的消息前缀（callback / async / interrupt
/// 三处的 panic guard 共用）。
pub(crate) const PANIC_MSG_PREFIX: &str = "rust panic: ";

/// The wrapped-error userdata storage: a magic `TypeId` followed by the boxed
/// structured `Error`.
#[repr(C)]
struct WrappedError {
  type_id: TypeId,
  error: Box<Error>,
}

/// A private marker type whose `TypeId` tags our wrapped-error userdata.
struct WrappedErrorTag;

/// The magic tag identifying a ulua-rt wrapped-error userdata.
pub(crate) fn wrapped_error_tag() -> TypeId {
  TypeId::of::<WrappedErrorTag>()
}

/// Destructor for the [`WrappedError`] userdata: drops the boxed `Error`.
unsafe extern "C-unwind" fn wrapped_error_dtor(ptr: *mut c_void) {
  if !ptr.is_null() {
    unsafe { drop_in_place(ptr as *mut WrappedError) };
  }
}

/// Whether the given error should be raised as a *structured* userdata error
/// object (so the caller can match on it) rather than a flat string. Only the
/// scope-destruction errors qualify; everything else keeps the string path to
/// preserve backward-compatible `RuntimeError` behavior.
pub(crate) fn is_structured(err: &Error) -> bool {
  match err {
    Error::CallbackDestructed | Error::UserDataDestructed | Error::RecursiveMutCallback => true,
    // A `CallbackError` produced when a *nested* structured error crossed a
    // `pcall` boundary is itself re-raised structured, so each boundary adds
    // one `CallbackError` wrapper layer — mirroring mlua's nested
    // `CallbackError { cause: CallbackError { cause: .. } }`.
    Error::CallbackError { cause, .. } => is_structured(cause),
    _ => false,
  }
}

/// Push a structured [`Error`] as a wrapped-error userdata error object and
/// invoke [`lua_error`]. Diverges (unwinds via the VM's longjmp).
///
/// # Safety
/// `state` must be a valid `lua_State` with at least one free stack slot.
pub(crate) unsafe fn raise_structured_error(state: *mut lua_State, err: Error) -> ! {
  unsafe {
    let storage = lua_newuserdatadtor(state, size_of::<WrappedError>(), Some(wrapped_error_dtor));
    if storage.is_null() {
      // Fall back to a string error if we cannot allocate the userdata.
      raise_lua_error(state, &err.to_string());
    }
    write(
      storage as *mut WrappedError,
      WrappedError {
        type_id: wrapped_error_tag(),
        error: Box::new(err),
      },
    );
    lua_error(state) // diverges (`-> !`)
  }
}

/// If the value at stack index `idx` is a ulua-rt wrapped-error userdata,
/// return a clone of the contained [`Error`]. Does not pop.
///
/// # Safety
/// `state` must be valid and `idx` a valid (absolute) stack index.
pub(crate) unsafe fn recover_wrapped_error(state: *mut lua_State, idx: c_int) -> Option<Error> {
  unsafe {
    if lua_type(state, idx) != ttype::USERDATA {
      return None;
    }
    let ptr = lua_touserdata(state, idx);
    if ptr.is_null() {
      return None;
    }
    // A script can raise any userdata (`error(newproxy())` raises a zero-length
    // one), so the stored length must cover the layout before it is read.
    if lua_objlen(state, idx) < size_of::<WrappedError>() as c_int {
      return None;
    }
    // Only our wrapped errors carry the magic tag.
    let wrapped = &*(ptr as *const WrappedError);
    if wrapped.type_id != wrapped_error_tag() {
      return None;
    }
    Some((*wrapped.error).clone())
  }
}

/// The type-erased boxed callback stored in the trampoline's upvalue userdata.
///
/// Under the `send` feature the boxed closure is additionally `Send` (so a
/// callback may capture `Send` data moved in from another thread and the whole
/// VM can be moved across threads). Without the feature the `+ Send` bound is
/// absent and this is byte-identical to before. See [`crate::sync::MaybeSend`].
#[cfg(feature = "send")]
pub(crate) type BoxedCallback = Box<dyn Fn(&Lua, MultiValue) -> Result<MultiValue> + Send>;

/// See the `send`-gated variant above.
#[cfg(not(feature = "send"))]
pub(crate) type BoxedCallback = Box<dyn Fn(&Lua, MultiValue) -> Result<MultiValue>>;

/// The destructor installed on the callback userdata: reconstruct the `Box`
/// inside the userdata storage and drop it (calling `Drop` on captures).
///
/// `lua_newuserdatadtor` stores the data inline; `lua_touserdata` returns a
/// pointer to that storage, which is exactly where we wrote the
/// `BoxedCallback`. We drop it in place.
unsafe extern "C-unwind" fn callback_dtor(ptr: *mut c_void) {
  if !ptr.is_null() {
    let bc = ptr as *mut BoxedCallback;
    unsafe { drop_in_place(bc) };
  }
}

/// The one C trampoline shared by every `create_function` closure.
unsafe extern "C-unwind" fn trampoline(state: *mut lua_State) -> c_int {
  unsafe {
    // 1. Recover the boxed callback from upvalue 1.
    let ud = lua_touserdata(state, lua_upvalueindex(1));
    if ud.is_null() {
      // Should be impossible; fail loudly but safely via lua_error.
      raise_lua_error(state, "ulua-rt: missing callback upvalue");
    }
    let callback = &*(ud as *const BoxedCallback);

    // 2. Build a borrowed Lua handle for the calling thread (must NOT close it).
    let lua = Lua::from_borrowed(state);

    // 3. Pull the arguments off the stack into a MultiValue. They occupy
    //    stack indices 1..=nargs.
    let nargs = lua_gettop(state);
    let args = match collect_stack_args(&lua, nargs) {
      Ok(a) => a,
      Err(e) => raise_lua_error(state, &e.to_string()),
    };

    // 4. Run the user closure inside catch_unwind so a user `panic!` never
    //    becomes a nested panic when we then call lua_error.
    let outcome: ThreadResult<Result<MultiValue>> =
      catch_unwind(AssertUnwindSafe(|| callback(&lua, args)));

    match outcome {
      Ok(Ok(results)) => {
        // 5a. Push every result and return its count. Reserve stack space
        //     first: an unchecked push of a very large result list would
        //     overflow the Lua stack and trip a fatal VM assertion
        //     (SIGTRAP) instead of erroring. Guard it and raise a
        //     catchable error if the results cannot fit (mirroring mlua).
        let n = results.len() as c_int;
        if lua_checkstack(state, n.max(1)) == 0 {
          raise_lua_error(state, "too many results to return to Lua");
        }
        for v in results.iter() {
          if let Err(e) = lua.push_value(v) {
            raise_lua_error(state, &e.to_string());
          }
        }
        n
      }
      Ok(Err(err)) => {
        // 5b. The closure returned Err -> raise it as a Lua error.
        //     Structured errors (scope destruction) travel as a userdata
        //     error object so the caller can pattern-match on them; all
        //     others keep the flat string path.
        if is_structured(&err) {
          raise_structured_error(state, err)
        } else {
          raise_lua_error(state, &err.to_string())
        }
      }
      Err(panic_payload) => {
        // 5c. The closure panicked -> turn it into a catchable Lua error.
        let msg = panic_message(&panic_payload);
        raise_lua_error(state, &format!("{PANIC_MSG_PREFIX}{msg}"))
      }
    }
  }
}

/// The actual `lua_CFunction` pointer (an `Option<unsafe fn(...)>`).
fn trampoline_ptr() -> lua_CFunction {
  Some(trampoline)
}

/// Collect stack arguments `1..=nargs` into a [`MultiValue`] (stopping at the
/// first conversion error). Shared by the sync trampoline and the async
/// `get_future` closure.
///
/// # Safety
/// `1..=nargs` 必须是 `lua` 栈上的有效槽位（调用方是刚拿到 `lua_gettop` 的
/// trampoline）。
pub(crate) unsafe fn collect_stack_args(lua: &Lua, nargs: c_int) -> Result<MultiValue> {
  (1..=nargs).map(|i| lua.value_from_stack(i)).collect()
}

/// Push `msg` as the error object and invoke [`lua_error`]. Diverges (unwinds
/// via the VM's longjmp-style error). Shared with `async.rs` / `interrupt.rs`.
pub(crate) unsafe fn raise_lua_error(state: *mut lua_State, msg: &str) -> ! {
  unsafe {
    lua_pushlstring(state, msg.as_ptr() as *const c_char, msg.len());
    lua_error(state)
  }
}

/// Best-effort extraction of a panic payload's message (borrowed, no
/// allocation). Shared with `interrupt.rs` / `async.rs` (their user-callback
/// panic guards).
pub(crate) fn panic_message(payload: &Box<dyn Any + Send>) -> Cow<'_, str> {
  if let Some(s) = payload.downcast_ref::<&'static str>() {
    Cow::Borrowed(*s)
  } else if let Some(s) = payload.downcast_ref::<String>() {
    Cow::Borrowed(s)
  } else {
    Cow::Borrowed("unknown panic")
  }
}

/// 把 `Fn(&Lua, A) -> Result<R>` 装箱为类型擦除回调：参数经
/// [`FromLuaMulti`] 解包、返回值经 [`IntoLuaMulti`] 打包。
/// [`Lua::create_function`](crate::Lua::create_function) 与 userdata 的
/// `add_function` 共用这段转换样板。
pub(crate) fn wrap_callback<F, A, R>(func: F) -> BoxedCallback
where
  F: Fn(&Lua, A) -> Result<R> + MaybeSend + 'static,
  A: FromLuaMulti,
  R: IntoLuaMulti,
{
  Box::new(move |lua, args| {
    let a = A::from_lua_multi(args, lua)?;
    func(lua, a)?.into_lua_multi(lua)
  })
}

/// Build a [`Function`] from a type-erased boxed callback. Used by
/// [`Lua::create_function`] and the userdata method machinery.
pub(crate) fn create_callback_function(lua: &Lua, callback: BoxedCallback) -> Result<Function> {
  let state = lua.state();
  unsafe {
    // Allocate userdata sized for a BoxedCallback, with our dtor.
    let storage = lua_newuserdatadtor(state, size_of::<BoxedCallback>(), Some(callback_dtor));
    if storage.is_null() {
      return Err(Error::runtime(
        "ulua-rt: failed to allocate callback userdata",
      ));
    }
    // Move the box into the userdata storage (do NOT run its drop here).
    write(storage as *mut BoxedCallback, callback);

    // The userdata is now on top of the stack; capture it as upvalue 1 of
    // the trampoline closure.
    lua_pushcclosurek(
      state,
      trampoline_ptr(),
      c"ulua-rt-callback".as_ptr(),
      1, // nup: consumes the userdata above as upvalue 1
      None,
    );
    // The closure is now on top; take a registry ref.
    Ok(Function::from_ref(lua.pop_ref()))
  }
}

/// Neutralise a scope-created callback: replace the boxed closure stored in the
/// function's upvalue-1 userdata with a sentinel that always returns
/// [`Error::CallbackDestructed`], dropping the original closure (and thereby
/// ending any borrows it held).
///
/// This is the **invalidation half** of `Lua::scope`'s soundness guarantee: the
/// original closure (which may borrow non-`'static` data) is dropped here, on
/// scope exit, *before* the borrowed data's lifetime can end. The Lua function
/// object itself is left fully valid — only its behavior changes to "destructed"
/// — so a post-scope call from Lua hits the sentinel and surfaces as
/// `CallbackError { cause: CallbackDestructed }` instead of touching freed
/// memory.
///
/// Must be called while the scope (and hence the VM) is still alive.
pub(crate) fn destruct_callback(func: &Function) {
  let lua = func.lua();
  let state = lua.state();
  unsafe {
    // Push the function, then fetch its upvalue 1 (the callback userdata).
    func.push_to_stack();
    let name = lua_getupvalue(state, -1, 1);
    if name.is_null() {
      // No upvalue (should not happen for our callbacks); just pop the fn.
      lua_pop(state, 1);
      return;
    }
    // stack: [func, upvalue-userdata]
    let ud = lua_touserdata(state, -1);
    if !ud.is_null() {
      let slot = ud as *mut BoxedCallback;
      // Swap in the sentinel; the returned old box is dropped at end of
      // scope here, running Drop on the original closure's captures.
      let sentinel: BoxedCallback = Box::new(|_lua, _args| Err(Error::CallbackDestructed));
      let old = replace(slot, sentinel);
      drop(old);
    }
    // Pop the upvalue and the function.
    lua_pop(state, 2);
  }
}
