use ulua_ast::{
  records::{
    ast_attr::{AstAttr, AstAttrType},
    ast_expr_function::AstExprFunction,
    ast_stat::AstStat,
    ast_visitor::AstVisitor,
  },
  visit::ast_stat_visit_ref,
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
    self.visit_ast_expr_function(node)
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
  pub(crate) fn visit_ast_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    // node.body 句柄 `cast::<AstStat>` + `get_mut()` 物化基类独占借用（repr(C)
    // 前缀重合），`_ref` 门面全链路 safe；attributes 为只读遍历。
    ast_stat_visit_ref(node.body.cast::<AstStat>().get_mut(), self);
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
    false
  }
}
