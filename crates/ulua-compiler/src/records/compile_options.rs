//! 编译选项结构，cpp `lua_CompileOptions`（`Compiler/include/luacode.h:23`）的
//! `#[repr(C)]` 镜像。头注释里的 `// default=` 即本类型 [`Default`] 的唯一真源。
//!
//! 字段保持 `*const c_char` / `*const *const c_char` / `extern "C-unwind"` 回调
//! 不是内部实现的选择，而是 C ABI 契约本身：宿主（含本仓 ulua-rt、各 CLI 与
//! conformance 测试）以 C 串指针数组/回调构造并读取这些字段。裸指针在这里是
//! 边界数据形态而非借用，空值一律以“null → 缺省”归一，见各消费点门面。

use core::{ffi::c_char, ptr::null, str::from_utf8};

use ulua_common::functions::c_str::cstr_bytes;

use crate::{
  functions::cstr_ptr_array::cstr_ptr_array,
  type_aliases::{
    library_member_constant_callback::LibraryMemberConstantCallback,
    library_member_type_callback::LibraryMemberTypeCallback,
  },
};

/// 编译选项结构，cpp `lua_CompileOptions`（`Compiler/include/luacode.h:23`）的
/// `#[repr(C)]` 镜像。头注释里的 `// default=` 即本类型 [`Default`] 的唯一真源。
///
/// 字段保持 `*const c_char` / `*const *const c_char` / `extern "C-unwind"` 回调
/// 不是内部实现的选择，而是 C ABI 契约本身：宿主（含本仓 ulua-rt、各 CLI 与
/// conformance 测试）以 C 串指针数组/回调构造并读取这些字段。裸指针在这里是
/// 边界数据形态而非借用，空值一律以“null → 缺省”归一，见各消费点门面。
///
/// # 安全不变式 (Safety Invariants)
///
/// 当非空时，各指针字段必须满足以下生命周期与内存有效性契约：
/// - `vector_lib`, `vector_ctor`, `vector_type`:
///   非空时必须指向以 ASCII NUL（`\0`）结尾的有效 C 字符串，且在整个编译调用期间有效。
/// - `mutable_globals`, `userdata_types`, `libraries_with_known_members`, `disabled_builtins`:
///   非空时必须指向一个以 null 指针作为终止哨兵的指针数组（`const char* const*`），
///   数组中每个非 null 元素都必须指向以 ASCII NUL 结尾、在编译调用期间有效的 C 字符串。
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CompileOptions {
  /// `luacode.h` `// default=1`
  pub optimization_level: i32,
  /// `luacode.h` `// default=1`
  pub debug_level: i32,
  /// `luacode.h` `// default=0`
  pub type_info_level: i32,
  /// `luacode.h` `// default=0`
  pub coverage_level: i32,

  /// 宿主注册的向量库名（`const char*`，null 表示未配置）
  pub vector_lib: *const c_char,
  /// 宿主注册的向量构造器名
  pub vector_ctor: *const c_char,
  /// 宿主注册的向量类型名
  pub vector_type: *const c_char,

  /// null 结尾的 `const char* const*` 可变全局名单
  pub mutable_globals: *const *const c_char,
  /// null 结尾的 `const char* const*` userdata 类型名单
  pub userdata_types: *const *const c_char,
  /// null 结尾的 `const char* const*` 已知成员库名单
  pub libraries_with_known_members: *const *const c_char,
  pub library_member_type_cb: LibraryMemberTypeCallback,
  pub library_member_constant_cb: LibraryMemberConstantCallback,
  /// null 结尾的 `const char* const*` 禁用内置函数名单
  pub disabled_builtins: *const *const c_char,
}

/// `luacode.h:28/33/39/44` 的 `// default=` 注释即缺省值；其余字段是 C 串指针/
/// 回调，缺省为空（null / None）。`vectorPrecision`（`luacode.h:55`）本端口未
/// 建模，故不出现在此处。
impl Default for CompileOptions {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl CompileOptions {
  /// 创建缺省编译选项配置（`optimization_level = 1`, `debug_level = 1`, 其余为 0 / null）
  #[inline]
  pub const fn new() -> Self {
    Self {
      optimization_level: 1,
      debug_level: 1,
      type_info_level: 0,
      coverage_level: 0,
      vector_lib: null(),
      vector_ctor: null(),
      vector_type: null(),
      mutable_globals: null(),
      userdata_types: null(),
      libraries_with_known_members: null(),
      library_member_type_cb: None,
      library_member_constant_cb: None,
      disabled_builtins: null(),
    }
  }

  /// 设置优化级别（0..=2）
  ///
  /// - 0: 不进行优化
  /// - 1: 基线优化（缺省）
  /// - 2: 激进优化（含函数内联与更多常量折叠）
  #[inline]
  #[must_use]
  pub const fn with_optimization_level(mut self, level: i32) -> Self {
    self.optimization_level = level;
    self
  }

  /// 设置调试级别（0..=2）
  ///
  /// - 0: 无调试信息
  /// - 1: 行号信息（缺省）
  /// - 2: 行号与局部变量名调试信息
  #[inline]
  #[must_use]
  pub const fn with_debug_level(mut self, level: i32) -> Self {
    self.debug_level = level;
    self
  }

  /// 设置类型信息级别（0..=1）
  ///
  /// - 0: 不生成类型信息（缺省）
  /// - 1: 为原生执行生成类型注解与断言
  #[inline]
  #[must_use]
  pub const fn with_type_info_level(mut self, level: i32) -> Self {
    self.type_info_level = level;
    self
  }

  /// 设置覆盖率级别（0..=2）
  ///
  /// - 0: 不收集覆盖率（缺省）
  /// - 1: 函数级覆盖率
  /// - 2: 基本块/行级覆盖率
  #[inline]
  #[must_use]
  pub const fn with_coverage_level(mut self, level: i32) -> Self {
    self.coverage_level = level;
    self
  }

  /// 原地配置原生编译选项（optimization_level = 2, type_info_level = 1）
  #[inline]
  pub const fn set_for_native_compilation(&mut self) {
    self.optimization_level = 2;
    self.type_info_level = 1;
  }

  /// 链式启用原生编译选项（optimization_level = 2, type_info_level = 1）
  #[inline]
  #[must_use]
  pub const fn with_native_compilation(mut self) -> Self {
    self.optimization_level = 2;
    self.type_info_level = 1;
    self
  }

  /// 获取向量库名（Rust 字符串切片视图，非 UTF-8 降级为 None）
  #[inline]
  pub fn vector_lib(&self) -> Option<&str> {
    self.vector_lib_bytes().and_then(|b| from_utf8(b).ok())
  }

  /// 获取向量库名原始字节切片
  #[inline]
  pub fn vector_lib_bytes(&self) -> Option<&[u8]> {
    (!self.vector_lib.is_null()).then(|| {
      // Safety: 非空 `vector_lib` 按 C ABI 契约保证指向有效且以 NUL 结尾的 C 字符串，
      // 借用切片生命周期受限于 `&self`。
      unsafe { cstr_bytes(self.vector_lib) }
    })
  }

  /// 获取向量构造器名（Rust 字符串切片视图，非 UTF-8 降级为 None）
  #[inline]
  pub fn vector_ctor(&self) -> Option<&str> {
    self.vector_ctor_bytes().and_then(|b| from_utf8(b).ok())
  }

  /// 获取向量构造器名原始字节切片
  #[inline]
  pub fn vector_ctor_bytes(&self) -> Option<&[u8]> {
    (!self.vector_ctor.is_null()).then(|| {
      // Safety: 非空 `vector_ctor` 按 C ABI 契约保证指向有效且以 NUL 结尾的 C 字符串，
      // 借用切片生命周期受限于 `&self`。
      unsafe { cstr_bytes(self.vector_ctor) }
    })
  }

  /// 获取向量类型名（Rust 字符串切片视图，非 UTF-8 降级为 None）
  #[inline]
  pub fn vector_type(&self) -> Option<&str> {
    self.vector_type_bytes().and_then(|b| from_utf8(b).ok())
  }

  /// 获取向量类型名原始字节切片
  #[inline]
  pub fn vector_type_bytes(&self) -> Option<&[u8]> {
    (!self.vector_type.is_null()).then(|| {
      // Safety: 非空 `vector_type` 按 C ABI 契约保证指向有效且以 NUL 结尾的 C 字符串，
      // 借用切片生命周期受限于 `&self`。
      unsafe { cstr_bytes(self.vector_type) }
    })
  }

  /// 便捷设置向量配置（输入为以 NUL 结尾的静态字节串切片，如 `b"Vector3\0"`）
  ///
  /// # 安全契约
  /// 传入切片非空时必须以 ASCII NUL（`\0`）结尾，且在整个编译调用期间有效。
  #[inline]
  pub const fn set_vector(
    &mut self,
    lib: Option<&'static [u8]>,
    ctor: Option<&'static [u8]>,
    ty: Option<&'static [u8]>,
  ) -> &mut Self {
    self.vector_lib = match lib {
      Some(s) => s.as_ptr().cast(),
      None => null(),
    };
    self.vector_ctor = match ctor {
      Some(s) => s.as_ptr().cast(),
      None => null(),
    };
    self.vector_type = match ty {
      Some(s) => s.as_ptr().cast(),
      None => null(),
    };
    self
  }

  /// 链式设置向量配置（输入为以 NUL 结尾的静态字节串切片，如 `b"Vector3\0"`）
  ///
  /// # 安全契约
  /// 传入切片非空时必须以 ASCII NUL（`\0`）结尾，且在整个编译调用期间有效。
  #[inline]
  #[must_use]
  pub const fn with_vector(
    mut self,
    lib: Option<&'static [u8]>,
    ctor: Option<&'static [u8]>,
    ty: Option<&'static [u8]>,
  ) -> Self {
    let _ = self.set_vector(lib, ctor, ty);
    self
  }

  /// 链式设置向量库名（输入为以 NUL 结尾的静态字节串切片，如 `b"Vector3\0"`）
  #[inline]
  #[must_use]
  pub const fn with_vector_lib(mut self, lib: Option<&'static [u8]>) -> Self {
    self.vector_lib = match lib {
      Some(s) => s.as_ptr().cast(),
      None => null(),
    };
    self
  }

  /// 链式设置向量构造器名（输入为以 NUL 结尾的静态字节串切片，如 `b"new\0"`）
  #[inline]
  #[must_use]
  pub const fn with_vector_ctor(mut self, ctor: Option<&'static [u8]>) -> Self {
    self.vector_ctor = match ctor {
      Some(s) => s.as_ptr().cast(),
      None => null(),
    };
    self
  }

  /// 链式设置向量类型名（输入为以 NUL 结尾的静态字节串切片，如 `b"Vector3\0"`）
  #[inline]
  #[must_use]
  pub const fn with_vector_type(mut self, ty: Option<&'static [u8]>) -> Self {
    self.vector_type = match ty {
      Some(s) => s.as_ptr().cast(),
      None => null(),
    };
    self
  }

  /// 链式设置可变全局名单（`const char* const*`，以 null 指针槽结尾）
  ///
  /// # 安全契约
  /// 若非空，必须满足 [`CompileOptions`] 的可变全局名单安全不变式。
  #[inline]
  #[must_use]
  pub const fn with_mutable_globals(mut self, mutable_globals: *const *const c_char) -> Self {
    self.mutable_globals = mutable_globals;
    self
  }

  /// 链式设置 userdata 类型名单（`const char* const*`，以 null 指针槽结尾）
  ///
  /// # 安全契约
  /// 若非空，必须满足 [`CompileOptions`] 的 userdata 类型名单安全不变式。
  #[inline]
  #[must_use]
  pub const fn with_userdata_types(mut self, userdata_types: *const *const c_char) -> Self {
    self.userdata_types = userdata_types;
    self
  }

  /// 链式设置已知成员库名单（`const char* const*`，以 null 指针槽结尾）
  ///
  /// # 安全契约
  /// 若非空，必须满足 [`CompileOptions`] 的已知成员库名单安全不变式。
  #[inline]
  #[must_use]
  pub const fn with_libraries_with_known_members(
    mut self,
    libraries_with_known_members: *const *const c_char,
  ) -> Self {
    self.libraries_with_known_members = libraries_with_known_members;
    self
  }

  /// 链式设置禁用内置函数名单（`const char* const*`，以 null 指针槽结尾）
  ///
  /// # 安全契约
  /// 若非空，必须满足 [`CompileOptions`] 的禁用内置函数名单安全不变式。
  #[inline]
  #[must_use]
  pub const fn with_disabled_builtins(mut self, disabled_builtins: *const *const c_char) -> Self {
    self.disabled_builtins = disabled_builtins;
    self
  }

  /// 链式设置库成员类型解析回调
  #[inline]
  #[must_use]
  pub const fn with_library_member_type_cb(mut self, cb: LibraryMemberTypeCallback) -> Self {
    self.library_member_type_cb = cb;
    self
  }

  /// 链式设置库成员常量解析回调
  #[inline]
  #[must_use]
  pub const fn with_library_member_constant_cb(
    mut self,
    cb: LibraryMemberConstantCallback,
  ) -> Self {
    self.library_member_constant_cb = cb;
    self
  }

  /// 可变全局名单字节切片迭代器
  #[inline]
  pub fn mutable_globals(&self) -> impl Iterator<Item = &[u8]> {
    cstr_ptr_array(self.mutable_globals)
  }

  /// userdata 类型名单字节切片迭代器
  #[inline]
  pub fn userdata_types(&self) -> impl Iterator<Item = &[u8]> {
    cstr_ptr_array(self.userdata_types)
  }

  /// 已知成员库名单字节切片迭代器
  #[inline]
  pub fn libraries_with_known_members(&self) -> impl Iterator<Item = &[u8]> {
    cstr_ptr_array(self.libraries_with_known_members)
  }

  /// 禁用内置函数名单字节切片迭代器
  #[inline]
  pub fn disabled_builtins(&self) -> impl Iterator<Item = &[u8]> {
    cstr_ptr_array(self.disabled_builtins)
  }
}
