use ulua_ast::records::{ast_expr::AstExpr, ast_name::AstName, ast_type::AstType};
use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType, records::dense_hash_set::DenseHashSet,
};

use crate::{functions::get_type::get_type, records::type_map_visitor::TypeMapVisitor};

impl<'a> TypeMapVisitor<'a> {
  pub fn record_resolved_type_ast_expr_ast_type(
    &mut self,
    expr: *mut AstExpr,
    ty: *const AstType,
  ) -> LuauBytecodeType {
    let ty = self.resolve_aliases_deprecated(ty);

    *self.resolved_exprs.get_or_insert(expr) = ty;

    let mut seen_aliases: DenseHashSet<AstName> = DenseHashSet::new(AstName::new());

    let bty = unsafe {
      get_type(
        ty,
        Default::default(),
        &self.type_aliases,
        true,
        self.host_vector_type,
        self.userdata_types,
        self.bytecode,
        &mut seen_aliases,
      )
    };

    *self.expr_types.get_or_insert(expr) = bty;
    bty
  }
}
