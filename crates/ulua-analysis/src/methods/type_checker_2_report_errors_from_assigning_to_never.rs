//! `TypeChecker2::reportErrorsFromAssigningToNever`（TypeChecker2.cpp:1188-1217）。
use alloc::vec::Vec;
use core::{ffi::CStr, ptr::null};

use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_index_name::AstExprIndexName, ast_node::AstNode},
  rtti::ast_node_try_as,
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::{normalization_result::NormalizationResult, reason::Reason, value_context::ValueContext},
  functions::get_type_alt_j::get_type_id,
  records::{
    cannot_assign_to_never::CannotAssignToNever, never_type::NeverType, r#type::Type,
    type_checker_2::TypeChecker2, type_error::TypeError,
  },
  type_aliases::{type_error_data::IntoTypeErrorData, type_id::TypeId},
};
impl TypeChecker2 {
  pub fn report_errors_from_assigning_to_never(&mut self, lhs: &AstExpr, rhs_type: TypeId) {
    // SAFETY: AstExpr 是 #[repr(C)] 单继承，AstNode 子对象在偏移 0，cast 有效。
    let node = unsafe { &*((lhs as *const AstExpr).cast::<AstNode>()) };
    let Some(index_name) = ast_node_try_as::<AstExprIndexName>(node) else {
      return;
    };

    // SAFETY: index_name->expr 指向 AST arena 节点，与 visit 树同寿。
    let indexed_type = self.lookup_type(unsafe { &*index_name.expr });

    // if it's already never, I don't think we have anything to do here.
    if get_type_id::<NeverType>(indexed_type).is_some() {
      return;
    }

    // SAFETY: index.value 指向 AST arena 内的 NUL 结尾字面量。
    let prop = unsafe { CStr::from_ptr(index_name.index.value) }
      .to_string_lossy()
      .into_owned();

    // C++: `std::shared_ptr<const NormalizedType> norm =
    // normalizer.normalize(indexedType);` followed by `if (!norm) { reportError(
    // NormalizationTooComplex{}); return; }`. `Normalizer::normalize` here returns a
    // non-nullable `Arc<NormalizedType>`, so the limits-exceeded null case the C++
    // guards against is not representable through this signature and the value is
    // always present.
    let norm = self.normalizer.normalize(indexed_type);

    // if the type is error suppressing, we don't actually have any work left to do.
    if norm.should_suppress_errors() {
      return;
    }

    let location = lhs.base.location;
    let mut cause = Vec::new();

    // C++ passes `lookupProp(...).typesOfProp` as the cause.  The full
    // `lookup_prop` method is still a stub, but the table component path
    // is enough to preserve the tagged-union narrowing reason here.
    for &ty in norm.tables.order.iter() {
      if self.normalizer.is_inhabited_type_id(ty) != NormalizationResult::True {
        continue;
      }

      let mut seen = DenseHashSet::new(null::<Type>());
      let mut dummy_errors: Vec<TypeError> = Vec::new();
      // SAFETY: builtin_types 由构造方保证有效（C++ 同契约）。
      let prop_type = self.has_index_type_from_type(
        ty,
        &prop,
        ValueContext::LValue,
        &location,
        &mut seen,
        unsafe { (*self.builtin_types).string_type },
        &mut dummy_errors,
      );

      if prop_type.present == NormalizationResult::True
        && let Some(result) = prop_type.result
      {
        cause.push(result);
      }
    }

    let err = CannotAssignToNever::new(rhs_type, cause, Reason::PropertyNarrowed);
    self.report_error_type_error_data_location(err.into_type_error_data(), &location);
  }
}
