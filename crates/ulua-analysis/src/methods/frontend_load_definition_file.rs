//! C++ `Frontend::loadDefinitionFile` (`Analysis/src/Frontend.cpp:218-247`) 的
//! Rust 形态。
//!
//! #24 b13-tc2-fields 目标表选择器收口：原签名把 C++ `GlobalTypes& globals` 形参
//! 直译为 `&mut GlobalTypes`，而全部 13 个调用点实际传入的都是 `Frontend.globals`
//! 或 `Frontend.globals_for_autocomplete`——「整体 + 字段」重叠借用，借用检查器
//! 无法表达「在 `&mut self` 上调方法又把它的字段借出去」，调用点因此一律降级
//! `(*frontend_ptr).load_definition_file(&mut (*frontend_ptr).globals, ..)`
//! 形态的裸指针拆借。函数体内部的两阶段借用冲突（第一阶段检查要整体
//! `&mut self`，第二阶段持久化要目标表的独占借用）亦是该直译的连带产物。
//! 本次把函数体按时间轴拆为两阶段门面，目标表改由选择器闭包在第二阶段才
//! 物化字段借用，破开两阶段借用：
//!
//! - [`Frontend::load_definition_file`]：对外单 `&mut Frontend` 形态，目标表经
//!   `FnOnce(&mut Frontend) -> &mut GlobalTypes` 选择器闭包（如
//!   `|f| &mut f.globals`）指定，调用点免裸指针、免 `unsafe`；
//! - [`Frontend::load_definition_file_into_globals`]：crate 内形态，
//!   `register_builtin_globals` 已持有拆借好的 `&mut GlobalTypes`（重叠别名
//!   chokepoint 登记于 `methods/frontend_register_builtin_globals.rs`），
//!   直接沿用该表引用。

use alloc::{boxed::Box, string::String, vec::Vec};

use ulua_ast::{enums::mode::Mode, records::parse_result::ParseResult};
use ulua_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

use crate::{
  functions::{
    parse_source_for_module::parse_source_for_module, persist_checked_types::persist_checked_types,
  },
  records::{
    frontend::{Frontend, FrontendStats},
    global_types::GlobalTypes,
    load_definition_file_result::LoadDefinitionFileResult,
    require_cycle::RequireCycle,
    source_module::SourceModule,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::{
    module_name_type::ModuleName, module_ptr_module::ModulePtr, scope_ptr_type::ScopePtr,
  },
};

/// 第二阶段（持久化 + 结果组装）：把已检查模块的类型面写入 `globals` 选定的
/// 目标作用域表，见 [`Frontend::load_definition_file`] 模块头的两阶段说明。
fn finalize_definition(
  globals: &mut GlobalTypes,
  target_scope: ScopePtr,
  parse_result: ParseResult,
  source_module: SourceModule,
  checked_module: ModulePtr,
  package_name: String,
) -> LoadDefinitionFileResult {
  if !checked_module.errors.is_empty() {
    return LoadDefinitionFileResult {
      success: false,
      parse_result,
      source_module,
      module: Some(checked_module),
    };
  }

  persist_checked_types(checked_module.clone(), globals, target_scope, package_name);

  // Retain the checked module so its `TypeArena` — which owns the types
  // just persisted into the global scope — outlives the returned
  // `LoadDefinitionFileResult`. Without this, dropping the result (e.g. at
  // the end of `register_builtin_globals`, which loads `"@luau"` twice)
  // frees the arena while the global scope still references its types: a
  // use-after-free the type checker then reads, SIGSEGVing on some
  // toolchains (issue #6). Append-only so a repeated package name does not
  // evict an earlier, still-referenced module.
  globals.retained_modules.push(checked_module.clone());

  LoadDefinitionFileResult {
    success: true,
    parse_result,
    source_module,
    module: Some(checked_module),
  }
}

impl Frontend {
  /// 目标表选择器形态（收口原 `&mut GlobalTypes` 裸拆借传参）：`select_globals`
  /// 只在检查完成后的第二阶段被调用，把目标表字段借用物化在自身作用域内，
  /// 第一阶段 `&mut self` 整体借用彼已结束——两阶段借用就此破开。
  pub fn load_definition_file(
    &mut self,
    select_globals: impl for<'a> FnOnce(&'a mut Frontend) -> &'a mut GlobalTypes,
    target_scope: ScopePtr,
    source: &str,
    package_name: String,
    capture_comments: bool,
    _type_check_for_autocomplete: bool,
  ) -> LoadDefinitionFileResult {
    LUAU_TIMETRACE_SCOPE!("loadDefinitionFile", "Frontend");

    let (parse_result, source_module, checked_module) =
      match self.prepare_definition(source, &package_name, capture_comments) {
        Err(failed) => return *failed,
        Ok(ready) => ready,
      };

    // 第二阶段才物化目标表借用（C++ 里 `globals` 形参与 `*this` 本就同源，
    // 时序拆分不改变任何写入次序）。
    let globals = select_globals(self);
    finalize_definition(
      globals,
      target_scope,
      parse_result,
      source_module,
      checked_module,
      package_name,
    )
  }

  /// crate 内形态：调用方（`register_builtin_globals`）已按 chokepoint 契约持有
  /// 拆借好的 `&mut GlobalTypes`，目标表不经选择器闭包、直接沿用该引用；
  /// 两阶段拆分与 [`Frontend::load_definition_file`] 同一实现。
  pub(crate) fn load_definition_file_into_globals(
    &mut self,
    globals: &mut GlobalTypes,
    target_scope: ScopePtr,
    source: &str,
    package_name: String,
    capture_comments: bool,
    _type_check_for_autocomplete: bool,
  ) -> LoadDefinitionFileResult {
    LUAU_TIMETRACE_SCOPE!("loadDefinitionFile", "Frontend");

    let (parse_result, source_module, checked_module) =
      match self.prepare_definition(source, &package_name, capture_comments) {
        Err(failed) => return *failed,
        Ok(ready) => ready,
      };

    finalize_definition(
      globals,
      target_scope,
      parse_result,
      source_module,
      checked_module,
      package_name,
    )
  }

  /// 第一阶段（解析 + 检查）：在 `&mut self` 独占借用内完成（C++
  /// `checkSourceModule` 同为 `*this` 方法），产出组装结果所需的三元组；
  /// 解析失败时直接返回已组装的失败结果（不经目标表）。失败结果是冷路径，
  /// 按 clippy `result_large_err` 装箱压缩 `Result` 尺寸（成功路径不受影响）。
  fn prepare_definition(
    &mut self,
    source: &str,
    package_name: &str,
    capture_comments: bool,
  ) -> Result<(ParseResult, SourceModule, ModulePtr), Box<LoadDefinitionFileResult>> {
    let mut source_module = SourceModule::new();
    source_module.name = ModuleName::from(package_name);
    source_module.human_readable_name = String::from(package_name);

    let parse_result = parse_source_for_module(source, &mut source_module, capture_comments);
    if !parse_result.errors.is_empty() {
      return Err(Box::new(LoadDefinitionFileResult {
        success: false,
        parse_result,
        source_module,
        module: None,
      }));
    }

    let mut dummy_stats = FrontendStats::default();
    let checked_module = self.check_source_module_mode_vector_require_cycle_optional_scope_ptr_bool_bool_frontend_stats_type_check_limits(
            &source_module,
            Mode::Definition,
            Vec::<RequireCycle>::new(),
            None,
            /* forAutocomplete */ false,
            /* recordJsonLog */ false,
            &mut dummy_stats,
            TypeCheckLimits::default(),
        );

    Ok((parse_result, source_module, checked_module))
  }
}
