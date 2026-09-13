use crate::{
  enums::table_state::TableState,
  records::{
    scope::Scope, table_indexer::TableIndexer, table_type::TableType, type_level::TypeLevel,
  },
  type_aliases::props_type::Props,
};

impl TableType {
  pub fn table_type_props_optional_table_indexer_type_level_scope_table_state(
    props: &Props,
    indexer: Option<TableIndexer>,
    level: TypeLevel,
    scope: *mut Scope,
    state: TableState,
  ) -> Self {
    TableType {
      props: props.clone(),
      indexer,
      state,
      level,
      scope,
      name: None,
      synthetic_name: None,
      instantiated_type_params: Default::default(),
      instantiated_type_pack_params: Default::default(),
      definition_module_name: Default::default(),
      definition_location: Default::default(),
      bound_to: None,
      tags: Default::default(),
      remaining_props: 0,
    }
  }
}
