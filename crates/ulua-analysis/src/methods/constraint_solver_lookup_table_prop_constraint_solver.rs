use core::ptr::null;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::value_context::ValueContext,
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver,
    table_prop_lookup_result::TablePropLookupResult,
  },
  type_aliases::type_id::TypeId,
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证 `constraint` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool(
    &mut self,
    constraint: *const Constraint,
    subject_type: TypeId,
    prop_name: &str,
    context: ValueContext,
    in_conditional: bool,
    suppress_simplification: bool,
  ) -> TablePropLookupResult {
    let mut seen = DenseHashSet::new(null());
    unsafe {
      self.lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool_set_type_id(
        constraint,
        subject_type,
        prop_name,
        context,
        in_conditional,
        suppress_simplification,
        &mut seen,
      )
    }
  }
}
