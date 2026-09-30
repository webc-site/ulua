use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString,
  },
  rtti::ast_node_try_as_ptr,
};

use crate::{
  functions::flatten_type_pack::flatten_type_pack_id,
  records::{
    generic_error::GenericError, type_checker::TypeChecker, type_error::TypeError,
    with_predicate::WithPredicate,
  },
  type_aliases::{
    scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
pub fn magic_select_handle_old_solver(
  typechecker: &mut TypeChecker,
  scope: &ScopePtr,
  expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let (param_pack, _predicates) = (with_predicate.r#type, with_predicate.predicates);

  let _ = scope;

  if expr.args.size == 0 {
    typechecker.report_error_type_error(&TypeError::type_error_location_type_error_data(
      expr.base.base.location,
      TypeErrorData::GenericError(GenericError::new(
        "select should take 1 or more arguments".to_string(),
      )),
    ));
    return None;
  }

  // 上方 `size == 0` 早退保证 args 非空，改用安全切片取首元素表达式指针。
  let arg1 = expr.args.as_slice()[0];
  // Safety: `arg1` 是 parser 写入 arena 的存活非空 `*mut AstExpr`；try_as_ptr 先判空
  // 再按 class_index 甄别，命中即 repr(C) 首字段基址重合的存活 AstExprConstantNumber
  // 只读借用，未命中返回 None（对应 C++ `arg1->as<AstExprConstantNumber>()` 返 null）。
  if let Some(num) = unsafe { ast_node_try_as_ptr::<AstExprConstantNumber>(arg1) } {
    let (params, tail) = flatten_type_pack_id(param_pack);

    // `num` 由 try_as_ptr 命中，只读 `Copy` 的 `value` 字段（C++ `int(num->value)`）。
    let offset = num.value as i32;
    if offset > 0 {
      let offset_usize = offset as usize;
      if offset_usize < params.len() {
        let result: Vec<TypeId> = params[offset_usize..].to_vec();
        return Some(WithPredicate::with_predicate_t(
          typechecker.add_type_pack_vector_type_id_optional_type_pack_id(&result, tail),
        ));
      } else if let Some(tail) = tail {
        // 双写合一：`is_some()` 判定与取值并为一次 `let Some`（tail 为 Copy 指针别名）。
        return Some(WithPredicate::with_predicate_t(tail));
      }
    }

    typechecker.report_error_type_error(&TypeError::type_error_location_type_error_data(
      // 同一命中节点沿 repr(C) 基链只读其 location（即 C++ `arg1->location`）。
      num.base.base.location,
      TypeErrorData::GenericError(GenericError::new(
        "bad argument #1 to select (index out of range)".to_string(),
      )),
    ));
  } else if let Some(str_expr) = unsafe { ast_node_try_as_ptr::<AstExprConstantString>(arg1) }
    && str_expr.value.as_bytes() == b"#"
  {
    return Some(WithPredicate::with_predicate_t(
      typechecker.add_type_pack_initializer_list_type_id(&[typechecker.number_type]),
    ));
  }

  None
}
