use alloc::vec::Vec;

use crate::{
  enums::table_state::TableState,
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
    get_type_pack, reduce_union::reduce_union,
  },
  records::{
    arena_handle::{alias, alias_ref},
    magic_function_call_context::MagicFunctionCallContext,
    property_type::Property,
    table_indexer::TableIndexer,
    table_type::TableType,
    type_pack::TypePack,
    union_type::UnionType,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{props_type::Props, type_id::TypeId, type_pack_variant::TypePackVariant},
};
pub fn magic_pack_infer(context: &MagicFunctionCallContext) -> bool {
  let solver = alias_ref(context.solver.as_ptr());
  // Safety: `solver.arena` 对应 C++ `NotNull<TypeArena*>`，构造期接线、非空且
  // 比求解器长寿，arena 节点块地址不移动；本可变借用是消解会话内 magic 的
  // 独占句柄——借用创建到首次 `add_type` 之间仅有 flatten/get 的只读访问
  //（读不构成别名冲突），全部写入按语句时序串行，单线程无第二处可变句柄
  // 并发使用。
  let arena = { &mut solver.arena.get_mut() };
  // Safety: `solver.builtin_types` 对应 C++ `NotNull<BuiltinTypes>` 单例，
  // 构造期接线非空、内容只读且比持有者长寿。
  let builtin_types = { &solver.builtin_types.get_mut() };

  let (param_types, param_tail) = flatten_type_pack_id(context.arguments);

  let mut options: Vec<TypeId> = Vec::with_capacity(param_types.len());
  options.extend(param_types);

  if let Some(param_tail) = param_tail
    && let Some(vtp) = get_type_pack::get::<VariadicTypePack>(param_tail)
  {
    options.push(vtp.ty);
  }

  let options = reduce_union(&options);

  let result = if options.is_empty() {
    builtin_types.nil_type
  } else if options.len() == 1 {
    options[0]
  } else {
    arena.add_type(UnionType { options })
  };

  let number_type = builtin_types.number_type;
  let mut props = Props::default();
  props.insert("n".to_string(), Property::rw_type_id(number_type));

  let packed_table = arena.add_type(TableType {
    props,
    indexer: Some(TableIndexer {
      index_type: number_type,
      index_result_type: result,
      is_read_only: false,
    }),
    state: TableState::Sealed,
    ..Default::default()
  });

  let table_type_pack = arena.add_type_pack_t(TypePack::single(packed_table));
  let result_mut = as_mutable_type_pack(context.result);
  alias(result_mut).ty = TypePackVariant::Bound(table_type_pack);

  true
}
