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
  fn default() -> Self {
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
}

impl CompileOptions {
  pub fn set_for_native_compilation(&mut self) {
    self.optimization_level = 2;
    self.type_info_level = 1;
  }

  /// 获取向量库名（Rust 字符串切片视图，非 UTF-8 降级为 None）
  #[inline]
  pub fn vector_lib(&self) -> Option<&str> {
    self.vector_lib_bytes().and_then(|b| from_utf8(b).ok())
  }

  /// 获取向量库名原始字节切片
  #[inline]
  pub fn vector_lib_bytes(&self) -> Option<&[u8]> {
    (!self.vector_lib.is_null()).then(|| unsafe { cstr_bytes(self.vector_lib) })
  }

  /// 获取向量构造器名（Rust 字符串切片视图，非 UTF-8 降级为 None）
  #[inline]
  pub fn vector_ctor(&self) -> Option<&str> {
    self.vector_ctor_bytes().and_then(|b| from_utf8(b).ok())
  }

  /// 获取向量构造器名原始字节切片
  #[inline]
  pub fn vector_ctor_bytes(&self) -> Option<&[u8]> {
    (!self.vector_ctor.is_null()).then(|| unsafe { cstr_bytes(self.vector_ctor) })
  }

  /// 获取向量类型名（Rust 字符串切片视图，非 UTF-8 降级为 None）
  #[inline]
  pub fn vector_type(&self) -> Option<&str> {
    self.vector_type_bytes().and_then(|b| from_utf8(b).ok())
  }

  /// 获取向量类型名原始字节切片
  #[inline]
  pub fn vector_type_bytes(&self) -> Option<&[u8]> {
    (!self.vector_type.is_null()).then(|| unsafe { cstr_bytes(self.vector_type) })
  }

  /// 便捷设置向量配置（输入为以 NUL 结尾的静态字节串切片，如 `b"Vector3\0"`）
  #[inline]
  pub fn set_vector(
    &mut self,
    lib: Option<&'static [u8]>,
    ctor: Option<&'static [u8]>,
    ty: Option<&'static [u8]>,
  ) -> &mut Self {
    self.vector_lib = lib.map_or(null(), |s| s.as_ptr().cast());
    self.vector_ctor = ctor.map_or(null(), |s| s.as_ptr().cast());
    self.vector_type = ty.map_or(null(), |s| s.as_ptr().cast());
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
