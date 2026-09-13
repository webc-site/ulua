use ulua_ast::records::ast_expr::AstExpr;

use crate::{enums::type_constant_folding::Type, records::cost_visitor::CostVisitor};

impl CostVisitor {
  pub fn get_number(&self, node: *mut AstExpr, result: &mut f64) -> bool {
    unsafe {
      if let Some(constant) = (*self.constants).find(&node)
        && constant.r#type == Type::Number
      {
        *result = constant.data.value_number;
        return true;
      }
    }
    false
  }
}
