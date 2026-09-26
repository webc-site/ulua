use alloc::string::ToString;
use core::cmp::min;

use ulua_ast::{
  records::{
    ast_expr_constant_string::AstExprConstantString, ast_expr_index_name::AstExprIndexName,
  },
  rtti::ast_node_try_as_ptr,
};
use ulua_common::fflag;

use crate::{
  enums::value::Value,
  functions::{
    begin_type_pack::begin, end_type_pack::end, extract_format_string::extract_format_string,
    flatten_type_pack::flatten_type_pack_id, follow_type,
    parse_format_string::parse_format_string_bytes,
    should_suppress_errors_type_utils::should_suppress_errors, unwrap_group::unwrap_group,
  },
  records::{
    cannot_check_dynamic_string_format_calls::CannotCheckDynamicStringFormatCalls,
    count_mismatch::{CountMismatch, CountMismatchContext},
    error_suppression::ErrorSuppression,
    magic_function_type_check_context::MagicFunctionTypeCheckContext,
    type_mismatch::TypeMismatch,
  },
  type_aliases::type_error_data::TypeErrorData,
};
pub fn magic_format_type_check(context: &MagicFunctionTypeCheckContext) -> bool {
  // Safety: context.typechecker 是 C++ NotNull<TypeChecker2> 形参的 NonNull 化，
  // 由 TypeChecker2::visit(ExprCall) 在调用本 magic 时以自身构造；&mut 借用存续
  // 于整个函数体，期间 context 仅按值携带指针本身、别无访问该 checker 的路径。
  let typechecker = unsafe { &mut *context.typechecker.as_ptr() };
  // Safety: call_site 是正被 visit 的 AstExprCall（解析 arena 节点，check 期间
  // 存活不改写）的非空指针，只读再借用；其子指针字段仅按裸指针继续下传。
  let call_site = unsafe { &*context.call_site };

  let iter = { begin(context.arguments) };
  let end_iter = { end(context.arguments) };

  if iter == end_iter {
    typechecker.report_error_type_error_data_location(
      TypeErrorData::CountMismatch(CountMismatch {
        expected: 1,
        maximum: None,
        actual: 0,
        context: CountMismatchContext::Arg,
        is_variadic: true,
        function: "string.format".to_string(),
      }),
      &call_site.base.base.location,
    );
    return true;
  }

  // we'll suppress any errors for `string.format` if the format string is error suppressing.
  // Safety: iter 已过 begin!=end 判定，current 读 arguments TypePack 首个
  // head 元素（arena 活 TypeId）；normalizer 裸句柄得自 typechecker 的 &mut 字段
  // 借用，在调用期内该字段无第二访问路径，callee 的只读规范化缓存写入限于 self。
  if unsafe {
    // Safety: 同上——&mut typechecker.normalizer 派生自本函数持有的唯一可变借用。
    should_suppress_errors(
      &mut typechecker.normalizer as *mut _,
      follow_type::follow(*iter.current()),
    )
  } == ErrorSuppression::from_value(Value::Suppress)
  {
    return true;
  }

  let mut fmt: Option<&AstExprConstantString> = None;
  if !call_site.func.is_null() {
    // Safety: func 刚判非空，指向存活 AstExpr（解析 arena）；try_as_ptr 判空+
    // class_index 甄别，命中即 repr(C) 基址重合的存活派生节点只读借用，未命中 None。
    let index_expr = unsafe { ast_node_try_as_ptr::<AstExprIndexName>(call_site.func) };
    if let Some(index_expr) = index_expr
      && call_site.self_
    {
      // Safety: index_expr 是上一步 class_index 甄别命中的存活节点，此处只读 expr
      // 字段（Copy 裸指针值），取共享借用而非 &mut 以免与 func 的既有只读访问形成
      // 互斥别名。
      // expr 已句柄化恒非空；unwrap_group 行走链为既有裸指针 API，经 as_ptr 桥接。
      let unwrapped = unwrap_group(index_expr.expr.as_ptr());
      // Safety: unwrap_group 返回 null 或解组后的存活表达式节点（源自同一 arena）；
      // try_as_ptr 判空+甄别，命中即存活 AstExprConstantString 只读借用。
      fmt = unsafe { ast_node_try_as_ptr::<AstExprConstantString>(unwrapped) };
    }
  }

  if !call_site.self_
    && let Some(&arg) = call_site.args.first()
  {
    // Safety: 元素是存活表达式节点指针；try_as_ptr 判空+甄别，命中即存活 AstExprConstantString 借用。
    fmt = unsafe { ast_node_try_as_ptr::<AstExprConstantString>(arg) };
  }

  // 双写合一：原「fflag 分支 is_none 早退」+「else is_none 报告并早退」+
  // 「块后 unwrap()」三段收为一条 let-else——None 走各自原早退路径（含
  // 报告与 return true），Some 直接绑定，逐分支行为等价。
  let Some(format_str) = extract_format_string(fmt, &iter) else {
    if fflag::LuauSilenceDynamicFormatStringErrors.get() {
      return true;
    }
    typechecker.report_error_type_error_data_location(
      TypeErrorData::CannotCheckDynamicStringFormatCalls(
        CannotCheckDynamicStringFormatCalls::default(),
      ),
      &call_site.base.base.location,
    );
    return true;
  };

  // CLI-150726: The block below effectively constructs a type pack and then type checks it by going parameter-by-parameter.
  let expected = parse_format_string_bytes(context.builtin_types, format_str.as_bytes());

  let (params, _tail) = flatten_type_pack_id(context.arguments);

  let param_offset = 1;
  // Compare the expressions passed with the types the function expects to determine whether this function was called with : or .
  let called_with_self = expected.len() == call_site.args.size;
  // unify the prefix one argument at a time
  for (i, &expected_ty) in expected.iter().enumerate() {
    let Some(&actual_ty) = params.get(i + param_offset) else {
      break;
    };
    // No argument expressions ⇒ nothing to attach a location to, and
    // `args.size - 1` would underflow.
    if call_site.args.is_empty() {
      break;
    }
    let arg_index = min(
      call_site.args.len() - 1,
      i + if called_with_self { 0 } else { param_offset },
    );
    let location = unsafe { (*call_site.args[arg_index]).base.location };
    // use subtyping instead here
    let scope_ptr = context.check_scope.as_ptr();
    // typechecker.subtyping 字段已句柄化（C++ NotNull<Subtyping> 成员，指向 checker
    // 自持的 _subtyping），`subtyping_mut` 收口判空与解引用契约，与本函数体持有的
    // checker &mut 借用同源共存、派生借用止于本次调用；check_scope 的 NonNull 由
    // visit 入栈时构造，Scope 存活。
    let result = typechecker
      .subtyping_mut()
      .is_subtype_type_id_type_id_not_null_scope(actual_ty, expected_ty, scope_ptr);

    if !result.is_subtype {
      // Safety: actual_ty 为 flatten 结果的 arena 活 TypeId；normalizer 句柄
      // 派生自唯一可变借用 typechecker，本次调用点无并存访问。
      match unsafe { should_suppress_errors(&mut typechecker.normalizer as *mut _, actual_ty) }
        .value
      {
        Value::Suppress => {}
        Value::NormalizationFailed => {}
        Value::DoNotSuppress => {
          let reasonings = typechecker
            .explain_reasonings_type_id_type_id_location_subtyping_result(
              actual_ty,
              expected_ty,
              location,
              &result,
            );

          if !reasonings.suppressed {
            let reason = reasonings.to_string();
            typechecker.report_error_type_error_data_location(
              TypeErrorData::TypeMismatch(TypeMismatch::from_wanted_given_reason(
                expected_ty,
                actual_ty,
                reason,
              )),
              &location,
            );
          }
        }
      }
    }
  }

  true
}
