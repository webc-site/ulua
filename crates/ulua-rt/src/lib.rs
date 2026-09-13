//! # ulua-rt
//!
//! A safe, ergonomic, **mlua-style** high-level API for
//! [`ulua`](https://github.com/webc-site/ulua) — a pure-Rust translation of
//! Roblox's [Luau](https://github.com/luau-lang/luau).
//!
//! The public surface deliberately mirrors [`mlua`](https://docs.rs/mlua)'s
//! interface — the same type names ([`Lua`], [`Value`], [`Table`],
//! [`Function`], [`LuaString`], [`MultiValue`], [`Variadic`], [`Error`]), the
//! same method names and call shapes (`Lua::new`, `lua.globals()`,
//! `lua.create_function`, `lua.load(src).eval::<T>()`, `table.set/get`,
//! `function.call::<R>(args)`), and the same conversion traits ([`FromLua`],
//! [`IntoLua`], [`FromLuaMulti`], [`IntoLuaMulti`]) and userdata traits
//! ([`UserData`], [`UserDataMethods`]). The *implementation*, however, is
//! entirely original: it is written directly over ulua's pure-Rust C API
//! (`lua_*`), not over a C FFI.
//!
//! ```
//! use ulua_rt::prelude::*;
//!
//! let lua = Lua::new();
//! let add = lua
//!     .create_function(|_, (a, b): (i64, i64)| Ok(a + b))
//!     .unwrap();
//! lua.globals().set("add", add).unwrap();
//! let sum: i64 = lua.load("return add(2, 3)").eval().unwrap();
//! assert_eq!(sum, 5);
//! ```
//!
//! ## Single-threaded
//!
//! Like mlua's default, [`Lua`] is single-threaded: it is built on `Rc`, so it
//! is neither `Send` nor `Sync`. Clone a [`Lua`] to get another handle to the
//! same VM.
//!
//! ## Deferred (not yet implemented)
//!
//! The following parts of mlua's surface are intentionally **out of scope** and
//! are noted here rather than implemented:
//!
//! - Multi-VM `Send`/`Sync` (`WeakLua`, send-able handles) (P4).
//! - Thread event callbacks (`ThreadEvent`/`ThreadTriggers`/
//!   `set_thread_event_callback`) and per-thread hooks.
//! - The `chunk!` proc-macro.
//! - Async userdata methods (`add_async_method*`) and the `ObjectLike`
//!   `call_async_method` surface (depend on the deferred userdata registry).
//!
//! Implemented in Phase 1: `Thread`/coroutine wrappers, public `RegistryKey`
//! storage, `UserDataFields`, typed `AnyUserData::borrow`/`borrow_mut`/`take`/
//! `is`, the `MetaMethod` enum, and `Function::info`/`environment`.
//!
//! Implemented in Phase 2: the Luau-specific runtime types [`Buffer`] (the
//! `buffer` type) and [`Vector`] (the `vector` type), with their
//! [`Value::Buffer`]/[`Value::Vector`] variants and `FromLua`/`IntoLua` impls.
//!
//! Implemented in Phase 4a (behind the `serde` cargo feature): serde
//! (de)serialization between Rust types and Lua [`Value`]s — the `LuaSerdeExt`
//! trait on [`Lua`], a serde `Serializer`/`Deserializer` over [`Value`]/
//! [`Table`], `SerializeOptions`/`DeserializeOptions`, and `Serialize` impls
//! for [`Value`]/[`Table`] with the `Value::to_serializable` wrapper.
//!
//! Implemented in Phase 4c (behind the `async` cargo feature): Rust `async`/
//! `await` support — the Rust-`Future` ⟷ Lua-coroutine bridge.
//! `Lua::create_async_function` / `Lua::yield_with`, `Function::call_async` /
//! `Function::wrap_async` / `Function::wrap_raw_async`, `Chunk::call_async` /
//! `Chunk::exec_async` / `Chunk::eval_async`, and `Thread::into_async`
//! producing an `AsyncThread` that implements `Future` + `Stream`.
//! Executor-agnostic, like mlua: the caller drives the returned futures on
//! their own runtime.

#![forbid(unsafe_op_in_unsafe_fn)]

// `send` + `async` now compose. The async bridge keeps its per-VM waker +
// implicit-thread ownership map in a process-wide table keyed by the VM's
// global-state pointer (a real `Mutex` under `send`, a thread-local otherwise),
// so the state travels with the VM across thread moves; the type-erased async
// callback / future boxes carry a `MaybeSend` bound (`+ Send` under `send`)
// exactly like the synchronous callbacks. See `async.rs` + `sync.rs`.

mod app_data;
#[cfg(feature = "async")]
#[path = "async.rs"]
mod async_support;
mod buffer;
mod callback;
mod chunk;
mod compiler;
mod conversion;
mod debug;
mod error;
mod exec_raw;
/// The public, mlua-style `ffi` surface (mounted from `ffi_public.rs`).
#[path = "ffi_public.rs"]
pub mod ffi;
mod function;
mod gc;
mod interrupt;
mod light_userdata;
mod luau_ext;
mod memory;
mod metamethod;
mod module;
mod multi;
mod options;
mod registry;
mod scope;
#[cfg(feature = "serde")]
mod serde;
pub mod state;
mod string;
mod sync;
/// The internal, crate-private ulua C API re-exports (mounted from `ffi.rs`).
#[path = "ffi.rs"]
mod sys;
mod table;
mod thread;
mod traits;
#[cfg(feature = "typecheck")]
mod typecheck;
mod userdata;
mod value;
mod vector;

pub use app_data::{AppDataRef, AppDataRefMut};
/// The [`AsyncThread`] driver — a coroutine being run to completion as a Rust
/// `Future`/`Stream` (the `async` feature).
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub use async_support::AsyncThread;
pub use buffer::Buffer;
pub use chunk::{Chunk, ChunkMode};
pub use compiler::Compiler;
pub use debug::{Debug, DebugWhat};
pub use error::{Error, ExternalError, ExternalResult, Result};
pub use function::{Function, FunctionInfo, LuaNativeFn};
pub use interrupt::VmState;
pub use light_userdata::LightUserData;
pub use luau_ext::TypeMetatable;
pub use metamethod::MetaMethod;
pub use multi::{MultiValue, Variadic};
pub use options::{LuaOptions, StdLib};
pub use registry::RegistryKey;
pub use scope::Scope;
#[cfg(feature = "serde")]
pub use serde::{
  DeserializeOptions, Deserializer as LuaDeserializer, LuaSerdeExt, SerializableTable,
  SerializableValue, SerializeOptions, Serializer as LuaSerializer,
};
pub use state::{Lua, WeakLua};
pub use string::LuaString;
pub use sync::{MaybeSend, MaybeSync};
pub use table::{Table, TablePairs, TableSequence};
pub use thread::{Thread, ThreadStatus};
pub use traits::{FromLua, FromLuaMulti, IntoLua, IntoLuaMulti};
/// Static type-checking surface (the `typecheck` feature): the structured
/// [`TypeDiagnostic`] type plus the free [`check`] / [`check_with_definitions`]
/// helpers. The `Lua`/`Chunk` objects gain `check` / `add_definitions` methods
/// (see [`Lua::check`] / [`Chunk::check`]) that return
/// [`Error::TypeError`](crate::Error::TypeError).
#[cfg(feature = "typecheck")]
#[cfg_attr(docsrs, doc(cfg(feature = "typecheck")))]
pub use typecheck::{
  Checker, TypeDiagnostic, check, check_modules, check_modules_with_definitions,
  check_with_definitions,
};
/// The `#[derive(UserData)]` / `#[derive(FromLua)]` procedural derive macros
/// (mirroring mlua's `macros` feature), re-exported so users can write
/// `#[derive(ulua_rt::UserData)]` / `#[derive(ulua_rt::FromLua)]`.
#[cfg(feature = "macros")]
pub use ulua_rt_derive::{FromLua, UserData};
pub use userdata::{
  AnyUserData, UserData, UserDataFields, UserDataMethods, UserDataRef, UserDataRefMut,
};
/// `Value::Nil`, re-exported as a bare name (the enum *variant*, so it works in
/// both value and pattern position). Mirrors `mlua::Nil`.
pub use value::Value::Nil;
pub use value::{Integer, Number, Value};
pub use vector::Vector;

/// Idiomatic glob-import prelude. Mirrors `mlua::prelude`, additionally
/// re-exporting the short names so `use ulua_rt::prelude::*;` brings the whole
/// ergonomic surface into scope.
pub mod prelude {
  // mlua-style `Lua*`-prefixed aliases for users coming from mlua's prelude.
  pub use crate::{
    AnyUserData as LuaAnyUserData, AnyUserData, Buffer, Chunk, Error, Error as LuaError,
    ExternalError, ExternalResult, FromLua, FromLuaMulti, Function, Function as LuaFunction,
    IntoLua, IntoLuaMulti, Lua, LuaString, MetaMethod, MetaMethod as LuaMetaMethod, MultiValue,
    MultiValue as LuaMultiValue, RegistryKey, RegistryKey as LuaRegistryKey, Result,
    Result as LuaResult, Scope, Table, Table as LuaTable, Thread, Thread as LuaThread,
    ThreadStatus, ThreadStatus as LuaThreadStatus, UserData, UserData as LuaUserData,
    UserDataFields, UserDataFields as LuaUserDataFields, UserDataMethods,
    UserDataMethods as LuaUserDataMethods, Value, Value as LuaValue, Variadic,
    Variadic as LuaVariadic, Vector,
  };
  // `LuaString` already carries the `Lua` prefix.
}

/// A raw `lua_State` pointer type alias, re-exported at the crate root for
/// signature parity with `mlua::lua_State`.
pub use ulua_vm::type_aliases::lua_state::lua_State;
