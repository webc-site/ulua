use alloc::vec::Vec;

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
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn will_run_time_error(
    &mut self,
    fragment: *mut AstExpr,
    context: &NonStrictContext,
    scope: *mut Scope,
  ) -> Option<TypeId> {
    // Safety: `self.dfg` 是构造期从调用方长生命周期 `&DataFlowGraph`（直译 C++
    // `const DataFlowGraph&`）接线的非空裸指针，比本 checker 长寿；`get_def`
    // 只读 def 表，`fragment` 依本 `unsafe fn` 契约为存活 AST 节点，仅查键。
    let def: DefId = unsafe { &*self.dfg }.get_def(fragment);
    let mut defs: Vec<DefId> = Vec::new();
    collect_operands(def, &mut defs);

    for def_item in defs.iter() {
      if let Some(context_ty) = context.find_def(*def_item) {
        // Safety: `fragment` 由 visit 分发链传入、指向 parse arena 存活的
        // `AstExpr`（本函数契约要求，且上方 dfg.get_def 已以其为键查询），满足
        // `lookup_type` 的「expr 为 module 登记的存活节点」前提。
        let actual_type = unsafe { self.lookup_type(fragment) };

        if self.should_skip_runtime_error_testing(actual_type) {
          continue;
        }

        let r: SubtypingResult =
          self
            .subtyping
            .is_subtype_type_id_type_id_not_null_scope(actual_type, context_ty, scope);

        if r.normalization_too_complex {
          // Safety: `fragment` 依函数级契约指向存活 repr(C) `AstExpr`，此处只
          // 拷出基址重合的首字段 `base.location` 值，随后即离开本借用窗口。
          let loc = unsafe { (*fragment).base.location };
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
