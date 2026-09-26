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
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

/// `_` 前缀局部变量视为有意忽略，不告警。
///
/// SAFETY: `local` 须指向 visit 遍历中的存活 `AstLocal`（与
/// `visit_ast_stat_local` 同源于 `AstStatLocal::vars`，访问期无变异）。
fn is_underscore_name(local: *mut AstLocal) -> bool {
  // SAFETY: 见上。
  unsafe { &*local }.name.as_bytes() == b"_"
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
    // Safety: node 由 AstVisitor 的 `&mut AstStatLocal`（from_mut）转成，存活、对齐且本帧
    // 唯一访问路径，故 `&*node` 只读其 vars 数组无别名冲突；vars 是 parser 成对写入 data/size
    // 的 AstArray<*mut AstLocal>，元素恒非空且指向 parse arena 存活节点，故 `&*local` 合法，
    // `&*local_ref.shadow` 前亦已判 shadow.is_null。self.locals 以这些存活指针为键，
    // 全程单线程顺序遍历，emit_warning 参数借用止于本次调用。
    unsafe {
      let node_ref = &*node;
      // early out for performance
      if node_ref.vars.len() == 1 {
        return true;
      }
      for &var in node_ref.vars.as_slice() {
        *self.locals.get_or_insert(var) = node.as_ast_node();
      }
      for &local in node_ref.vars.as_slice() {
        let local_ref = &*local;
        if !local_ref.shadow.is_null()
          && self.locals.find(&local_ref.shadow).copied() == Some(node.as_ast_node())
          && !self.ignore_duplicate(local)
        {
          let shadow = &*local_ref.shadow;
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
    }
    true
  }
  pub(crate) fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
    // Safety: node 由 AstVisitor 的 `&mut AstExprFunction`（from_mut）转成，存活、对齐且本
    // 帧唯一访问路径，`&*node` 只读其 self_/args；args 为 parser 写入的 AstArray<*mut AstLocal>
    // 元素恒非空，self_ 使用前已判 is_null；`&*local_ref.shadow` 前亦判空。self.locals 的
    // 借用前提同 visit_ast_stat_local（单线程顺序遍历）。
    unsafe {
      let node_ref = &*node;
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
            let shadow = &*local_ref.shadow;
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
    }
    true
  }
}
