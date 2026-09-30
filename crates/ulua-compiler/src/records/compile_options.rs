//! 编译选项结构，语义对应 cpp `lua_CompileOptions`（`Compiler/include/luacode.h:23`）
//! 的输入面，但以惯用 Rust 类型建模：字符串选项为 `String`/`Vec<String>` 自有字段，
//! 不再有 `*const c_char` / `*const *const c_char` 裸指针链。头注释里的 `// default=`
//! 即本类型 [`Default`] 的唯一真源。
//!
//! 本仓不存在消费该结构的 C ABI 边界（`ulua-capi` 不暴露 `lua_CompileOptions`），
//! 故不设 `#[repr(C)]` 镜像；真正的 C 串转换收敛在各自的宿主边界（如 `ulua-capi`
//! 的入参解码处），选项本体只讲 Rust 类型。

use alloc::{string::String, vec::Vec};

use crate::type_aliases::{
  library_member_constant_callback::LibraryMemberConstantCallback,
  library_member_type_callback::LibraryMemberTypeCallback,
};

/// 编译选项结构，语义对应 cpp `lua_CompileOptions`（`Compiler/include/luacode.h:23`）。
/// 头注释里的 `// default=` 即本类型 [`Default`] 的唯一真源。
///
/// 字符串选项全部为自有 Rust 值：`vector_*` 是 `Option<String>`（None 即 cpp 的
/// null 缺省），名单类字段是 `Vec<String>`（空表即 cpp 的 null 数组早退）。
/// 宿主回调维持既有 `*const u8` C 形签名（cpp `luauLibraryTypeLookup` 同形），
/// 由宿主在其边界自行解码。
#[derive(Debug, Clone)]
pub struct CompileOptions {
  /// `luacode.h` `// default=1`
  pub optimization_level: i32,
  /// `luacode.h` `// default=1`
  pub debug_level: i32,
  /// `luacode.h` `// default=0`
  pub type_info_level: i32,
  /// `luacode.h` `// default=0`
  pub coverage_level: i32,

  /// 宿主注册的向量库名（缺省 None）
  pub vector_lib: Option<String>,
  /// 宿主注册的向量构造器名（缺省 None）
  pub vector_ctor: Option<String>,
  /// 宿主注册的向量类型名（缺省 None）
  pub vector_type: Option<String>,

  /// 可变全局名单（空表即 cpp 的 null 数组）
  pub mutable_globals: Vec<String>,
  /// userdata 类型名单（空表即 cpp 的 null 数组）
  pub userdata_types: Vec<String>,
  /// 已知成员库名单（空表即 cpp 的 null 数组）
  pub libraries_with_known_members: Vec<String>,
  pub library_member_type_cb: LibraryMemberTypeCallback,
  pub library_member_constant_cb: LibraryMemberConstantCallback,
  /// 禁用内置函数名单（空表即 cpp 的 null 数组）
  pub disabled_builtins: Vec<String>,
}

/// `luacode.h:28/33/39/44` 的 `// default=` 注释即缺省值；其余字段缺省为空
/// （None / 空表 / None 回调）。`vectorPrecision`（`luacode.h:55`）本端口未
/// 建模，故不出现在此处。
impl Default for CompileOptions {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

/// 把迭代器里的元素统一收成 `String` 名单。
fn name_list<S: Into<String>>(names: impl IntoIterator<Item = S>) -> Vec<String> {
  names.into_iter().map(Into::into).collect()
}

impl CompileOptions {
  /// 创建缺省编译选项配置（`optimization_level = 1`, `debug_level = 1`, 其余为 0 / 空）
  #[inline]
  pub fn new() -> Self {
    Self {
      optimization_level: 1,
      debug_level: 1,
      type_info_level: 0,
      coverage_level: 0,
      vector_lib: None,
      vector_ctor: None,
      vector_type: None,
      mutable_globals: Vec::new(),
      userdata_types: Vec::new(),
      libraries_with_known_members: Vec::new(),
      library_member_type_cb: None,
      library_member_constant_cb: None,
      disabled_builtins: Vec::new(),
    }
  }

  /// 设置优化级别（0..=2）
  ///
  /// - 0: 不进行优化
  /// - 1: 基线优化（缺省）
  /// - 2: 激进优化（含函数内联与更多常量折叠）
  #[inline]
  #[must_use]
  pub fn with_optimization_level(mut self, level: i32) -> Self {
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
  pub fn with_debug_level(mut self, level: i32) -> Self {
    self.debug_level = level;
    self
  }

  /// 设置类型信息级别（0..=1）
  ///
  /// - 0: 不生成类型信息（缺省）
  /// - 1: 为原生执行生成类型注解与断言
  #[inline]
  #[must_use]
  pub fn with_type_info_level(mut self, level: i32) -> Self {
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
  pub fn with_coverage_level(mut self, level: i32) -> Self {
    self.coverage_level = level;
    self
  }

  /// 原地配置原生编译选项（optimization_level = 2, type_info_level = 1）
  #[inline]
  pub fn set_for_native_compilation(&mut self) {
    self.optimization_level = 2;
    self.type_info_level = 1;
  }

  /// 链式启用原生编译选项（optimization_level = 2, type_info_level = 1）
  #[inline]
  #[must_use]
  pub fn with_native_compilation(mut self) -> Self {
    self.optimization_level = 2;
    self.type_info_level = 1;
    self
  }

  /// 链式设置向量三元组（库名 / 构造器名 / 类型名，None 表示未配置）
  #[inline]
  #[must_use]
  pub fn with_vector(mut self, lib: Option<&str>, ctor: Option<&str>, ty: Option<&str>) -> Self {
    self.vector_lib = lib.map(str::to_owned);
    self.vector_ctor = ctor.map(str::to_owned);
    self.vector_type = ty.map(str::to_owned);
    self
  }

  /// 链式设置向量库名
  #[inline]
  #[must_use]
  pub fn with_vector_lib(mut self, lib: Option<&str>) -> Self {
    self.vector_lib = lib.map(str::to_owned);
    self
  }

  /// 链式设置向量构造器名
  #[inline]
  #[must_use]
  pub fn with_vector_ctor(mut self, ctor: Option<&str>) -> Self {
    self.vector_ctor = ctor.map(str::to_owned);
    self
  }

  /// 链式设置向量类型名
  #[inline]
  #[must_use]
  pub fn with_vector_type(mut self, ty: Option<&str>) -> Self {
    self.vector_type = ty.map(str::to_owned);
    self
  }

  /// 链式设置可变全局名单
  #[inline]
  #[must_use]
  pub fn with_mutable_globals<S: Into<String>>(
    mut self,
    names: impl IntoIterator<Item = S>,
  ) -> Self {
    self.mutable_globals = name_list(names);
    self
  }

  /// 链式设置 userdata 类型名单
  #[inline]
  #[must_use]
  pub fn with_userdata_types<S: Into<String>>(
    mut self,
    names: impl IntoIterator<Item = S>,
  ) -> Self {
    self.userdata_types = name_list(names);
    self
  }

  /// 链式设置已知成员库名单
  #[inline]
  #[must_use]
  pub fn with_libraries_with_known_members<S: Into<String>>(
    mut self,
    names: impl IntoIterator<Item = S>,
  ) -> Self {
    self.libraries_with_known_members = name_list(names);
    self
  }

  /// 链式设置禁用内置函数名单
  #[inline]
  #[must_use]
  pub fn with_disabled_builtins<S: Into<String>>(
    mut self,
    names: impl IntoIterator<Item = S>,
  ) -> Self {
    self.disabled_builtins = name_list(names);
    self
  }
}
