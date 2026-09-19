//! The [`Chunk`] builder returned by [`Lua::load`]. Mirrors `mlua::Chunk`.
//!
//! Compilation reuses the same machinery as the umbrella `ulua` crate's
//! `compile`/`eval` helpers: source -> `ulua_compiler::compile` -> bytecode ->
//! `luau_load` -> a Lua function on the stack -> `lua_pcall`.

use core::ffi::c_char;
use std::{ffi::CString, mem};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_compiler::{
  functions::compile::compile as compiler_compile, records::compile_options::CompileOptions,
};

use crate::{
  compiler::Compiler,
  error::{Error, Result},
  function::Function,
  state::Lua,
  sys::*,
  table::Table,
  traits::{FromLuaMulti, IntoLuaMulti},
};

/// A not-yet-executed piece of Lua source.
///
/// Mirrors `mlua::Chunk`. Produced by [`Lua::load`]; finalized with
/// [`Chunk::exec`], [`Chunk::eval`], or [`Chunk::into_function`].
pub struct Chunk {
  pub(crate) lua: Lua,
  pub(crate) source: String,
  pub(crate) name: String,
  /// Optional environment table applied to the loaded function. Mirrors the
  /// per-chunk environment set by `mlua::Chunk::set_environment`.
  pub(crate) environment: Option<Table>,
  /// Optional per-chunk compiler. Mirrors `mlua::Chunk::set_compiler`. When
  /// `None`, the VM-default compiler (`Lua::set_compiler`) is used, falling
  /// back to ulua's default options.
  pub(crate) compiler: Option<Compiler>,
}

impl Chunk {
  /// Override the chunk name shown in error messages / tracebacks.
  ///
  /// Mirrors `mlua::Chunk::set_name`.
  pub fn set_name(mut self, name: impl Into<String>) -> Self {
    self.name = name.into();
    self
  }

  /// Set the environment (globals table) the loaded chunk runs in.
  ///
  /// Mirrors `mlua::Chunk::set_environment`. Applied to the function produced
  /// by [`Chunk::into_function`] / [`Chunk::exec`] / [`Chunk::eval`].
  pub fn set_environment(mut self, env: Table) -> Self {
    self.environment = Some(env);
    self
  }

  /// Set the [`Compiler`](crate::Compiler) used to compile this chunk.
  /// Mirrors `mlua::Chunk::set_compiler`.
  pub fn set_compiler(mut self, compiler: Compiler) -> Self {
    self.compiler = Some(compiler);
    self
  }

  /// Compile the chunk and call it with `args`, converting the result to `R`.
  /// Mirrors `mlua::Chunk::call`.
  pub fn call<R: FromLuaMulti>(self, args: impl IntoLuaMulti) -> Result<R> {
    self.into_function()?.call::<R>(args)
  }

  /// The current chunk mode is always text here (ulua-rt loads source).
  /// Mirrors `mlua::Chunk::set_mode` as a no-op accepting `mlua::ChunkMode`'s
  /// role: ulua-rt auto-detects, so this is provided only for signature
  /// parity and returns `self` unchanged.
  pub fn set_mode(self, _mode: ChunkMode) -> Self {
    self
  }

  /// The chunk name used for error messages / tracebacks.
  ///
  /// Mirrors `mlua::Chunk::name`.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Compile the source to bytecode (or return a [`Error::SyntaxError`]).
  fn compile(&self) -> Result<Vec<u8>> {
    // Pick the effective compiler: a per-chunk one wins over the VM-default
    // one (`Lua::set_compiler`); otherwise use ulua's default options.
    // 借用分支直取，chunk 自带 compiler 时不再克隆整份选项。
    let mut scratch: Vec<*const c_char> = Vec::new();
    let options = match self.compiler.as_ref() {
      Some(c) => c.to_options(&mut scratch),
      // VM 默认编译器存于进程级 side-store，取回必经一次克隆。
      None => match self.lua.vm_compiler() {
        Some(c) => c.to_options(&mut scratch),
        None => CompileOptions::default(),
      },
    };
    let parse_options = ParseOptions::default();
    let bytes = compiler_compile(&self.source, &options, &parse_options, NoopEncoder);
    // A leading 0 byte is the compiler's error marker.
    if bytes.first() == Some(&0u8) {
      let message = String::from_utf8_lossy(&bytes[1..]).into_owned();
      // Luau reports a syntax error caused by hitting end-of-input (an
      // unterminated block/expression) with a "got <eof>" suffix; mlua
      // surfaces that as `incomplete_input: true` so a REPL can keep
      // reading. Detect it the same way.
      let incomplete_input = message.contains("<eof>");
      return Err(Error::SyntaxError {
        message,
        incomplete_input,
      });
    }
    Ok(bytes)
  }

  /// Load the compiled chunk and leave the resulting function on top of the
  /// stack, returning a [`Function`] handle.
  ///
  /// Mirrors `mlua::Chunk::into_function`.
  pub fn into_function(self) -> Result<Function> {
    let bytecode = self.compile()?;
    self.load_bytecode(bytecode)
  }

  /// 加载已编译字节码：`luau_load` → 栈顶函数 → 环境挂接 → [`Function`] 句柄。
  /// [`Chunk::into_function`] 与 [`Chunk::eval`] / [`Chunk::eval_async`] 的
  /// 表达式优先路径共用（后者已持有字节码，避免二次编译）。
  fn load_bytecode(self, bytecode: Vec<u8>) -> Result<Function> {
    let state = self.lua.state();
    unsafe {
      // 名字含 NUL 时回退到固定占位名（常量字面量不可能含 NUL）。
      let chunkname =
        CString::new(format!("={}", self.name)).unwrap_or_else(|_| c"=chunk".to_owned());
      let rc = luau_load(
        state,
        chunkname.as_ptr(),
        bytecode.as_ptr() as *const c_char,
        bytecode.len(),
        0,
      );
      if rc != 0 {
        // luau_load failure leaves an error message on the stack.
        return Err(self.lua.pop_error(rc));
      }
      let func = Function::from_ref(self.lua.pop_ref());
      if let Some(env) = &self.environment {
        func.set_environment(env.clone())?;
      }
      Ok(func)
    }
  }

  /// Statically type-check this chunk's source against the owning [`Lua`]'s
  /// accumulated host definitions (the `typecheck` feature).
  ///
  /// Returns `Ok(())` when the source type-checks clean, or
  /// [`Error::TypeError`](crate::Error::TypeError) carrying the structured
  /// diagnostics otherwise. Because Luau is dynamically typed, the check is
  /// advisory — it composes with `?` ahead of [`Chunk::exec`] / [`Chunk::eval`]
  /// without changing what running the chunk does:
  ///
  /// ```
  /// # #[cfg(feature = "typecheck")] {
  /// # use ulua_rt::Lua;
  /// let lua = Lua::new();
  /// let c = lua.load("local x: number = 1\nreturn x");
  /// c.check().unwrap();
  /// # }
  /// ```
  #[cfg(feature = "typecheck")]
  #[cfg_attr(docsrs, doc(cfg(feature = "typecheck")))]
  pub fn check(&self) -> Result<()> {
    self.lua.check(&self.source)
  }

  /// Run the chunk for its side effects, discarding return values.
  ///
  /// Mirrors `mlua::Chunk::exec`.
  pub fn exec(self) -> Result<()> {
    let f = self.into_function()?;
    f.call::<()>(())
  }

  /// 表达式优先加载（[`Chunk::eval`] / [`Chunk::eval_async`] 共用）：先把源码改写
  /// 为 `return <src>` 试编译一次，命中直接加载该字节码；失败则恢复原源码走
  /// 语句块路径。改写原地进行，不克隆整块 [`Chunk`]。
  fn into_function_expr_first(mut self) -> Result<Function> {
    let source = mem::take(&mut self.source);
    self.source = format!("return {source}");
    let expression = self.compile().ok();
    self.source = source;
    match expression {
      Some(bytecode) => self.load_bytecode(bytecode),
      None => self.into_function(),
    }
  }

  /// Run the chunk and convert its return value(s) to `R`.
  ///
  /// Mirrors `mlua::Chunk::eval`. Like the Lua REPL (and mlua), the source is
  /// first tried as an *expression* (by prepending `return `); if that
  /// compiles it is used, otherwise the chunk is run as a statement block.
  /// This is what lets `lua.load("coroutine.create(f)").eval::<Thread>()` and
  /// `lua.load("function() ... end").eval::<Function>()` work.
  pub fn eval<R: FromLuaMulti>(self) -> Result<R> {
    self.into_function_expr_first()?.call::<R>(())
  }

  /// Asynchronously load the chunk and call it with `args` (the `async`
  /// feature). Mirrors `mlua::Chunk::call_async`.
  #[cfg(feature = "async")]
  #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
  pub async fn call_async<R>(self, args: impl IntoLuaMulti) -> Result<R>
  where
    R: FromLuaMulti,
  {
    self.into_function()?.call_async(args).await
  }

  /// Asynchronously run the chunk for its side effects (the `async` feature).
  /// Mirrors `mlua::Chunk::exec_async`.
  #[cfg(feature = "async")]
  #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
  pub async fn exec_async(self) -> Result<()> {
    self.call_async(()).await
  }

  /// Asynchronously evaluate the chunk as an expression (or block) and convert
  /// the result to `R` (the `async` feature). Mirrors `mlua::Chunk::eval_async`.
  ///
  /// Like [`Chunk::eval`], the source is first tried as an expression (by
  /// prepending `return `); if that compiles it is driven, otherwise the chunk
  /// runs as a statement block.
  #[cfg(feature = "async")]
  #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
  pub async fn eval_async<R: FromLuaMulti>(self) -> Result<R> {
    self.into_function_expr_first()?.call_async::<R>(()).await
  }
}

/// How a chunk's bytes are interpreted. Mirrors `mlua::ChunkMode`. ulua-rt
/// always loads text source, so this exists for signature parity with
/// [`Chunk::set_mode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkMode {
  /// Text source (the default and only supported mode here).
  Text,
  /// Precompiled bytecode (not supported by ulua-rt's high-level loader).
  Binary,
}
