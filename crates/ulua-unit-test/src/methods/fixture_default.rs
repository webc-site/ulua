//! `Default` for the test `Fixture` — port of `Fixture::Fixture(bool=false)`
//! (tests/Fixture.cpp:263) plus the in-class member initializers from
//! tests/Fixture.h:170-184.
//!
//! The four `ScopedFastFlag` members all default to `true` in the C++ in-class
//! initializers (`sff_DebugLuauFreezeArena{FFlag::DebugLuauFreezeArena, true}`,
//! `sff_DebugLuauAlwaysShowConstraintSolvingIncomplete{..., true}`,
//! `sff_LuauBetterMetatableStringification{..., true}` at tests/Fixture.h:186,
//! `sff_LuauBetterInferredGenericNames{..., true}` at tests/Fixture.h:187).
//!
//! NOTE on the name table / allocator: `AstNameTable::new` stores a raw
//! `*mut Allocator` into the table. We construct against the *local* `allocator`
//! and then move both into the returned struct, so that stored pointer is left
//! dangling at the struct's final address. That is intentional and safe here:
//! every parse entry point (`parse` / `try_parse` / `match_parse_error`) calls
//! `name_table.rebind_allocator(&mut self.allocator)` before interning, so the
//! table always points at the allocator at its *current* address.
use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_analysis::records::{
  internal_error_reporter::InternalErrorReporter, null_module_resolver::NullModuleResolver,
  type_arena::TypeArena,
};
use ulua_ast::records::{allocator::Allocator, ast_name_table::AstNameTable};
use ulua_common::fflag;

use crate::{
  records::{
    fixture::Fixture, test_config_resolver::TestConfigResolver,
    test_file_resolver::TestFileResolver,
  },
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};
impl Default for Fixture {
  fn default() -> Self {
    let mut allocator = Allocator::new();
    let name_table = AstNameTable::new(&mut allocator);

    Fixture {
      sff_debug_luau_freeze_arena: ScopedFastFlag::new(&fflag::DebugLuauFreezeArena, true),
      sff_debug_luau_always_show_constraint_solving_incomplete: ScopedFastFlag::new(
        &fflag::DebugLuauAlwaysShowConstraintSolvingIncomplete,
        true,
      ),
      sff_luau_better_inferred_generic_names: ScopedFastFlag::new(
        &fflag::LuauBetterInferredGenericNames,
        true,
      ),
      sff_luau_better_metatable_stringification: ScopedFastFlag::new(
        &fflag::LuauBetterMetatableStringification,
        true,
      ),
      file_resolver: Box::new(TestFileResolver::default()),
      config_resolver: Box::new(TestConfigResolver::default()),
      module_resolver: NullModuleResolver,
      source_module: None,
      ice: InternalErrorReporter::default(),
      allocator,
      name_table,
      arena: TypeArena::default(),
      has_dumped_errors: false,
      for_autocomplete: false,
      frontend: None,
      // DELIBERATE DEVIATION / 保留理由：`builtin_types` 是 Frontend 自引用指针的
      // 缓存句柄（声明在 `records::fixture::Fixture`），语义「frontend 未建」而非
      // 可空资源，读写端都不在本轮清单：唯一写入点
      // `methods::fixture_get_frontend`（`self.builtin_types =
      // frontend.builtin_types_handle().as_ptr()`，每次 get_frontend 刷新）、
      // 唯一解引用门面 `methods::fixture_get_builtins`
      // （`unsafe { &mut *self.builtin_types }`），另有 13 个文件 22 处
      // `base.builtin_types` 直读（tests/ 与 sub-fixture）。目标形态
      // `Option<NonNull<BuiltinTypes>>` + `get_builtins()` 内
      // `.expect("frontend not built")` 才能既消灭 null 初值又保住 panic 信息，
      // 需与上述两端同批改；本端单改构造即不可编译。夹具 arena 句柄字段，既有约定（review.md §2）。
      builtin_types: null_mut(),
      dynamic_scoped_ints: Vec::new(),
    }
  }
}
