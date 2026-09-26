use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
    follow_type, get_mutable_type, get_type, shallow_clone_clone::shallow_clone,
    track_interior_free_type::track_interior_free_type,
  },
  records::{
    clone_state::CloneState, count_mismatch::CountMismatch,
    magic_function_call_context::MagicFunctionCallContext, table_type::TableType,
    type_pack::TypePack,
  },
  type_aliases::{type_error_data::TypeErrorData, type_pack_variant::TypePackVariant},
};
pub fn magic_clone_infer(context: &MagicFunctionCallContext) -> bool {
  // Safety: `context.solver` 是构造 `MagicFunctionCallContext` 时由调用方注入的
  // `NonNull<ConstraintSolver>`，指向本次 magic 求解期间存活的约束求解器；只读取共享引用。
  let solver = unsafe { context.solver.as_ref() };
  // Safety: `solver.arena` 对应 C++ `NotNull<TypeArena>`，非空且块地址在遍历期不移动；
  // 本函数单线程串行，向其后各调用提供独占可变借用，无别名冲突。
  let arena = { &mut solver.arena.get_mut() };
  // Safety: `context.call_site` 为注入的 `NonNull<AstExprCall>`，指向存活 AST 调用节点，
  // 只读共享借用。
  let call_site = unsafe { context.call_site.as_ref() };

  let (param_types, _param_tail) = flatten_type_pack_id(context.arguments);
  if param_types.is_empty() || call_site.args.size == 0 {
    // Safety: `context.solver.as_ptr()` 即上面同源的存活 `ConstraintSolver`；
    // `report_error_*` 在单线程求解上下文内独占调用，`&call_site.arg_location` 只读活引用。
    unsafe {
      (*context.solver.as_ptr()).report_error_type_error_data_location(
        TypeErrorData::CountMismatch(CountMismatch {
          expected: 1,
          actual: 0,
          ..Default::default()
        }),
        &call_site.arg_location,
      );
    }
    return false;
  }

  let input_type = follow_type::follow(param_types[0]);

  if get_type::get::<TableType>(input_type).is_none() {
    return false;
  }

  let mut clone_state = CloneState {
    builtin_types: solver.builtin_types,
    seen_types: DenseHashMap::default(),
    seen_type_packs: DenseHashMap::default(),
  };
  // Safety: `shallow_clone` 要求 arena 有效可写、clone_state.builtin_types 非空且输入
  // 类型为 arena 存活 `TypeId`。`arena` 是上面重建的独占可变借用（非空、块地址稳定），
  // `clone_state.builtin_types` 取自 `solver.builtin_types`（`NotNull` 单例），
  // `input_type` 经 `follow_type_id` 解析为存活节点，全程单线程。
  let result_type = unsafe {
    shallow_clone(
      input_type,
      arena,
      &mut clone_state,
      /* ignorePersistent */ true,
    )
  };

  // Safety: `context.constraint` 为注入的 `NonNull<Constraint>`，指向存活约束节点，
  // 此处仅只读其 `scope` 字段。
  let constraint_scope = unsafe { (*context.constraint.as_ptr()).scope };

  if let Some(table_type) = get_mutable_type::get_mutable::<TableType>(result_type) {
    table_type.scope = constraint_scope;
  }

  track_interior_free_type(constraint_scope, result_type);

  let cloned_type_pack = arena.add_type_pack_t(TypePack::single(result_type));
  let result_mut = as_mutable_type_pack(context.result);
  // Safety: `as_mutable_type_pack` 将 `context.result` 这一 arena 存活的 `TypePackId`
  // 原样转成 `*mut TypePackVar`（arena 节点地址不移动）；本函数单线程独占，原地写其
  // `.ty` 为 Bound 无其它活动别名。
  unsafe {
    (*result_mut).ty = TypePackVariant::Bound(cloned_type_pack);
  }

  true
}
