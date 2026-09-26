use alloc::vec::Vec;
use core::ptr::{null, null_mut};

use ulua_ast::records::{ast_expr::AstExpr, ast_name::AstName, ast_stat::AstStat};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};
use ulua_config::{
  enums::code::Code,
  records::{lint_options::LintOptions, lint_warning::LintWarning},
};

use crate::{
  records::{global_linter::Global, module::Module},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

#[derive(Debug, Clone)]
pub struct LintContext {
  pub result: Vec<LintWarning>,
  pub options: LintOptions,
  pub root: *mut AstStat,
  pub placeholder: AstName,
  pub builtin_globals: DenseHashMap<AstName, Global>,
  pub scope: ScopePtr,
  pub module: *const Module,
}

// —— 原 methods/lint_context_get_type.rs ——
impl LintContext {
  pub fn get_type(&self, expr: *mut AstExpr) -> Option<TypeId> {
    if self.module.is_null() {
      return None;
    }
    let module = unsafe { &*self.module };
    module.ast_types.find(&(expr as *const AstExpr)).copied()
  }
}

// —— 原 methods/lint_context_lint_context.rs ——
impl DenseDefault for Global {
  fn dense_default() -> Self {
    Self::default()
  }
}
impl LintContext {
  pub fn lint_context(&mut self) {
    self.root = null_mut();
    self.placeholder = AstName::new();
    self.builtin_globals = DenseHashMap::default();
    self.module = null();
  }
}

// —— 原 methods/lint_context_warning_enabled.rs ——
impl LintContext {
  pub fn warning_enabled(&mut self, code: Code) -> bool {
    let code_val = code as u64;
    (self.options.warning_mask & (1u64 << code_val)) != 0
  }
}
