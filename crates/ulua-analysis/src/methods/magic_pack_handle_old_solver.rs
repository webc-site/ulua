use alloc::{string::ToString, sync::Arc, vec::Vec};

use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::{
  enums::table_state::TableState,
  functions::{
    arc_as_mut::arc_as_mut, flatten_type_pack::flatten_type_pack_id, get_type_pack,
    reduce_union::reduce_union,
  },
  records::{
    property_type::Property, scope::Scope, table_indexer::TableIndexer, table_type::TableType,
    type_checker::TypeChecker, type_pack::TypePack, union_type::UnionType,
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
  let arena = unsafe { &mut (*(arc_as_mut(module))).internal_types };

  let (param_types, param_tail) = flatten_type_pack_id(param_pack);

  let mut options: Vec<TypeId> = Vec::with_capacity(param_types.len());
  options.extend(param_types);

  if let Some(param_tail) = param_tail
    && let Some(vtp) = get_type_pack::get::<VariadicTypePack>(param_tail)
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
    ..Default::default()
  });

  let result_pack = arena.add_type_pack_t(TypePack::single(packed_table));
  Some(WithPredicate::with_predicate_t(result_pack))
}
