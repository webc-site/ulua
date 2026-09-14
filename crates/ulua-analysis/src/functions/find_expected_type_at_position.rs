use ulua_ast::records::{ast_expr::AstExpr, position::Position};

use crate::{
  functions::find_expr_at_position::find_expr_at_position,
  records::{module::Module, source_module::SourceModule},
  type_aliases::type_id::TypeId,
};
pub fn find_expected_type_at_position(
  module: &Module,
  source_module: &SourceModule,
  pos: Position,
) -> Option<TypeId> {
  let expr = find_expr_at_position(source_module, pos);
  if !expr.is_null()
    && let Some(&ty) = module.ast_expected_types.find(&(expr as *const AstExpr))
  {
    return Some(ty);
  }
  None
}
