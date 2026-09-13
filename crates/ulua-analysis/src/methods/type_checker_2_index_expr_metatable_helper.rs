//! `TypeChecker2::indexExprMetatableHelper`（TypeChecker2.cpp:2153-2175）。
use ulua_ast::records::ast_expr_index_expr::AstExprIndexExpr;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    cannot_extend_table::{CannotExtendTable, Context},
    metatable_type::MetatableType,
    table_type::TableType,
    type_checker_2::TypeChecker2,
    type_error_data::TypeErrorData,
  },
  type_aliases::type_id::TypeId,
};
impl TypeChecker2 {
  pub fn type_checker_2_index_expr_metatable_helper(
    &mut self,
    index_expr: &AstExprIndexExpr,
    meta_table: &MetatableType,
    expr_type: TypeId,
    index_type: TypeId,
  ) {
    let table_followed = follow_type_id(meta_table.table());
    if let Some(tt) = get_type_id::<TableType>(table_followed)
      && let Some(indexer) = &tt.indexer
    {
      self.test_is_subtype_type_id_type_id_location(
        index_type,
        indexer.index_type,
        index_expr.base.base.location,
      );
      return;
    }

    if let Some(mt) = get_type_id::<MetatableType>(table_followed) {
      self.type_checker_2_index_expr_metatable_helper(index_expr, mt, expr_type, index_type);
      return;
    }

    let metatable_followed = follow_type_id(meta_table.metatable());
    if let Some(tmt) = get_type_id::<TableType>(metatable_followed)
      && let Some(indexer) = &tmt.indexer
    {
      self.test_is_subtype_type_id_type_id_location(
        index_type,
        indexer.index_type,
        index_expr.base.base.location,
      );
      return;
    }

    if let Some(mtmt) = get_type_id::<MetatableType>(metatable_followed) {
      self.type_checker_2_index_expr_metatable_helper(index_expr, mtmt, expr_type, index_type);
      return;
    }

    let cannot_extend = CannotExtendTable {
      table_type: expr_type,
      context: Context::Indexer,
      prop: "indexer??".to_string(),
    };
    self.report_error_type_error_data_location(
      TypeErrorData::CannotExtendTable(cannot_extend),
      &index_expr.base.base.location,
    );
  }
}
