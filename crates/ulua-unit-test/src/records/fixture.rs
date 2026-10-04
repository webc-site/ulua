//! Source: `tests/Fixture.h:116-211` (hand-ported, fields only — methods live
//! in their own node files under methods/fixture_*.rs)

use alloc::{boxed::Box, vec::Vec};

use ulua_analysis::records::{
  builtin_types::BuiltinTypes, frontend::Frontend, internal_error_reporter::InternalErrorReporter,
  null_module_resolver::NullModuleResolver, source_module::SourceModule, type_arena::TypeArena,
};
use ulua_ast::records::{allocator::Allocator, ast_name_table::AstNameTable};

use crate::{
  records::{test_config_resolver::TestConfigResolver, test_file_resolver::TestFileResolver},
  type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
};

/// C++ `struct Fixture` — the base test fixture every *.test.cpp derives from.
#[derive(Debug)]
pub struct Fixture {
  pub dynamic_scoped_ints: Vec<ScopedFastInt>,

  pub builtin_types: *mut BuiltinTypes,
  /// `Frontend` 是自带自引用指针（`builtin_types`、两个 module_resolver）的
  /// C++ 移植结构，契约要求「落位后不得移动」；`Box` 把它钉在堆地址上，
  /// `Fixture` 按值移动（测试里 `XxxFixture::default()` 返回值搬进调用槽）
  /// 只会移动 Box 句柄，堆内容地址与其中的自指针恒有效。
  pub frontend: Option<Box<Frontend>>,
  pub for_autocomplete: bool,

  pub has_dumped_errors: bool,

  pub arena: TypeArena,
  pub name_table: AstNameTable,
  pub allocator: Allocator,
  pub ice: InternalErrorReporter,
  /// C++ `std::unique_ptr<SourceModule>` (null until a parse happens).
  pub source_module: Option<Box<SourceModule>>,
  pub module_resolver: NullModuleResolver,
  /// `config_resolver` 以 `Box` 钉在堆地址上：`Frontend` 按 C++ 语义只存其裸
  /// 句柄（`NonNull<ConfigResolver>`，且 `TestConfigResolver` 的 C ABI 回桥以
  /// base 字段地址反推宿主），而 `Fixture` 会按值移动（`XxxFixture::default()`
  /// 返回值搬进调用槽）。钉堆后句柄在构造期一次布线即恒有效，
  /// `get_frontend` 不再需要每次访问刷新裸句柄。
  pub config_resolver: Box<TestConfigResolver>,
  /// 夹具侧句柄：可变状态全部经 `Rc` 共享槽承载（见 `TestFileResolver` 注）。
  /// `get_frontend` 把克隆移交 `Frontend` 独占持有后，本句柄与 frontend 内
  /// 实例读写同一份表，测试对 `file_resolver.source` 等既有访问形态不变。
  pub file_resolver: TestFileResolver,

  // ScopedFastFlag members are declared first in C++, so their destructors run
  // last. Rust drops fields in declaration order, so keep them at the end.
  pub sff_debug_luau_always_show_constraint_solving_incomplete: ScopedFastFlag,
  pub sff_debug_luau_freeze_arena: ScopedFastFlag,
  pub sff_luau_better_inferred_generic_names: ScopedFastFlag,
  pub sff_luau_better_metatable_stringification: ScopedFastFlag,
}
