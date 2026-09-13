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
/// Compile-time checked Luau source macros (`checked-macros` feature).
use core::ffi::c_char;
use core::{ptr::null_mut, result, slice::from_raw_parts};

pub use ulua_analysis as analysis;
pub use ulua_ast as ast;
pub use ulua_bytecode as bytecode;
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
use ulua_bytecode::records::bytecode_encoder::BytecodeEncoder;
use ulua_compiler::{
  functions::compile::compile as compiler_compile, records::compile_options::CompileOptions,
};

/// A no-op bytecode encoder — the compiler requires an encoder, and the default
/// (no encryption / no transform) one simply leaves the words untouched, exactly
/// like Luau's own `BytecodeEncoder` base class.
struct NoopEncoder;

impl BytecodeEncoder for NoopEncoder {
  fn encode(&mut self, _data: &mut [u32]) {}
}

/// Compile Luau `source` to bytecode using default compile/parse options.
///
/// On success the raw bytecode blob is returned. On a parse or compile error the
/// compiler emits an "error blob" (a leading `\0` marker byte followed by the
/// human-readable message); we detect that marker and surface the message as the
/// `Err` variant instead.
pub fn compile(source: &str) -> result::Result<Vec<u8>, String> {
  let options = CompileOptions::default();
  let parse_options = ParseOptions::default();
  let mut encoder = NoopEncoder;
  let owned = source.to_string();

  let blob = compiler_compile(
    &owned,
    &options,
    &parse_options,
    &mut encoder as *mut dyn BytecodeEncoder,
  );

  let bytes = blob.into_bytes();
  // A leading 0 byte is the compiler's error marker (valid bytecode starts with
  // the non-zero LBC_VERSION_TARGET).
  if bytes.first() == Some(&0u8) {
    let message = String::from_utf8_lossy(&bytes[1..]).into_owned();
    return Err(message);
  }
  Ok(bytes)
}

/// Compile, load and run `source` on a fresh Luau VM, mirroring the reference
/// `luau` CLI (`luau_run` driver): a fresh state with the standard library open,
/// the chunk loaded into a new thread, and `lua_resume` to execute it.
///
/// Returns `Ok(())` if the script ran to completion, or `Err(message)` carrying
/// the Lua error string (the same text the CLI would print) on a compile, load
/// or runtime error.
pub fn eval(source: &str) -> result::Result<(), String> {
  use ulua_vm::functions::{
    lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_newthread::lua_newthread,
    lua_resume::lua_resume, lua_tolstring::lua_tolstring, luau_load::luau_load,
  };

  let bytecode = compile(source)?;

  // v11+ bytecode needs the default Luau flags enabled (matches the CLI's
  // setLuauFlagsDefault(true)).
  ulua_common::set_all_flags(true);

  unsafe {
    let l = lua_l_newstate();
    if l.is_null() {
      return Err("lua_l_newstate returned null".to_string());
    }
    lua_l_openlibs(l);

    // Run on a fresh thread, like CLI/src/Repl.cpp's runCode.
    let t = lua_newthread(l);
    if t.is_null() {
      return Err("lua_newthread returned null".to_string());
    }

    let rc = luau_load(
      t,
      c"=eval".as_ptr(),
      bytecode.as_ptr() as *const c_char,
      bytecode.len(),
      0,
    );
    if rc != 0 {
      return Err(format!("luau_load failed: rc={rc}"));
    }

    let status = lua_resume(t, null_mut(), 0);
    if status != 0 {
      // The error object is on top of T's stack; recover its text.
      let mut len = 0usize;
      let s = lua_tolstring(t, -1, &mut len);
      let msg = if s.is_null() {
        "<non-string error>".to_string()
      } else {
        let bytes = from_raw_parts(s as *const u8, len);
        String::from_utf8_lossy(bytes).into_owned()
      };
      return Err(msg);
    }
  }

  Ok(())
}
