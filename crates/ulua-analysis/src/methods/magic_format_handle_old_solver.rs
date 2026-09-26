use alloc::vec::Vec;
use core::ptr::NonNull;
use std::cmp::min;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
    ast_expr_group::AstExprGroup, ast_expr_index_name::AstExprIndexName,
  },
  rtti::ast_node_try_as_ptr,
};

use crate::{
  functions::{
    arc_as_mut::arc_as_mut, flatten_type_pack::flatten_type_pack_id,
    parse_format_string::parse_format_string,
  },
  records::{
    count_mismatch::{CountMismatch, CountMismatchContext},
    type_checker::TypeChecker,
    type_error::TypeError,
    with_predicate::WithPredicate,
  },
  type_aliases::{
    scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
pub fn magic_format_handle_old_solver(
  typechecker: &mut TypeChecker,
  scope: &ScopePtr,
  expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let (param_pack, _predicates) = (with_predicate.r#type, with_predicate.predicates);

  let module = typechecker.current_module.as_ref()?;
  // Safety: `arc_as_mut(module)` 返回 `Arc<Module>` 内嵌 Module 的裸地址（Arc 由
  // `current_module` 持有、`module` 借用覆盖全函数，指针非空对齐且存活）。取
  // `&mut internal_types` 是 C++ `asMutable(module)->internalTypes` 的等价惯用法：
  // 全函数单线程串行，此 arena 无第二处可变句柄并存。
  let arena = unsafe { &mut (*(arc_as_mut(module))).internal_types };

  let mut fmt: Option<&AstExprConstantString> = None;

  if expr.self_ {
    // Safety: `expr.func` 是 parser 写入 arena 的存活调用目标节点（C++
    // `AstExprCall::func` 非 optional）；try_as_ptr 判空+class_index 甄别，命中即
    // repr(C) 基址重合的存活派生节点只读借用，未命中返回 None。
    if let Some(index) = unsafe { ast_node_try_as_ptr::<AstExprIndexName>(expr.func) } {
      // Safety: `index.expr` 为命中后存活的 AstExprIndexName 子节点，parser 保证
      // 非空；同样经 class_index 判型，未命中返回 None。
      let group = unsafe { ast_node_try_as_ptr::<AstExprGroup>(index.expr) };
      if let Some(group) = group {
        // Safety: `group.expr` 是 parser 保证非空的 AstExprGroup 子节点，命中即
        // repr(C) 基址重合、类型正确的存活节点借用。
        fmt = unsafe { ast_node_try_as_ptr::<AstExprConstantString>(group.expr) };
      } else {
        fmt = unsafe { ast_node_try_as_ptr::<AstExprConstantString>(index.expr) };
      }
    }
  }

  if !expr.self_ && expr.args.size > 0 {
    // size > 0 保证第 0 实参在界内，改用安全切片取元素指针。
    let arg0 = expr.args.as_slice()[0];
    // Safety: `arg0` 是 parser 写入 arena 的存活表达式节点；try_as_ptr 判空+甄别，
    // 命中即存活 AstExprConstantString 只读借用。
    fmt = unsafe { ast_node_try_as_ptr::<AstExprConstantString>(arg0) };
  }

  let fmt = fmt?;

  let expected: Vec<TypeId> = unsafe {
    // Safety: `typechecker.builtin_types` 为 Handle（NonNull 编码非空）持有的
    // 会话级内置类型表，比本次调用长寿，as_ptr 还原裸地址后 new_unchecked
    // 不变量由句柄成立；`fmt` 经 try_as_ptr 命中，指向存活 AstExprConstantString，
    // 其 value.data/size 由 arena 成对写入，仅只读用于格式串解析。
    parse_format_string(
      NonNull::new_unchecked(typechecker.builtin_types.as_ptr()),
      fmt.value.data,
      fmt.value.size,
    )
  };

  let (params, tail) = flatten_type_pack_id(param_pack);

  let param_offset: usize = 1;
  let data_offset: usize = if expr.self_ { 0 } else { 1 };

  for (i, &expected_ty) in expected.iter().enumerate() {
    let Some(param) = params.get(i + param_offset) else {
      break;
    };
    // No argument expressions ⇒ nothing to attach a location to, and
    // `args.size - 1` would underflow (the self-call path lacks the
    // `args.size > 0` guard the non-self path has).
    if expr.args.size == 0 {
      break;
    }

    let arg_index = min(i + data_offset, expr.args.size - 1);
    // Safety: arg_index ≤ args.size-1 保证在界内；size>0 已由上方早退保证，
    // as_slice 安全取元素指针；元素是 parser 写入 arena 的存活表达式节点，
    // 仅只读其 base.location 交给 unify 记录错误位置。
    let location = unsafe { &(*expr.args.as_slice()[arg_index as usize]).base.location };

    typechecker.unify_type_id_type_id_scope_ptr_location(*param, expected_ty, scope, location);
  }

  let num_actual_params = params.len();
  let num_expected_params = expected.len() + 1;

  if num_expected_params != num_actual_params
    && (!tail.is_some() || num_expected_params < num_actual_params)
  {
    let error = TypeError::type_error_location_type_error_data(
      expr.base.base.location,
      TypeErrorData::CountMismatch(CountMismatch {
        expected: num_expected_params,
        maximum: None,
        actual: num_actual_params,
        context: CountMismatchContext::Arg,
        is_variadic: false,
        function: String::new(),
      }),
    );
    typechecker.report_error_type_error(&error);
  }

  Some(WithPredicate::with_predicate_t(
    arena.add_type_pack_initializer_list_type_id(&[typechecker.string_type]),
  ))
}
