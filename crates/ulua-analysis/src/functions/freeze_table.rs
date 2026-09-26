use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  enums::table_state::TableState,
  functions::{follow_type, get_mutable_type, get_type, shallow_clone_clone::shallow_clone},
  records::{
    clone_state::CloneState, magic_function_call_context::MagicFunctionCallContext,
    metatable_type::MetatableType, table_type::TableType, type_mismatch::TypeMismatch,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};
pub fn freeze_table(input_type: TypeId, context: &MagicFunctionCallContext) -> Option<TypeId> {
  // Safety: `context.solver` 是消解期 magic 分发注入的 NotNull 语义
  // `NonNull<ConstraintSolver>`，指向本次调用栈上存活的求解器；此处共享借用
  // 只读取其 `arena` 字段（C++ `solver.arena` 解引用等价），调用串行单线程。
  let solver = unsafe { context.solver.as_ref() };
  // Safety: `solver.arena` 对应 C++ `NotNull<TypeArena*>`，构造期接线、非空且
  // 比求解器长寿；类型 arena 节点块地址不移动。对本 arena 的写穿全部时序
  // 串行：上方 Metatable 分支的递归帧只在其调用内短借 arena，返回后本帧才在
  // 下方 `shallow_clone` 处再使用，单线程下无并存的可变访问。
  let arena = { &mut solver.arena.get_mut() };
  let input_type = follow_type::follow(input_type);

  if let Some(mt) = get_type::get::<MetatableType>(input_type) {
    let mt_table = mt.table;
    let frozen_table = freeze_table(mt_table, context)?;

    // Safety: 函数头已证 `solver.arena` 非空存活；此为重借后的独占短借用，
    // 仅 add_type 一次，上方递归帧的 arena 借用已随其返回结束，时序串行。
    let result_type = solver.arena.get_mut().add_type(MetatableType {
      table: frozen_table,
      metatable: mt.metatable,
      synthetic_name: mt.synthetic_name.clone(),
    });

    return Some(result_type);
  }

  if get_type::get::<TableType>(input_type).is_some() {
    // Clone the input type, this will become our final result type after we mutate it.
    let mut clone_state = CloneState {
      builtin_types: solver.builtin_types,
      seen_types: DenseHashMap::default(),
      seen_type_packs: DenseHashMap::default(),
    };
    // Safety: `shallow_clone` 为 unsafe fn，契约是 arena 有效且 input_type 指向
    // 存活 arena 节点——arena 即函数头证过的 `solver.arena` 独占借用（本帧内
    // 递归帧的 arena 短借用均已随返回结束），input_type 已经 `follow_type_id`
    // 收敛为存活 TypeId；clone_state 的 seen 表为空表初始、随本调用栈存活。
    let result_type = unsafe {
      shallow_clone(
        input_type,
        arena,
        &mut clone_state,
        /* ignorePersistent */ true,
      )
    };
    let table_ty = get_mutable_type::get_mutable::<TableType>(result_type);
    // `clone` should not break this.
    LUAU_ASSERT!(table_ty.is_some());
    // 紧邻 LUAU_ASSERT 蕴含 Some。
    let table_ty = table_ty.expect("紧邻 LUAU_ASSERT(table_ty.is_some()) 蕴含");
    table_ty.state = TableState::Sealed;

    // We'll mutate the table to make every property type read-only.
    table_ty.props.retain(|_name, prop| !prop.is_write_only());
    for prop in table_ty.props.values_mut() {
      prop.write_ty = None;
    }

    return Some(result_type);
  }

  if !fflag::LuauTableFreezeCheckIsSubtype.get() {
    // Safety: `context.call_site` 是 magic 分发注入的 NotNull
    // `NonNull<AstExprCall>`，指向 AST arena 中本次调用存活的调用表达式节点；
    // 共享借用只读其 arg_location。
    let call_site = unsafe { context.call_site.as_ref() };
    // Safety: `solver.builtin_types` 对应 C++ `NotNull<BuiltinTypes>`，构造期
    // 接线、非空且比求解器长寿；只读取 table_type 一个内置 TypeId。
    let table_type = solver.builtin_types.get().table_type;
    // Safety: `context.solver` 同上为存活求解器的 NotNull 句柄；函数头的共享
    // 借用不构成并存可变别名（其生命周期止于各自语句），report_error 仅向
    // solver 的错误列表顺序追加，单线程串行。
    unsafe {
      (*context.solver.as_ptr()).report_error_type_error_data_location(
        TypeErrorData::TypeMismatch(TypeMismatch::from_wanted_given(table_type, input_type)),
        &call_site.arg_location,
      );
    }
  }
  None
}
