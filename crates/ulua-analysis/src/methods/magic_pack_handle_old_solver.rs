use alloc::{string::ToString, sync::Arc, vec, vec::Vec};
use core::ptr::null_mut;

use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::{
  enums::table_state::TableState,
  functions::{
    flatten_type_pack::flatten_type_pack_id, get_type_pack::get_type_pack_id,
    reduce_union::reduce_union,
  },
  records::{
    module::Module, property_type::Property, scope::Scope, table_indexer::TableIndexer,
    table_type::TableType, type_checker::TypeChecker, type_pack::TypePack, union_type::UnionType,
    variadic_type_pack::VariadicTypePack, with_predicate::WithPredicate,
  },
  type_aliases::{props_type::Props, type_id::TypeId, type_pack_id::TypePackId},
};
pub fn magic_pack_handle_old_solver(
  typechecker: &mut TypeChecker,
  scope: &Arc<Scope>,
  _expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let param_pack = with_predicate.r#type;

  let module = typechecker.current_module.as_ref()?;
  let arena = unsafe { &mut (*(Arc::as_ptr(module) as *mut Module)).internal_types };

  let (param_types, param_tail) = flatten_type_pack_id(param_pack);

  let mut options: Vec<TypeId> = Vec::with_capacity(param_types.len());
  options.extend(param_types);

  if let Some(param_tail) = param_tail
    && let Some(vtp) = get_type_pack_id::<VariadicTypePack>(param_tail)
  {
    options.push(vtp.ty);
  }

  let options = reduce_union(&options);

  // table.pack()         -> {| n: number, [number]: nil |}
  // table.pack(1)        -> {| n: number, [number]: number |}
  // table.pack(1, "foo") -> {| n: number, [number]: number | string |}
  let result = if options.is_empty() {
    typechecker.nil_type
  } else if options.len() == 1 {
    options[0]
  } else {
    arena.add_type(UnionType { options })
  };

  let mut props = Props::default();
  props.insert(
    "n".to_string(),
    Property::rw_type_id(typechecker.number_type),
  );

  let packed_table = arena.add_type(TableType {
    props,
    indexer: Some(TableIndexer {
      index_type: typechecker.number_type,
      index_result_type: result,
      is_read_only: false,
    }),
    state: TableState::Sealed,
    level: scope.level,
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

  let result_pack = arena.add_type_pack_t(TypePack {
    head: vec![packed_table],
    tail: None,
  });
  Some(WithPredicate::with_predicate_t(result_pack))
}
