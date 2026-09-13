use alloc::vec::Vec;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::FFlag;

use crate::{
  functions::collect_operands::collect_operands,
  records::{
    non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker,
    normalization_too_complex::NormalizationTooComplex, scope::Scope,
    subtyping_result::SubtypingResult,
  },
  type_aliases::{def_id_def::DefId, type_error_data::TypeErrorData, type_id::TypeId},
};

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn will_run_time_error(
    &mut self,
    fragment: *mut AstExpr,
    context: &NonStrictContext,
    scope: *mut Scope,
  ) -> Option<TypeId> {
    let def: DefId = unsafe { &*self.dfg }.get_def(fragment);
    let mut defs: Vec<DefId> = Vec::new();
    collect_operands(def, &mut defs);

    for def_item in defs.iter() {
      if let Some(context_ty) = context.find_def(*def_item) {
        let actual_type = unsafe { self.lookup_type(fragment) };

        if self.should_skip_runtime_error_testing(actual_type) {
          continue;
        }

        let r: SubtypingResult =
          self
            .subtyping
            .is_subtype_type_id_type_id_not_null_scope(actual_type, context_ty, scope);

        if r.normalization_too_complex {
          let loc = unsafe { (*fragment).base.location };
          self.report_error(
            TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
            &loc,
          );
        }

        if FFlag::LuauNonStrictModeUseErrorSupressingTag.get() {
          if r.is_subtype && !r.is_error_suppressing {
            return Some(actual_type);
          }
        } else {
          if r.is_subtype {
            return Some(actual_type);
          }
        }
      }
    }

    None
  }
}
