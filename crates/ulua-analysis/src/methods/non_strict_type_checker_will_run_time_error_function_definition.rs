use alloc::vec::Vec;

use ulua_ast::records::{ast_local::AstLocal, location::Location};

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
  pub unsafe fn will_run_time_error_function_definition(
    &mut self,
    fragment: *mut AstLocal,
    scope: *mut Scope,
    context: &NonStrictContext,
  ) -> Option<TypeId> {
    let def: DefId = unsafe { &*self.dfg }.get_def_ast_local(fragment);
    let mut defs: Vec<DefId> = Vec::new();
    collect_operands(def, &mut defs);

    for def_item in defs.iter() {
      if let Some(context_ty) = context.find_def_id(def_item) {
        let r1: SubtypingResult = self.subtyping.is_subtype_type_id_type_id_not_null_scope(
          unsafe { (*self.builtin_types).unknown_type },
          context_ty,
          scope,
        );

        let r2: SubtypingResult = self.subtyping.is_subtype_type_id_type_id_not_null_scope(
          context_ty,
          unsafe { (*self.builtin_types).unknown_type },
          scope,
        );

        if r1.normalization_too_complex || r2.normalization_too_complex {
          let loc: Location = unsafe { &*fragment }.location;
          self.report_error(
            TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
            &loc,
          );
        }

        let is_unknown: bool = r1.is_subtype && r2.is_subtype;
        if is_unknown {
          return Some(unsafe { (*self.builtin_types).unknown_type });
        }
      }
    }

    None
  }
}
