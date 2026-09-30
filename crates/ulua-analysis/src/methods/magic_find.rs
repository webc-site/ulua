//! `magic_find` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{vec, vec::Vec};
use core::ptr::NonNull;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_string::AstExprConstantString,
  },
  rtti::ast_node_try_as_ptr,
};

use crate::{
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
    parse_pattern_string::parse_pattern_string_bytes, shared_mut::shared_mut,
  },
  records::{
    magic_find::MagicFind, magic_function_call_context::MagicFunctionCallContext,
    type_checker::TypeChecker, type_pack::TypePack, union_type::UnionType,
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
  let pattern = if expr.args.size > pattern_index {
    // size > pattern_index 保证下标在界内，改用安全切片取元素指针。
    let arg = expr.args.as_slice()[pattern_index];
    // Safety: `arg` 是 arena 写入 args 的存活表达式节点（或 null）；try_as_ptr
    // 先判空再按 class_index 甄别，未命中返回 None 且从不解引用，命中即 repr(C)
    // 基址重合的存活 AstExprConstantString 只读借用，模块活过本次求解、地址不移动。
    unsafe { ast_node_try_as_ptr::<AstExprConstantString>(arg) }
  } else {
    None
  };

  let pattern = pattern?;

  let plain_index = if expr.self_ { 2 } else { 3 };
  let mut plain = false;
  if expr.args.size > plain_index {
    // size > plain_index 保证界内，安全切片读取。
    let arg = expr.args.as_slice()[plain_index];
    // Safety: `arg` 同源于 arena 写入 args 的存活表达式节点指针；try_as_ptr 判空+
    // 甄别，命中即存活 AstExprConstantBool 只读借用，value 为 Copy 字段读取。
    plain = unsafe { ast_node_try_as_ptr::<AstExprConstantBool>(arg) }.is_some_and(|p| p.value);
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

  // Safety: `pattern` 命中意味着上方分支成立，即 args.size > pattern_index ≥ 0，
  // 故第 0 实参在界内；元素是 parser 写入 arena 的存活表达式指针，仅只读其
  // base.location 并交给 unify 记录错误位置。
  let first_location = unsafe { &(*expr.args.as_slice()[0]).base.location };
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
    // Safety: size > init_index 保证切片下标界内，元素是 arena 存活表达式指针，
    // 仅短暂借出其 base.location 只读引用供 unify 记录错误位置。
    let location = unsafe { &(*expr.args.as_slice()[init_index]).base.location };
    typechecker.unify_type_id_type_id_scope_ptr_location(
      params[2],
      optional_number,
      scope,
      location,
    );
  }

  if params.len() == 4 && expr.args.size > plain_index {
    // Safety: size > plain_index 保证界内；该实参节点由 arena 持有、求解期间
    // 稳定，location 只读后即弃，不与 unify 的可变借用重叠。
    let location = unsafe { &(*expr.args.as_slice()[plain_index]).base.location };
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

    // Safety: `context.solver` 是派发本约束时以 NotNull 语义写入的求解器地址，
    // 指向调用栈内存活、覆盖本次 infer 全程的 ConstraintSolver。
    let solver = unsafe { context.solver.as_ref() };
    // Safety: `solver.arena` 构造期指向 solver 内嵌 TypeArena；本次 infer
    // 新增类型/类型包是它的唯一可变借用路径。
    let arena = { &mut solver.arena.get_mut() };
    // Safety: `solver.builtin_types` 为会话级 NotNull 内置类型表，比本次调用
    // 长寿且此处仅读取若干 TypeId 值。
    let builtin_types = { &solver.builtin_types.get_mut() };
    // Safety: `context.call_site` 指向 parser 拥有、求解期间稳定的当前
    // AstExprCall 节点，本函数只读其 self_ 标志与 args。
    let call_site = unsafe { context.call_site.as_ref() };

    let pattern_index = if call_site.self_ { 0 } else { 1 };
    let pattern = if call_site.args.size > pattern_index {
      // size > pattern_index 保证下标在界内，改用安全切片取元素指针。
      let expr = call_site.args.as_slice()[pattern_index];
      // Safety: `expr` 是 arena 写入 args 的存活表达式节点（或 null）；
      // try_as_ptr 先判空再按 class_index 甄别，未命中返回 None 且从不解引用，
      // 命中即 repr(C) 基址重合的存活 AstExprConstantString 只读借用；
      // arena 地址不移动，借用活过本函数。
      unsafe { ast_node_try_as_ptr::<AstExprConstantString>(expr) }
    } else {
      None
    };

    let Some(pattern) = pattern else {
      return false;
    };

    let mut plain = false;
    let plain_index = if call_site.self_ { 2 } else { 3 };
    if call_site.args.size > plain_index {
      // size > plain_index 保证下标在界内，安全切片读取。
      let expr = call_site.args.as_slice()[plain_index];
      // Safety: `expr` 同源于 arena 的存活表达式节点指针；try_as_ptr 判空+甄别，
      // 命中即存活 AstExprConstantBool 的只读借用，value 为 Copy 字段读取。
      if let Some(bool_expr) = unsafe { ast_node_try_as_ptr::<AstExprConstantBool>(expr) } {
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

    // Safety: `as_ptr` 取回仍独占存活的 solver 地址；constraint_solver_unify
    // 需 `&mut self`，本调用同步持有 solver 独占访问，constraint 指针作为
    // 参数交由 callee 自行只读解引用。
    unsafe {
      (*context.solver.as_ptr()).constraint_solver_unify(
        context.constraint.as_ptr(),
        params[0],
        builtin_types.string_type,
      );
    }

    let optional_number = arena.add_type(UnionType {
      options: vec![builtin_types.nil_type, builtin_types.number_type],
    });
    let optional_boolean = arena.add_type(UnionType {
      options: vec![builtin_types.nil_type, builtin_types.boolean_type],
    });

    let init_index = if call_site.self_ { 1 } else { 2 };
    if params.len() >= 3 && call_site.args.size > init_index {
      // Safety: 同上——solver 地址本次调用内独占，unify 借用其 &mut self
      // 向约束集追加，无并发别名。
      unsafe {
        (*context.solver.as_ptr()).constraint_solver_unify(
          context.constraint.as_ptr(),
          params[2],
          optional_number,
        );
      }
    }

    if params.len() == 4 && call_site.args.size > plain_index {
      // Safety: 同前两处 unify——对独占存活 solver 的可变借用，向约束集追加
      // 第 4 参数的可选布尔。
      unsafe {
        (*context.solver.as_ptr()).constraint_solver_unify(
          context.constraint.as_ptr(),
          params[3],
          optional_boolean,
        );
      }
    }

    return_types.insert(0, optional_number);
    return_types.insert(1, optional_number);

    let return_list = arena.add_type_pack_vector_type_id_optional_type_pack_id(return_types, None);
    let result_mut = as_mutable_type_pack(context.result);
    // Safety: `context.result` 是 solver 为本约束分配、尚待定版的 arena
    // TypePackVar，TypePackId→*mut 对应 cpp `asMutable` 的 const_cast，
    // 本次 infer 独占写其 `ty` 为 Bound。
    unsafe {
      (*result_mut).ty = TypePackVariant::Bound(return_list);
    }

    true
  }
}
