use alloc::{string::String, vec::Vec};

use ulua_ast::records::{ast_expr_call::AstExprCall, location::Location};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    follow_type, get_type, size_type_pack::size, to_string_to_string::to_string_type_id,
  },
  records::{
    extra_information::ExtraInformation, function_type::FunctionType, generic_error::GenericError,
    overload_error_entry::OverloadErrorEntry, type_checker::TypeChecker,
  },
  type_aliases::{
    scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
impl TypeChecker {
  pub(crate) fn report_overload_resolution_error(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprCall,
    ret_pack: TypePackId,
    arg_pack: TypePackId,
    arg_locations: &[Location],
    overloads: &[TypeId],
    overloads_that_match_arg_count: &[TypeId],
    errors: &mut [OverloadErrorEntry],
  ) {
    if overloads.len() == 1 {
      let error = errors
        .first_mut()
        .expect("single failed overload has errors");
      error.log.commit();
      let reported_errors = error.errors.clone();

      self.report_errors(&reported_errors);
      return;
    }

    let mut overload_types = overloads_that_match_arg_count.to_vec();
    if overloads_that_match_arg_count.is_empty() {
      self.report_error_location_type_error_data(
        &expr.base.base.location,
        TypeErrorData::GenericError(GenericError::new(format!(
          "No overload for function accepts {} arguments.",
          // Safety: arg_pack 指向类型 pack arena（bump 块地址不移动）中存活节点；
          // log 传 null 等价 C++ 默认实参 TxnLog* = nullptr，size 内部判空后走全局
          // follow 分支，不会解引用该空指针。
          size(arg_pack, None)
        ))),
      );

      overload_types = overloads.to_vec();
    } else {
      // Report errors of the first argument-count-matching, but failing overload
      let overload = overloads_that_match_arg_count[0];
      // 上游 `const FunctionType* ftv = get<FunctionType>(overload)` 后按指针比对
      // （TypeInfer.cpp:4905-4916）；TypeId 即类型节点地址，直接相等即可。
      overload_types.retain(|ty| *ty != overload);

      let matched = errors.iter_mut().find(|e| e.fn_ty == overload);
      // 上游 `LUAU_ASSERT(error != errors.end())`：条目缺失说明重载消解状态自相矛盾。
      // 断言（仅 debug 生效）之后安全跳过该条上报，而不是解引用不存在的条目。
      LUAU_ASSERT!(matched.is_some());
      if let Some(error) = matched {
        error.log.commit();
        let reported_errors = error.errors.clone();

        self.report_errors(&reported_errors);
      }

      // If only one overload matched, we don't need this error because we provided the previous errors.
      if overloads_that_match_arg_count.len() == 1 {
        return;
      }
    }

    let mut s = String::new();
    for (i, overload) in overload_types.iter().enumerate() {
      let overload = follow_type::follow(*overload);
      let mut state = self.mk_unifier(scope, &expr.base.base.location);

      if let Some(ftv) = get_type::get::<FunctionType>(overload) {
        // Safety: expr.func 是 parser 恒写入的非空子表达式指针（错误路径也有
        // AstExprError 占位，且该字段非显式 Optional），指向本模块 parse arena
        // 中整个报错期存活的节点；两处只读取，形参为 &AstExpr 共享引用。
        self.check_argument_list(
          scope,
          unsafe { &*expr.func },
          &mut state,
          ret_pack,
          ftv.ret_types,
          &Vec::new(),
        );
        // Safety: 同上——expr.func 非空存活，本处仍只读取。
        self.check_argument_list(
          scope,
          unsafe { &*expr.func },
          &mut state,
          arg_pack,
          ftv.arg_types,
          arg_locations,
        );
      }

      if state.errors.is_empty() {
        state.log.commit();
      }

      if i > 0 {
        s.push_str("; ");
      }

      if i > 0 && i == overload_types.len() - 1 {
        s.push_str("and ");
      }

      s.push_str(&to_string_type_id(overload));
    }

    let message = if overloads_that_match_arg_count.is_empty() {
      String::from("Available overloads: ") + &s
    } else {
      String::from("Other overloads are also not viable: ") + &s
    };

    self.report_error_location_type_error_data(
      // Safety: expr.func 非空存活（parser 恒写入、错误占位亦为节点），repr(C)
      // 首字段 base 使 AstExpr 读取同址有效，此处仅拷贝 location。
      unsafe { &(*expr.func).base.location },
      TypeErrorData::ExtraInformation(ExtraInformation::new(message)),
    );
  }
}
