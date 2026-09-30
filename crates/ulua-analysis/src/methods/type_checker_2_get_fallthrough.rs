use ulua_ast::{
  enums::ast_stat_ref::AstStatRef,
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool, ast_stat::AstStat,
  },
  rtti::ast_node_try_as,
};

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  // cpp TypeChecker2.cpp:411
  pub fn type_checker_2_get_fallthrough<'a>(&mut self, node: &'a AstStat) -> Option<&'a AstStat> {
    match node.as_stat_ref() {
      AstStatRef::Block(stat) => {
        if stat.body.is_empty() {
          return Some(node);
        }

        // 除最后一条外的所有语句都必须能"穿透"（不含 return/break）
        let body = stat.body.as_slice();
        if body[..body.len() - 1]
          .iter()
          .any(|child| self.type_checker_2_get_fallthrough(child.get()).is_none())
        {
          return None;
        }

        self.type_checker_2_get_fallthrough(body[body.len() - 1].get())
      }

      AstStatRef::If(stat) => {
        let thenf = self.type_checker_2_get_fallthrough(&stat.thenbody.get().base);
        if thenf.is_some() {
          return thenf;
        }

        if let Some(else_stat) = stat.elsebody.get() {
          return self.type_checker_2_get_fallthrough(else_stat);
        }
        Some(node)
      }

      AstStatRef::Return(_) => None,

      AstStatRef::Expr(stat) => {
        if let Some(call) = ast_node_try_as::<AstExprCall>(stat.expr.get())
          && self.is_error_call(call)
        {
          return None;
        }
        Some(node)
      }

      AstStatRef::While(stat) => {
        if let Some(expr) = ast_node_try_as::<AstExprConstantBool>(stat.condition.get())
          && expr.value
          && !self.type_checker_2_has_break(&stat.body.get().base)
        {
          return None;
        }
        Some(node)
      }

      AstStatRef::Repeat(stat) => {
        if let Some(expr) = ast_node_try_as::<AstExprConstantBool>(stat.condition.get())
          && !expr.value
          && !self.type_checker_2_has_break(&stat.body.get().base)
        {
          return None;
        }
        self
          .type_checker_2_get_fallthrough(&stat.body.get().base)?;
        Some(node)
      }

      _ => Some(node),
    }
  }
}
