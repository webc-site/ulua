//! The [`Compiler`] builder. Mirrors `mlua::Compiler`.
//!
//! Luau compiles *source* to bytecode through a set of compile-time options
//! (optimization level, debug level, the `vector` library/ctor/type names used
//! to enable vector fastcalls, the set of globals treated as mutable, etc.).
//! mlua exposes these as a fluent `Compiler` builder; this is the ulua-rt
//! equivalent, built directly on `ulua_compiler`'s [`CompileOptions`].
//!
//! A [`Compiler`] owns the C strings backing the pointer fields of
//! `CompileOptions` (the VM reads raw `*const c_char` pointers), so it must be
//! kept alive while a chunk built with it is compiled. [`Chunk::set_compiler`]
//! / [`Lua::set_compiler`] store it for exactly that scope.

use core::{
  ffi::{c_char, c_int},
  ptr::null,
};

use ulua_compiler::records::compile_options::CompileOptions;

/// 把编译选项名（vector 库/构造器/类型名、可变全局名）转成 NUL 结尾 `Vec<u8>`
/// 字节串（`CompileOptions` 的 `*const c_char` 字段在 `to_options` 收口点
/// `.as_ptr().cast()`）。这些名字理论上是合法标识符、绝不含 NUL，含 NUL 属调用方
/// bug：`debug_assert!` 在 debug 构建当场暴露；release 下安全降级为 `None`
/// （跳过该选项），与旧的 `.ok()` 行为一致，绝不 panic、不吞下后续路径。
fn option_name(name: impl AsRef<[u8]>) -> Option<Vec<u8>> {
  let bytes = name.as_ref();
  debug_assert!(
    !bytes.contains(&0),
    "compiler option name must not contain a NUL byte"
  );
  if bytes.contains(&0) {
    return None;
  }
  let mut v = Vec::with_capacity(bytes.len() + 1);
  v.extend_from_slice(bytes);
  v.push(0);
  Some(v)
}

/// Luau bytecode compiler options, mirroring `mlua::Compiler`.
///
/// Construct with [`Compiler::new`], tune with the `set_*` builder methods, and
/// attach to a chunk with [`Chunk::set_compiler`](crate::Chunk::set_compiler)
/// (or to the whole VM with [`Lua::set_compiler`](crate::Lua::set_compiler)).
///
/// ```
/// use ulua_rt::Compiler;
/// let _c = Compiler::new()
///     .set_optimization_level(2)
///     .set_debug_level(1)
///     .set_vector_ctor("vector");
/// ```
#[derive(Debug, Clone)]
pub struct Compiler {
  optimization_level: u8,
  debug_level: u8,
  type_info_level: u8,
  coverage_level: u8,
  vector_lib: Option<Vec<u8>>,
  vector_ctor: Option<Vec<u8>>,
  vector_type: Option<Vec<u8>>,
  mutable_globals: Vec<Vec<u8>>,
}

impl Default for Compiler {
  fn default() -> Self {
    Self::new()
  }
}

impl Compiler {
  /// Create a `Compiler` with Luau's default options (optimization level 1,
  /// debug level 1). Mirrors `mlua::Compiler::new`.
  pub fn new() -> Self {
    let defaults = CompileOptions::default();
    Compiler {
      optimization_level: defaults.optimization_level as u8,
      debug_level: defaults.debug_level as u8,
      type_info_level: defaults.type_info_level as u8,
      coverage_level: defaults.coverage_level as u8,
      vector_lib: None,
      vector_ctor: None,
      vector_type: None,
      mutable_globals: Vec::new(),
    }
  }

  /// Set the optimization level (0..=2). Mirrors
  /// `mlua::Compiler::set_optimization_level`.
  pub fn set_optimization_level(mut self, level: u8) -> Self {
    self.optimization_level = level;
    self
  }

  /// Set the debug level (0..=2). Mirrors `mlua::Compiler::set_debug_level`.
  pub fn set_debug_level(mut self, level: u8) -> Self {
    self.debug_level = level;
    self
  }

  /// Set the type-info level. Mirrors `mlua::Compiler::set_type_info_level`.
  pub fn set_type_info_level(mut self, level: u8) -> Self {
    self.type_info_level = level;
    self
  }

  /// Set the coverage level (0..=2). Mirrors
  /// `mlua::Compiler::set_coverage_level`.
  pub fn set_coverage_level(mut self, level: u8) -> Self {
    self.coverage_level = level;
    self
  }

  /// Set the library name whose members are recognized as vector operations
  /// (enables vector fastcalls). Mirrors `mlua::Compiler::set_vector_lib`.
  pub fn set_vector_lib(mut self, lib: impl AsRef<[u8]>) -> Self {
    self.vector_lib = option_name(lib.as_ref());
    self
  }

  /// Set the constructor name used to build vectors (enables compiling
  /// `vector(...)` calls to the `vector` type). Mirrors
  /// `mlua::Compiler::set_vector_ctor`.
  pub fn set_vector_ctor(mut self, ctor: impl AsRef<[u8]>) -> Self {
    self.vector_ctor = option_name(ctor.as_ref());
    self
  }

  /// Set the user vector *type* name used for field/method fastcalls.
  /// Mirrors `mlua::Compiler::set_vector_type`.
  pub fn set_vector_type(mut self, ty: impl AsRef<[u8]>) -> Self {
    self.vector_type = option_name(ty.as_ref());
    self
  }

  /// Set the list of globals the compiler is allowed to treat as mutable
  /// (so it won't constant-fold reads of them). Mirrors
  /// `mlua::Compiler::set_mutable_globals`.
  pub fn set_mutable_globals(mut self, globals: Vec<String>) -> Self {
    self.mutable_globals = globals.into_iter().filter_map(option_name).collect();
    self
  }

  /// Build a [`CompileOptions`] view over `self`. The returned options borrow
  /// the C strings owned by `self`, so `self` must outlive the compile call.
  ///
  /// The `null`-terminated pointer array for `mutable_globals` is built into
  /// `scratch`, which must also outlive the returned options.
  pub(crate) fn to_options<'a>(&'a self, scratch: &'a mut Vec<*const c_char>) -> CompileOptions {
    let mut options = CompileOptions {
      optimization_level: self.optimization_level as c_int,
      debug_level: self.debug_level as c_int,
      type_info_level: self.type_info_level as c_int,
      coverage_level: self.coverage_level as c_int,
      ..Default::default()
    };
    if let Some(s) = &self.vector_lib {
      options.vector_lib = s.as_ptr().cast();
    }
    if let Some(s) = &self.vector_ctor {
      options.vector_ctor = s.as_ptr().cast();
    }
    if let Some(s) = &self.vector_type {
      options.vector_type = s.as_ptr().cast();
    }
    if !self.mutable_globals.is_empty() {
      scratch.clear();
      for g in &self.mutable_globals {
        scratch.push(g.as_ptr().cast());
      }
      scratch.push(null());
      options.mutable_globals = scratch.as_ptr();
    }
    options
  }
}
