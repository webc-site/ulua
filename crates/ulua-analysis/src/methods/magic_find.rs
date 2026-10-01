//! `magic_find` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{vec, vec::Vec};
use core::ptr::NonNull;

use ulua_ast::records::{
  ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool,
  ast_expr_constant_string::AstExprConstantString, node_handle::OptNode,
};

use crate::{
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
    parse_pattern_string::parse_pattern_string_bytes, shared_mut::shared_mut,
  },
  records::{
    arena_handle::{alias, alias_ref},
    magic_find::MagicFind,
    magic_function_call_context::MagicFunctionCallContext,
    type_checker::TypeChecker,
    type_pack::TypePack,
    union_type::UnionType,
    with_predicate::WithPredicate,
  },
  type_aliases::{
    scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant,
  },
};

pub fn magic_find_handle_old_solver(
  typechecker: &mut TypeChecker,
  scope: &ScopePtr,
  expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let param_pack = with_predicate.r#type;
  let (params, _tail) = flatten_type_pack_id(param_pack);

  if params.len() < 2 || params.len() > 4 {
    return None;
  }

  let module = typechecker.current_module.as_ref()?;
  // Safety: `shared_mut(module)` 返回 Arc<Module> 内嵌 Module 的裸地址（Arc 由
  // `current_module` 持有、`module` 借用覆盖全函数，指针非空对齐且存活）。取
  // `&mut internal_types` 是 C++ `asMutable(module)->internalTypes` 的等价惯用法：
  // 全函数单线程串行，此 arena 无第二处可变句柄并存。
  let arena = { &mut (shared_mut(module)).internal_types };

  let pattern_index = if expr.self_ { 0 } else { 1 };
  // args 槽仍是 records 引用化波次前的裸指针数组：元素经局部 `OptNode` 句柄
  // 折叠判空，下转走生命周期正确的 `try_as`（判型不命中折叠为 None，与旧
  // try_as_ptr 语义逐格一致）；句柄置于函数作用域，借用半径覆盖 pattern 全程
  // 使用，不再锻造 'static。
  let pattern_slot = if expr.args.size > pattern_index {
    // size > pattern_index 保证下标在界内，改用安全切片取元素指针。
    OptNode::from_ptr(expr.args.as_slice()[pattern_index])
  } else {
    OptNode::default()
  };

  let pattern = pattern_slot.try_as::<AstExprConstantString>()?;

  let plain_index = if expr.self_ { 2 } else { 3 };
  let mut plain = false;
  if expr.args.size > plain_index {
    // size > plain_index 保证界内，安全切片读取。
    let arg = expr.args.as_slice()[plain_index];
    // 同上：局部句柄折叠判空 + 生命周期正确 try_as，value 为 Copy 字段读取。
    plain = OptNode::from_ptr(arg)
      .try_as::<AstExprConstantBool>()
      .is_some_and(|p| p.value);
  }

  let mut return_types: Vec<TypeId> = Vec::new();
  if !plain {
    return_types = unsafe {
      // Safety: `typechecker.builtin_types` 为 Handle（NonNull 编码非空）持有的
      // 会话级内置类型表，比本次调用长寿，as_ptr 还原裸地址后 new_unchecked
      // 不变量由句柄成立；`pattern` 经 try_as_ptr 命中即指向存活
      // AstExprConstantString，其 `value` 字节区成对有效，仅只读用于模式串解析。
      parse_pattern_string_bytes(
        NonNull::new_unchecked(typechecker.builtin_types.as_ptr()),
        pattern.value.as_bytes(),
      )
    };

    if return_types.is_empty() {
      return None;
    }
  }

  // pattern 命中意味着上方分支成立，即 args.size > pattern_index ≥ 0，
  // 第 0 实参在界内；仅只读其 base.location 并交给 unify 记录错误位置。
  let first_location = &alias_ref(expr.args.as_slice()[0]).base.location;
  typechecker.unify_type_id_type_id_scope_ptr_location(
    params[0],
    typechecker.string_type,
    scope,
    first_location,
  );

  let optional_number = arena.add_type(UnionType {
    options: vec![typechecker.nil_type, typechecker.number_type],
  });
  let optional_boolean = arena.add_type(UnionType {
    options: vec![typechecker.nil_type, typechecker.boolean_type],
  });

  let init_index = if expr.self_ { 1 } else { 2 };
  if params.len() >= 3 && expr.args.size > init_index {
    // size > init_index 保证切片下标界内，仅短暂借出其 base.location
    // 只读引用供 unify 记录错误位置。
    let location = &alias_ref(expr.args.as_slice()[init_index]).base.location;
    typechecker.unify_type_id_type_id_scope_ptr_location(
      params[2],
      optional_number,
      scope,
      location,
    );
  }

  if params.len() == 4 && expr.args.size > plain_index {
    // size > plain_index 保证界内；location 只读后即弃，不与 unify 的可变借用重叠。
    let location = &alias_ref(expr.args.as_slice()[plain_index]).base.location;
    typechecker.unify_type_id_type_id_scope_ptr_location(
      params[3],
      optional_boolean,
      scope,
      location,
    );
  }

  return_types.insert(0, optional_number);
  return_types.insert(1, optional_number);

  let return_list = arena.add_type_pack_t(TypePack::from_vec(return_types));
  Some(WithPredicate::with_predicate_t(return_list))
}

impl MagicFind {
  pub fn infer(&self, context: &MagicFunctionCallContext) -> bool {
    let (params, _tail) = flatten_type_pack_id(context.arguments);

    if params.len() < 2 || params.len() > 4 {
      return false;
    }

    let solver = alias_ref(context.solver.as_ptr());
    // Safety: `solver.arena` 构造期指向 solver 内嵌 TypeArena；本次 infer
    // 新增类型/类型包是它的唯一可变借用路径。
    let arena = { &mut solver.arena.get_mut() };
    // Safety: `solver.builtin_types` 为会话级 NotNull 内置类型表，比本次调用
    // 长寿且此处仅读取若干 TypeId 值。
    let builtin_types = { &solver.builtin_types.get_mut() };
    let call_site = alias_ref(context.call_site.as_ptr());

    let pattern_index = if call_site.self_ { 0 } else { 1 };
    // args 元素经局部 `OptNode` 句柄折叠判空，下转走生命周期正确的 `try_as`；
    // 句柄置于本帧作用域，借用覆盖 pattern 全程使用。
    let pattern_slot = if call_site.args.size > pattern_index {
      // size > pattern_index 保证下标在界内，改用安全切片取元素指针。
      OptNode::from_ptr(call_site.args.as_slice()[pattern_index])
    } else {
      OptNode::default()
    };

    let Some(pattern) = pattern_slot.try_as::<AstExprConstantString>() else {
      return false;
    };

    let mut plain = false;
    let plain_index = if call_site.self_ { 2 } else { 3 };
    if call_site.args.size > plain_index {
      // size > plain_index 保证下标在界内，安全切片读取。
      let expr = call_site.args.as_slice()[plain_index];
      // 同上：局部句柄折叠判空 + 生命周期正确 try_as，value 为 Copy 字段读取。
      if let Some(bool_expr) = OptNode::from_ptr(expr).try_as::<AstExprConstantBool>() {
        plain = bool_expr.value;
      }
    }

    let mut return_types: Vec<TypeId> = Vec::new();
    if !plain {
      // Safety: `pattern` 已由 try_as_ptr 命中证明为存活 AstExprConstantString，
      // `value.as_bytes()` 返回 arena 成对写入的合法字节区，仅只读用于模式串解析。
      let pattern_bytes = pattern.value.as_bytes();
      // Safety: `solver.builtin_types` 为构造期以 NotNull 语义持有的会话级
      // 内置类型表，非空且比本次调用长寿，new_unchecked 不变量由 solver 保证。
      let builtin_types_nn = unsafe { NonNull::new_unchecked(solver.builtin_types.as_ptr()) };
      return_types = parse_pattern_string_bytes(builtin_types_nn, pattern_bytes);

      if return_types.is_empty() {
        return false;
      }
    }

    alias(context.solver.as_ptr()).constraint_solver_unify(
      context.constraint.as_ptr(),
      params[0],
      builtin_types.string_type,
    );

    let optional_number = arena.add_type(UnionType {
      options: vec![builtin_types.nil_type, builtin_types.number_type],
    });
    let optional_boolean = arena.add_type(UnionType {
      options: vec![builtin_types.nil_type, builtin_types.boolean_type],
    });

    let init_index = if call_site.self_ { 1 } else { 2 };
    if params.len() >= 3 && call_site.args.size > init_index {
      alias(context.solver.as_ptr()).constraint_solver_unify(
        context.constraint.as_ptr(),
        params[2],
        optional_number,
      );
    }

    if params.len() == 4 && call_site.args.size > plain_index {
      alias(context.solver.as_ptr()).constraint_solver_unify(
        context.constraint.as_ptr(),
        params[3],
        optional_boolean,
      );
    }

    return_types.insert(0, optional_number);
    return_types.insert(1, optional_number);

    let return_list = arena.add_type_pack_vector_type_id_optional_type_pack_id(return_types, None);
    let result_mut = as_mutable_type_pack(context.result);
    alias(result_mut).ty = TypePackVariant::Bound(return_list);

    true
  }
}
