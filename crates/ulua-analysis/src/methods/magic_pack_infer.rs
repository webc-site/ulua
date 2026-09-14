use alloc::{vec, vec::Vec};
use core::ptr::null_mut;

use crate::{
  enums::table_state::TableState,
  functions::{
    as_mutable_type_pack_alt_d::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
    get_type_pack::get_type_pack_id, reduce_union::reduce_union,
  },
  records::{
    magic_function_call_context::MagicFunctionCallContext, property_type::Property,
    table_indexer::TableIndexer, table_type::TableType, type_level::TypeLevel, type_pack::TypePack,
    union_type::UnionType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{props_type::Props, type_id::TypeId, type_pack_variant::TypePackVariant},
};
pub fn magic_pack_infer(context: &MagicFunctionCallContext) -> bool {
  let solver = unsafe { context.solver.as_ref() };
  let arena = unsafe { &mut *solver.arena };
  let builtin_types = unsafe { &*solver.builtin_types };

  let (param_types, param_tail) = flatten_type_pack_id(context.arguments);

  let mut options: Vec<TypeId> = Vec::with_capacity(param_types.len());
  options.extend(param_types);

  if let Some(param_tail) = param_tail
    && let Some(vtp) = get_type_pack_id::<VariadicTypePack>(param_tail)
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
    level: TypeLevel::default(),
    scope: null_mut(),
    name: None,
    synthetic_name: None,
    instantiated_type_params: Vec::new(),
    instantiated_type_pack_params: Vec::new(),
    definition_module_name: Default::default(),
    definition_location: Default::default(),
    bound_to: None,
    tags: Default::default(),
    remaining_props: 0,
  });

  let table_type_pack = arena.add_type_pack_t(TypePack {
    head: vec![packed_table],
    tail: None,
  });
  let result_mut = as_mutable_type_pack(context.result);
  unsafe {
    (*result_mut).ty = TypePackVariant::Bound(table_type_pack);
  }

  true
}
