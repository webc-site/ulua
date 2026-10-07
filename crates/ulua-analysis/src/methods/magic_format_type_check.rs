use alloc::string::ToString;
use core::cmp::min;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString,
  ast_expr_index_name::AstExprIndexName, node_handle::OptNode,
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
    arena_handle::{alias, alias_ref},
    cannot_check_dynamic_string_format_calls::CannotCheckDynamicStringFormatCalls,
    count_mismatch::{CountMismatch, CountMismatchContext},
    error_suppression::ErrorSuppression,
    magic_function_type_check_context::MagicFunctionTypeCheckContext,
    type_mismatch::TypeMismatch,
  },
  type_aliases::type_error_data::TypeErrorData,
};
pub fn magic_format_type_check(context: &MagicFunctionTypeCheckContext) -> bool {
  let typechecker = alias(context.typechecker.as_ptr());
  let call_site = alias_ref(context.call_site);

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
  // iter 已过 begin!=end 判定，current 读 arguments TypePack 首个
  // head 元素（arena 活 TypeId）；should_suppress_errors 已前移为 &mut Normalizer
  // 形参，callee 的只读规范化缓存写入限于本次独占借用。
  if should_suppress_errors(
    &mut typechecker.normalizer,
    follow_type::follow(*iter.current()),
  ) == ErrorSuppression::from_value(Value::Suppress)
  {
    return true;
  }

  // 格式串候选以 fn 作用域句柄 `fmt_node` 承载身份，最终 `try_as` 一次兑现借用；
  // null/类位不命中都折叠为 None，借用半径不超过 fn 作用域句柄。
  let mut fmt_node: OptNode<AstExpr> = OptNode::default();
  {
    let func_slot = OptNode::from_ptr(call_site.func);
    if let Some(index_expr) = func_slot.try_as::<AstExprIndexName>()
      && call_site.self_
    {
      // expr 已句柄化恒非空；unwrap_group 行走链为既有裸指针 API，经 as_ptr 桥接。
      // 剥组结果（存活节点或 null）继续以句柄承载，判型收敛到尾部 try_as。
      fmt_node = OptNode::from_ptr(unwrap_group(index_expr.expr.as_ptr()));
    }
  }

  if !call_site.self_
    && let Some(&arg) = call_site.args.first()
  {
    // args 元素是 arena 存活表达式节点；同样以句柄承载身份，类位甄别延后统一。
    fmt_node = OptNode::from_ptr(arg);
  }
  let fmt = fmt_node.try_as::<AstExprConstantString>();

  // 双写合一：原「fflag 分支 is_none 早退」+「else is_none 报告并早退」+
  // 「块后 unwrap()」三段收为一条 let-else——None 走各自原早退路径（含
  // 报告与 return true），Some 直接绑定，逐分支行为等价。
  let Some(format_str) = extract_format_string(fmt, &iter) else {
    if fflag::LuauSilenceDynamicFormatStringErrors.get() {
      return true;
    }
    typechecker.report_error_type_error_data_location(
      TypeErrorData::CannotCheckDynamicStringFormatCalls(CannotCheckDynamicStringFormatCalls),
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
    let location = alias_ref(call_site.args[arg_index]).base.location;
    // use subtyping instead here
    // typechecker.subtyping 字段已句柄化（C++ NotNull<Subtyping> 成员，指向 checker
    // 自持的 _subtyping），`subtyping_mut` 收口判空与解引用契约，与本函数体持有的
    // checker &mut 借用同源共存、派生借用止于本次调用。
    let result = typechecker
      .subtyping_mut()
      .is_subtype_type_id_type_id_not_null_scope(
        actual_ty,
        expected_ty,
        alias_ref(context.check_scope.as_ptr()),
      );

    if !result.is_subtype {
      // actual_ty 为 flatten 结果的 arena 活 TypeId；normalizer 直传 typechecker
      // 的可变借用，本次调用点无并存访问（&mut 形参契约前移后无需 unsafe）。
      match should_suppress_errors(&mut typechecker.normalizer, actual_ty).value {
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
