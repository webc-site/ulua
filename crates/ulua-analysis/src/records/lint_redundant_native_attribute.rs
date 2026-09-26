use core::ptr::from_mut;

use ulua_ast::{
  records::{
    ast_attr::{AstAttr, AstAttrType},
    ast_expr_function::AstExprFunction,
    ast_stat::AstStat,
    ast_visitor::AstVisitor,
  },
  visit::ast_stat_visit,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  macros::lint_stat_process,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

#[derive(Debug, Clone)]
pub struct LintRedundantNativeAttribute<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> AstVisitor for LintRedundantNativeAttribute<'ctx> {
  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    self.visit_ast_expr_function(from_mut(node))
  }

  // visit_node 沿用 trait 默认实现（返回 true）
  fn visit_attr(&mut self, _node: &mut AstAttr) -> bool {
    false
  }
}

// —— 原 methods/lint_redundant_native_attribute_process.rs ——
impl<'ctx> LintRedundantNativeAttribute<'ctx> {
  lint_stat_process!(LintRedundantNativeAttribute);
}

// —— 原 methods/lint_redundant_native_attribute_visit.rs ——
impl<'ctx> LintRedundantNativeAttribute<'ctx> {
  pub(crate) fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
    unsafe {
      let node = &*node;
      ast_stat_visit(node.body.cast::<AstStat>().as_ptr(), self);
      for attr in node.attributes.iter() {
        if attr.r#type == AstAttrType::Native {
          emit_warning(
            self.context.get(),
            Code::RedundantNativeAttribute,
            attr.base.location,
            format_args!(
              "native attribute on a function is redundant in a native module; consider removing it"
            ),
          );
        }
      }
    }
    false
  }
}
