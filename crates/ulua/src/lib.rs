//! # ulua
//!
//! A faithful Rust translation of [Luau](https://github.com/luau-lang/luau) —
//! Roblox's typed Lua. This umbrella crate re-exports the individual layers
//! (lexer/parser/AST, bytecode, compiler, register VM, type checker, config and
//! require resolution) and provides three thin convenience helpers — [`compile`],
//! [`eval`], and [`check`] — for the common "compile a string / run a string /
//! type-check a string" cases.
//!
//! For finer-grained control depend on the sub-crates directly; they are all
//! re-exported here as modules.
//!
//! ```
//! ulua::eval("assert(1 + 1 == 2)").unwrap();
//! let bytecode = ulua::compile("return 2 + 2").unwrap();
//! assert!(!bytecode.is_empty());
//! ulua::check("local x: number = 1").unwrap();
//! ```

// Re-export the sub-crates as modules so `ulua::vm::...` etc. work from one dep.
use core::{
  ffi::{CStr, c_char},
  ptr::null_mut,
  slice::from_raw_parts,
};
use std::result::Result as StdResult;

pub use ulua_analysis as analysis;
pub use ulua_ast as ast;
pub use ulua_bytecode as bytecode;
/// Compile-time checked Luau source macros (`checked-macros` feature).
#[cfg(feature = "checked-macros")]
pub use ulua_checked_macros::{luau, luau_file, ulua, ulua_file};
pub use ulua_common as common;
pub use ulua_compiler as compiler;
pub use ulua_config as config;
pub use ulua_require as require;
pub use ulua_rt as rt;
/// The `async`-feature coroutine-as-`Future`/`Stream` driver, re-exported when
/// the umbrella's `async` feature (which forwards to `ulua-rt/async`) is on.
#[cfg(feature = "async")]
pub use ulua_rt::AsyncThread;
// The headline high-level, mlua-style API. Re-exported flat at the crate root
// so `ulua::Lua`, `ulua::Table`, etc. are available directly.
//
// NOTE on the derive macros: ulua-rt's `#[derive(UserData)]` /
// `#[derive(FromLua)]` (behind ulua-rt's `macros` feature) emit absolute
// `::ulua_rt::...` paths, so they are designed to be used through the
// `ulua-rt` crate directly (`#[derive(ulua_rt::UserData)]`). They are **not**
// re-exported here through the umbrella `ulua`: a `pub use` re-export does not
// give the user's crate an extern-crate name `ulua_rt`, so the macro's
// `::ulua_rt::...` paths would fail to resolve when invoked as
// `ulua::UserData`. Mirroring mlua's single-crate model, the derives live on
// `ulua-rt`. (Re-exported `ulua::rt` already aliases the crate for the rest
// of the API.)
pub use ulua_rt::{
  AnyUserData, AppDataRef, AppDataRefMut, Buffer, Chunk, ChunkMode, Compiler, Debug, DebugWhat,
  Error, ExternalError, ExternalResult, FromLua, FromLuaMulti, Function, FunctionInfo, Integer,
  IntoLua, IntoLuaMulti, LightUserData, Lua, LuaNativeFn, LuaOptions, LuaString, MaybeSend,
  MaybeSync, MetaMethod, MultiValue, Nil, Number, RegistryKey, Result, Scope, StdLib, Table,
  TablePairs, TableSequence, Thread, ThreadStatus, TypeMetatable, UserData, UserDataFields,
  UserDataMethods, UserDataRef, UserDataRefMut, Value, Variadic, Vector, VmState, WeakLua,
};
/// The `serde`-feature Rust↔Lua serialization surface, re-exported when the
/// umbrella's `serde` feature (forwarding to `ulua-rt/serde`) is on.
#[cfg(feature = "serde")]
pub use ulua_rt::{
  DeserializeOptions, LuaDeserializer, LuaSerdeExt, LuaSerializer, SerializableTable,
  SerializableValue, SerializeOptions,
};
// The static type-check helpers now live on `ulua-rt` (behind its `typecheck`
// feature, which the umbrella turns on by default). Re-export them so
// `ulua::check` / `ulua::check_with_definitions` stay public, alongside the
// structured `TypeDiagnostic` they now return.
#[cfg(feature = "typecheck")]
pub use ulua_rt::{
  TypeDiagnostic, check, check_modules, check_modules_with_definitions, check_with_definitions,
};
pub use ulua_vm as vm;

/// Common entry points, re-exported for convenience.
pub mod prelude {
  pub use ulua_ast::records::parse_options::ParseOptions;
  pub use ulua_compiler::records::compile_options::CompileOptions;
  // The mlua-style high-level API prelude (Lua, Value, Table, traits, ...).
  pub use ulua_rt::prelude::*;

  /// The type-check helpers (the `typecheck` feature; on by default).
  #[cfg(feature = "typecheck")]
  pub use crate::{check, check_modules, check_modules_with_definitions, check_with_definitions};
  pub use crate::{compile, eval};
}

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_common::set_luau_bool_flags;
use ulua_compiler::{
  functions::compile::compile as compiler_compile, records::compile_options::CompileOptions,
};
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_checkstack::lua_checkstack, lua_close::lua_close, lua_debugtrace::lua_debugtrace,
    lua_gettop::lua_gettop, lua_insert::lua_insert, lua_l_newstate::lua_l_newstate,
    lua_l_openlibs::lua_l_openlibs, lua_newthread::lua_newthread, lua_pcall::lua_pcall,
    lua_pushvalue::lua_pushvalue, lua_remove::lua_remove, lua_resume::lua_resume,
    lua_tolstring::lua_tolstring, lua_xmove::lua_xmove, luau_load::luau_load,
  },
  macros::{
    lua_getglobal::lua_getglobal, lua_isnil::lua_isnil, lua_minstack::LUA_MINSTACK,
    lua_pop::lua_pop,
  },
  type_aliases::lua_state::lua_State,
};

/// Compile Luau `source` to bytecode using default compile/parse options.
///
/// On success the raw bytecode blob is returned. On a parse or compile error the
/// compiler emits an "error blob" (a leading `\0` marker byte followed by the
/// human-readable message); we detect that marker and surface the message as
/// [`Error::SyntaxError`] instead.
pub fn compile(source: &str) -> StdResult<Vec<u8>, Error> {
  let options = CompileOptions::default();
  let parse_options = ParseOptions::default();

  let bytes = compiler_compile(source, &options, &parse_options, NoopEncoder);
  // A leading 0 byte is the compiler's error marker (valid bytecode starts with
  // the non-zero LBC_VERSION_TARGET).
  if bytes.first() == Some(&0u8) {
    let message = String::from_utf8_lossy(&bytes[1..]).into_owned();
    return Err(Error::SyntaxError {
      message,
      incomplete_input: false,
    });
  }
  Ok(bytes)
}

/// Compile, load and run `source`, mirroring cpp `CLI/src/Repl.cpp` `main` +
/// `runCode` (Repl.cpp:239-300): a fresh state with the standard library, the
/// chunk loaded on the main state then moved to a fresh coroutine, and
/// `lua_resume` to execute it. On normal return, leftover results are printed
/// through `_PRETTYPRINT` (falling back to `print`), exactly like the REPL.
///
/// Returns `Ok(())` if the script ran to completion, or [`Error`] carrying the
/// Lua error text with the `lua_debugtrace` stack backtrace appended (as the
/// C++ REPL does) on a compile, load or runtime error.
pub fn eval(source: &str) -> StdResult<(), Error> {
  // v11+ bytecode needs the default Luau flags enabled (the CLI's
  // setLuauFlagsDefault(true) analog; safe entry point).
  set_luau_bool_flags(true);

  let bytecode = compile(source)?;

  unsafe {
    let l = lua_l_newstate();
    if l.is_null() {
      return Err(Error::MemoryError(
        "lua_l_newstate returned null".to_string(),
      ));
    }

    // RAII 对应 cpp 侧 `std::unique_ptr<lua_State, lua_close>`: 所有退出路径
    // (含 `?`/early return) 都回收 VM; 子线程随宿主 state 一并销毁。
    struct StateGuard(*mut lua_State);
    impl Drop for StateGuard {
      fn drop(&mut self) {
        if !self.0.is_null() {
          unsafe { lua_close(self.0) };
        }
      }
    }
    let _state = StateGuard(l);

    lua_l_openlibs(l);
    run_code(l, &bytecode)
  }
}

/// cpp `Repl.cpp:239 runCode` 的加载/执行段（bytecode 已在外部编译）。
/// # Safety
/// `l` 必须是由 `lua_l_newstate` 创建且已 openlibs 的有效状态机。
unsafe fn run_code(l: *mut lua_State, bytecode: &[u8]) -> StdResult<(), Error> {
  unsafe {
    let rc = luau_load(
      l,
      c"=eval".as_ptr(),
      bytecode.as_ptr() as *const c_char,
      bytecode.len(),
      0,
    );
    if rc != LuaStatus::Ok as i32 {
      // C++ 此处直接 lua_tolstring(-1) 取错误文本并 pop。
      let mut len = 0usize;
      let s = lua_tolstring(l, -1, &mut len);
      let message = if s.is_null() {
        String::new()
      } else {
        String::from_utf8_lossy(from_raw_parts(s.cast::<u8>(), len)).into_owned()
      };
      lua_pop(l, 1);
      return Err(Error::RuntimeError(message));
    }

    let t = lua_newthread(l);
    if t.is_null() {
      return Err(Error::MemoryError(
        "lua_newthread returned null".to_string(),
      ));
    }

    // 闭包换入新协程：栈顶是 thread，-2 是刚 load 出的闭包。
    lua_pushvalue(l, -2);
    lua_remove(l, -3);
    lua_xmove(l, t, 1);

    let status = lua_resume(t, null_mut(), 0);
    if status == LuaStatus::Ok as i32 {
      let n = lua_gettop(t);
      if n != 0 {
        lua_checkstack(t, LUA_MINSTACK);
        lua_getglobal(t, c"_PRETTYPRINT".as_ptr());
        // _PRETTYPRINT 为 nil 时回退到标准 print（与 Repl.cpp 一致）。
        if lua_isnil!(t, -1) {
          lua_pop(t, 1);
          lua_getglobal(t, c"print".as_ptr());
        }
        lua_insert(t, 1);
        lua_pcall(t, n, 0, 0);
      }
      lua_pop(l, 1);
      return Ok(());
    }

    let mut error = String::new();
    if status == LuaStatus::Yield as i32 {
      error.push_str("thread yielded unexpectedly");
    } else {
      // C++ 用 lua_tostring（= lua_tolstring(L,-1,NULL)），非字符串得 nullptr，
      // 错误文本为空但仍追加回溯。这里改用带长度出参的 tolstring 以保留嵌入
      // NUL 字节（已知超集偏差），且不再引入 "<non-string error>" 哨兵。
      let mut len = 0usize;
      let s = lua_tolstring(t, -1, &mut len);
      if !s.is_null() {
        error.push_str(&String::from_utf8_lossy(from_raw_parts(
          s.cast::<u8>(),
          len,
        )));
      }
    }
    // C++ 无条件追加 "\nstack backtrace:\n" + lua_debugtrace(T)。
    error.push_str("\nstack backtrace:\n");
    let trace = lua_debugtrace(t);
    if !trace.is_null() {
      error.push_str(&String::from_utf8_lossy(CStr::from_ptr(trace).to_bytes()));
    }
    lua_pop(l, 1);
    Err(Error::RuntimeError(error))
  }
}
