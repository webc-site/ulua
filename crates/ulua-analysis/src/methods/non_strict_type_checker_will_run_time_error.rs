use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::fflag;

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
  /// 对应 C++ `bool NonStrictTypeChecker::willRunTimeError(...)`。`fragment`
  /// 为 parse arena 存活的 `AstExpr` 引用，`scope` 为模块 Scope 树内存活
  /// 作用域的共享引用；两者均由调用方（visit 分发链 / `find_innermost_scope`）
  /// 以引用类型传递，本函数无 unsafe。
  pub fn will_run_time_error(
    &mut self,
    fragment: &AstExpr,
    context: &NonStrictContext,
    scope: &Scope,
  ) -> Option<TypeId> {
    // dfg 解引用经 `dfg_ref` 收口（字段已句柄化）；指针仅作 DFG 映射键。
    let def: DefId = self.dfg_ref().get_def(fragment as *const AstExpr);
    let defs = collect_operands(def);

    for def_item in defs.iter() {
      if let Some(context_ty) = context.find_def(*def_item) {
        let actual_type = self.lookup_type(fragment);

        if self.should_skip_runtime_error_testing(actual_type) {
          continue;
        }

        let r: SubtypingResult =
          self
            .subtyping
            .is_subtype_type_id_type_id_not_null_scope(actual_type, context_ty, scope);

        if r.normalization_too_complex {
          let loc = fragment.base.location;
          self.report_error(
            TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
            &loc,
          );
        }

        if fflag::LuauNonStrictModeUseErrorSupressingTag.get() {
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
