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
  /// 对应 C++ `willRunTimeError` 的函数定义形态。`fragment` 为检查期间存活
  /// 于 AST arena 的 `AstLocal` 引用，`scope` 为调用方保活的模块 Scope 树内
  /// 作用域共享引用（栈顶守卫或模块根，C++ `NotNull<Scope*>` 契约的引用化）。
  pub fn will_run_time_error_function_definition(
    &mut self,
    fragment: &AstLocal,
    scope: &Scope,
    context: &NonStrictContext,
  ) -> Option<TypeId> {
    // dfg 解引用经 `dfg_ref` 收口（字段已句柄化）；指针仅作 DFG 映射键。
    let def: DefId = self
      .dfg_ref()
      .get_def_ast_local(fragment as *const AstLocal);
    let defs = collect_operands(def);

    for def_item in defs.iter() {
      if let Some(context_ty) = context.find_def_id(def_item) {
        let r1: SubtypingResult = self.subtyping.is_subtype_type_id_type_id_not_null_scope(
          self.builtin_types_ref().unknown_type,
          context_ty,
          scope,
        );

        let r2: SubtypingResult = self.subtyping.is_subtype_type_id_type_id_not_null_scope(
          context_ty,
          self.builtin_types_ref().unknown_type,
          scope,
        );

        if r1.normalization_too_complex || r2.normalization_too_complex {
          // fragment 为存活 AstLocal 引用，直接读 location 一份拷贝。
          let loc: Location = fragment.location;
          self.report_error(
            TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
            &loc,
          );
        }

        let is_unknown: bool = r1.is_subtype && r2.is_subtype;
        if is_unknown {
          return Some(self.builtin_types_ref().unknown_type);
        }
      }
    }

    None
  }
}
