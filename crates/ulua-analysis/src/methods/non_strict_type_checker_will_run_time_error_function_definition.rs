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
  /// `fragment` 须指向本次检查存活于 AST arena 的 `AstLocal` 节点，`scope` 须为
  /// 调用方保活的 `Arc<Scope>` 写句柄（C++ `NotNull<Scope*>` 契约，由
  /// non_strict_type_checker_visit 的函数体遍历处成立）。
  pub unsafe fn will_run_time_error_function_definition(
    &mut self,
    fragment: *mut AstLocal,
    scope: *mut Scope,
    context: &NonStrictContext,
  ) -> Option<TypeId> {
    // Safety: self.dfg 是 NonStrictTypeChecker 构造期接线的 *const DataFlowGraph，
    // 指向 TypeChecker2 在进入 non-strict 检查前构建完成、整会话存活的 DFG
    // （C++ DataFlowGraph& 引用成员直译）；get_def 仅按节点地址查映射，不解引用。
    let def: DefId = unsafe { &*self.dfg }.get_def_ast_local(fragment);
    let mut defs: Vec<DefId> = Vec::new();
    collect_operands(def, &mut defs);

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
          // Safety: fragment 满足本 fn 的 # Safety 契约——调用侧传入函数形参
          // arena 节点（同时是上方 DFG get_def 的键），AST 在检查期不可变，
          // 此处只读 location 一份拷贝。
          let loc: Location = unsafe { &*fragment }.location;
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
