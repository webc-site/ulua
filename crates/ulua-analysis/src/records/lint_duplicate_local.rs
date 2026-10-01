use core::ptr::from_mut;

use ulua_ast::{
  records::{
    ast_expr_function::AstExprFunction, ast_local::AstLocal, ast_node::AstNode,
    ast_stat_local::AstStatLocal, ast_visitor::AstVisitor,
  },
  rtti::AstNodePtr,
  visit::ast_stat_visit,
};
use ulua_common::records::dense_hash_map::DenseHashMap;
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  macros::lint_stat_process,
  records::{
    arena_handle::alias_ref, lint_context::LintContext, lint_context_handle::LintContextHandle,
  },
};

/// `_` 前缀局部变量视为有意忽略，不告警。
///
/// SAFETY: `local` 须指向 visit 遍历中的存活 `AstLocal`（与
/// `visit_ast_stat_local` 同源于 `AstStatLocal::vars`，访问期无变异）。
fn is_underscore_name(local: *mut AstLocal) -> bool {
  alias_ref(local).name.as_bytes() == b"_"
}

#[derive(Debug, Clone)]
pub struct LintDuplicateLocal<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
  pub(crate) locals: DenseHashMap<*mut AstLocal, *mut AstNode>,
}

impl<'ctx> LintDuplicateLocal<'ctx> {
  /// `_` 前缀局部变量视为有意忽略，不告警；指针有效性契约同
  /// [`is_underscore_name`]。
  pub fn ignore_duplicate(&self, local: *mut AstLocal) -> bool {
    is_underscore_name(local)
  }
}

impl<'ctx> AstVisitor for LintDuplicateLocal<'ctx> {
  fn visit_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    self.visit_ast_stat_local(from_mut(node))
  }

  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    self.visit_ast_expr_function(from_mut(node))
  }
}

// —— 原 methods/lint_duplicate_local_process.rs ——
impl<'ctx> LintDuplicateLocal<'ctx> {
  lint_stat_process!(LintDuplicateLocal {
    locals: DenseHashMap::default()
  });
}

// —— 原 methods/lint_duplicate_local_visit_linter.rs ——
impl<'ctx> LintDuplicateLocal<'ctx> {
  pub(crate) fn visit_ast_stat_local(&mut self, node: *mut AstStatLocal) -> bool {
    let node_ref = alias_ref(node);
    // early out for performance
    if node_ref.vars.len() == 1 {
      return true;
    }
    for &var in node_ref.vars.as_slice() {
      *self.locals.get_or_insert(var) = node.as_ast_node();
    }
    for &local in node_ref.vars.as_slice() {
      let local_ref = alias_ref(local);
      if !local_ref.shadow.is_null()
        && self.locals.find(&local_ref.shadow).copied() == Some(node.as_ast_node())
        && !self.ignore_duplicate(local)
      {
        let shadow = alias_ref(local_ref.shadow);
        if shadow.location.begin.line == local_ref.location.begin.line {
          emit_warning(
            self.context.get(),
            Code::DuplicateLocal,
            local_ref.location,
            format_args!(
              "Variable '{}' already defined on column {}",
              local_ref.name,
              shadow.location.begin.column + 1
            ),
          );
        } else {
          emit_warning(
            self.context.get(),
            Code::DuplicateLocal,
            local_ref.location,
            format_args!(
              "Variable '{}' already defined on line {}",
              local_ref.name,
              shadow.location.begin.line + 1
            ),
          );
        }
      }
    }
    true
  }
  pub(crate) fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
    let node_ref = alias_ref(node);
    if !node_ref.self_.is_null() {
      *self.locals.get_or_insert(node_ref.self_.as_ptr()) = node.as_ast_node();
    }
    for arg in node_ref.args.iter_nodes() {
      *self.locals.get_or_insert(arg.as_ptr()) = node.as_ast_node();
    }
    for local_node in node_ref.args.iter_nodes() {
      let local_ref = local_node.get();
      let local = local_node.as_ptr();
      if !local_ref.shadow.is_null()
        && self.locals.find(&local_ref.shadow).copied() == Some(node.as_ast_node())
        && !self.ignore_duplicate(local)
      {
        if local_ref.shadow == node_ref.self_.as_ptr() {
          emit_warning(
            self.context.get(),
            Code::DuplicateLocal,
            local_ref.location,
            format_args!("Function parameter 'self' already defined implicitly"),
          );
        } else {
          let shadow = alias_ref(local_ref.shadow);
          if shadow.location.begin.line == local_ref.location.begin.line {
            emit_warning(
              self.context.get(),
              Code::DuplicateLocal,
              local_ref.location,
              format_args!(
                "Function parameter '{}' already defined on column {}",
                local_ref.name,
                shadow.location.begin.column + 1
              ),
            );
          } else {
            emit_warning(
              self.context.get(),
              Code::DuplicateLocal,
              local_ref.location,
              format_args!(
                "Function parameter '{}' already defined on line {}",
                local_ref.name,
                shadow.location.begin.line + 1
              ),
            );
          }
        }
      }
    }
    true
  }
}
