//! # ulua
//!
//! A faithful Rust translation of [Luau](https://github.com/luau-lang/luau) —
//! Roblox's typed Lua. This umbrella crate re-exports the individual layers
//! (lexer/parser/AST, bytecode, compiler, register VM, type checker, config and
//! require resolution) and convenience helpers — [`compile`],
//! [`eval`], [`eval_bytecode`], and [`check`] — for the common "compile a string / run a string /
//! run bytecode / type-check a string" cases.
//!
//! For finer-grained control depend on the sub-crates directly; they are all
//! re-exported here as modules.
//!
//! ```
//! ulua::eval("assert(1 + 1 == 2)").unwrap();
//! let bytecode = ulua::compile("return 2 + 2").unwrap();
//! assert!(!bytecode.is_empty());
//! ulua::eval_bytecode(&bytecode).unwrap();
//! # #[cfg(feature = "typecheck")] {
//! ulua::check("local x: number = 1").unwrap();
//! # }
//! ```

// Re-export the sub-crates as modules so `ulua::vm::...` etc. work from one dep.
use std::{result::Result as StdResult, sync::Once};

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
use ulua_vm::records::lua_state_guard::LuaStateGuard;

/// Common entry points, re-exported for convenience.
pub mod prelude {
  pub use ulua_ast::records::parse_options::ParseOptions;
  pub use ulua_compiler::records::compile_options::CompileOptions;
  // The mlua-style high-level API prelude (Lua, Value, Table, traits, ...).
  pub use ulua_rt::prelude::*;

  /// The type-check helpers (the `typecheck` feature; on by default).
  #[cfg(feature = "typecheck")]
  pub use crate::{check, check_modules, check_modules_with_definitions, check_with_definitions};
  pub use crate::{compile, eval, eval_bytecode};
}

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_common::set_luau_bool_flags;
use ulua_compiler::{
  functions::compile::compile as compiler_compile, records::compile_options::CompileOptions,
};
use ulua_vm::{
  functions::{
    lua_l_newstate::lua_l_newstate, lua_l_openlibs::lua_l_openlibs, lua_l_sandbox::lua_l_sandbox,
    lua_l_sandboxthread::lua_l_sandboxthread, run_loaded_chunk::run_loaded_chunk,
  },
  records::lua_state::LuaState,
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

/// Compile, load and run `source`, mirroring cpp `CLI/src/Repl.cpp` 的 REPL
/// 会话：`setupState`（Repl.cpp:205-230，即 `luaL_openlibs` + `luaL_sandbox`）、
/// `runRepl` 在宿主 state 上的 `luaL_sandboxthread`（Repl.cpp:566），再接
/// `runCode` 的 load / 换协程 / resume（Repl.cpp:239-300）。On normal return,
/// leftover results are printed through `_PRETTYPRINT` (falling back to `print`),
/// exactly like the REPL.
///
/// 与 CLI `setupState` 有意未对齐的一段：`loadstring` / `collectgarbage` 注册与
/// `luaopen_require`（三者依赖 CLI 侧的 require 上下文与命令行配置，只有
/// `ulua` 二进制提供）。因此脚本里的 `require(...)` / `loadstring(...)` /
/// `collectgarbage(...)` 在 CLI 下可用、在 `eval` 下是 nil。
///
/// Returns `Ok(())` if the script ran to completion, or [`Error`] carrying the
/// Lua error text with the `lua_debugtrace` stack backtrace appended (as the
/// C++ REPL does) on a compile, load or runtime error.
pub fn eval(source: &str) -> StdResult<(), Error> {
  let bytecode = compile(source)?;
  eval_bytecode(&bytecode)
}

/// 进程内「置默认 FastFlag」一次性门（同 ulua-web `init_default_flags` 的契约）：
/// FastFlag 是进程级全局，ulua-common 要求其仅在线程启动前写入；per-call 裸写
/// 会与并发只读构成数据竞争（Rust 语义下即 UB），也会静默覆盖宿主先前配置的
/// flag。上游 `setLuauFlagsDefault()` 亦只在 CLI 入口调用一次。
static INIT_LUAU_FLAGS: Once = Once::new();

/// Load and run precompiled Luau `bytecode`, mirroring [`eval`].
///
/// Runs the precompiled bytecode in an isolated sandboxed VM state with
/// standard libraries opened.
///
/// Returns `Ok(())` if the bytecode ran to completion, or [`Error`] carrying
/// the Lua error text with stack backtrace appended.
pub fn eval_bytecode(bytecode: &[u8]) -> StdResult<(), Error> {
  INIT_LUAU_FLAGS.call_once(|| set_luau_bool_flags(true));

  // `lua_l_newstate` 为 safe 包装（内部收口 C 分配器边界），null 返回值在下一句
  // 即判出并早退，其后所有调用只作用于非空状态。
  let l = lua_l_newstate();
  if l.is_null() {
    return Err(Error::MemoryError(
      "lua_l_newstate returned null".to_string(),
    ));
  }
  // `_state` 守卫保证本帧退出时 `lua_close`，句柄不外泄。
  let _state = LuaStateGuard(l);

  // Safety: 上方 null 判定保证 `l` 是本帧独占的活跃状态机；openlibs → sandbox
  // 即 cpp `setupState`（Repl.cpp:205-230）的调用序。
  unsafe {
    lua_l_openlibs(l);
    lua_l_sandbox(l);
  }
  // Safety: 同上，冻结线程全局表（cpp runRepl 于宿主 state 的
  // luaL_sandboxthread，Repl.cpp:566）。
  unsafe { lua_l_sandboxthread(l) };
  // Safety: `run_code` 的两条前置（活跃状态机、已依次完成
  // `lua_l_openlibs`/`lua_l_sandbox`/`lua_l_sandboxthread`）恰由上三句成立，
  // 且状态关闭发生在本句之后、`_state` Drop 之时。
  unsafe { run_code(l, bytecode) }
}

/// cpp `Repl.cpp:239 runCode` 的加载/执行段薄壳（bytecode 已在外部编译）：
/// 实现整体委托 [`ulua_vm::functions::run_loaded_chunk`]（`_PRETTYPRINT` 为 nil
/// 时回退 print 的 REPL 差异经 `pretty_print_fallback = true` 打开），此处仅把
/// `Err(错误文本)` 映射为 [`Error::RuntimeError`]。
///
/// 与三方统一后的一处已知收口：cpp 对 `lua_newthread` 分配失败不设防，原
/// Rust 移植各自补救（本 crate 报 `MemoryError("lua_newthread returned null")`、
/// web 报 "not enough memory"、repl 未判空）；下沉后统一为「弹平 chunk 并返回
/// VM 标准内存错误文本」。该路径仅在进程 OOM 时可达。
/// # Safety
/// `l` 必须是由 `lua_l_newstate` 创建、已 `lua_l_openlibs` + `lua_l_sandbox` +
/// `lua_l_sandboxthread` 的有效状态机（闭包的 env 取自它的全局表）。
unsafe fn run_code(l: *mut LuaState, bytecode: &[u8]) -> StdResult<(), Error> {
  // Safety: 本函数 `# Safety` 契约保证 `l` 是已 newstate + openlibs + sandbox +
  // sandboxthread 的活跃状态机，且在调用帧结束前不会被关闭；run_loaded_chunk
  // 的前置（活跃状态机、调用前栈平衡）恰由 eval_bytecode 的建立序列成立。
  match unsafe { run_loaded_chunk(l, bytecode, true) } {
    Ok(()) => Ok(()),
    Err(message) => Err(Error::RuntimeError(message)),
  }
}
