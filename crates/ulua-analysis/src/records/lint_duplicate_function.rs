use alloc::string::String;
use core::ptr::from_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_stat_block::AstStatBlock, ast_stat_function::AstStatFunction,
    ast_stat_local_function::AstStatLocalFunction, ast_visitor::AstVisitor, location::Location,
  },
  rtti::{ast_node_try_as, ast_node_try_as_ptr},
  visit::ast_stat_visit,
};
use ulua_common::records::dense_hash_map::DenseHashMap;
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  macros::lint_stat_process,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

#[derive(Debug, Clone)]
pub struct LintDuplicateFunction<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
  pub(crate) defns: DenseHashMap<String, Location>,
}

impl<'ctx> AstVisitor for LintDuplicateFunction<'ctx> {
  fn visit_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    self.visit_ast_stat_block(from_mut(node))
  }
}

// —— 原 methods/lint_duplicate_function_build_name.rs ——
impl<'ctx> LintDuplicateFunction<'ctx> {
  pub(crate) fn build_name(&self, expr: *mut AstExpr) -> String {
    if let Some(local) = unsafe { ast_node_try_as_ptr::<AstExprLocal>(expr) } {
      let name = local.local.name;
      if !name.is_null() {
        return name.to_string();
      }
    } else if let Some(global) = unsafe { ast_node_try_as_ptr::<AstExprGlobal>(expr) } {
      let name = global.name;
      if !name.is_null() {
        return name.to_string();
      }
    } else if let Some(index_name) = unsafe { ast_node_try_as_ptr::<AstExprIndexName>(expr) } {
      // expr 已句柄化恒非空；build_name 行走链为既有裸指针 API，经 as_ptr 桥接。
      let lhs = self.build_name(index_name.expr.as_ptr());
      if lhs.is_empty() {
        return lhs;
      }
      return format!("{lhs}.{}", index_name.index);
    }
    String::new()
  }
}

// —— 原 methods/lint_duplicate_function_lint_duplicate_function.rs ——
impl<'ctx> LintDuplicateFunction<'ctx> {
  pub fn lint_duplicate_function(&mut self) {
    // Placeholder for constructor logic
  }
}

// —— 原 methods/lint_duplicate_function_process.rs ——
impl<'ctx> LintDuplicateFunction<'ctx> {
  lint_stat_process!(LintDuplicateFunction {
    defns: DenseHashMap::default()
  });
}

// —— 原 methods/lint_duplicate_function_report.rs ——
impl<'ctx> LintDuplicateFunction<'ctx> {
  #[inline]
  pub fn report(&mut self, name: &str, location: Location, other_location: Location) {
    self.report_location_c_char_location(name, location, other_location);
  }
  pub fn report_location_c_char_location(
    &mut self,
    name: &str,
    location: Location,
    other_location: Location,
  ) {
    emit_warning(
      self.context.get(),
      Code::DuplicateFunction,
      location,
      format_args!(
        "Duplicate function definition: '{}' also defined on line {}",
        name,
        other_location.begin.line + 1
      ),
    );
  }
}

// —— 原 methods/lint_duplicate_function_track_function.rs ——
impl<'ctx> LintDuplicateFunction<'ctx> {
  pub fn track_function(&mut self, location: Location, name: &str) {
    if name.is_empty() {
      return;
    }
    let mut other_location = None;
    {
      let defn = self.defns.get_or_insert(String::from(name));
      if defn.end.line == 0 && defn.end.column == 0 {
        *defn = location;
      } else {
        other_location = Some(*defn);
      }
    }
    if let Some(defn) = other_location {
      self.report(name, location, defn);
    }
  }
}

// —— 原 methods/lint_duplicate_function_visit.rs ——
impl<'ctx> LintDuplicateFunction<'ctx> {
  pub(crate) fn visit_ast_stat_block(&mut self, block: *mut AstStatBlock) -> bool {
    self.defns.clear();
    // Safety: `block` 由唯一内部调用点（`LintDuplicateFunction` 的 AstVisitor StatBlock 分派，
    // lint_duplicate_function.rs）传入，指向 parse 产出、在整次 lint 期内存活的 `AstStatBlock`，
    // 非空且对齐；此处取共享引用读其 `body`（`AstArray`），不写入。
    let block_ref = unsafe { &*block };
    let body = &block_ref.body;
    for stat in body.iter() {
      if let Some(func) = ast_node_try_as::<AstStatFunction>(stat) {
        // name 已句柄化为 Node<AstExpr>：`.get()` 即安全只读视图（原 unsafe 死
        // 守卫消失），`as_ptr` 桥交仍以指针形态消费的 build_name。
        let name_loc = func.name.get().base.location;
        self.track_function(name_loc, &self.build_name(func.name.as_ptr()));
        continue;
      }
      if let Some(local_func) = ast_node_try_as::<AstStatLocalFunction>(stat) {
        // name 已句柄化为 Node<AstLocal>：`.get()` 即安全只读视图，无裸指针解引用。
        let local_name = local_func.name.get();
        let name = local_name.name;
        if !name.is_null() {
          let name = name.as_str_or_empty();
          self.track_function(local_name.location, name);
        }
      }
    }
    true
  }
}
