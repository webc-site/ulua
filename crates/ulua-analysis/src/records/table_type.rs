//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Type.h:488:table_type`
//! Source: `Analysis/include/Luau/Type.h` (Type.h:488-524, hand-ported)

use alloc::{string::String, vec::Vec};

use ulua_ast::records::location::Location;

use crate::{
  enums::table_state::TableState,
  records::{scope::Scope, table_indexer::TableIndexer, type_level::TypeLevel},
  type_aliases::{
    module_name_type::ModuleName, props_type::Props, tags::Tags, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

#[derive(Debug, Clone)]
pub struct TableType {
  pub props: Props,
  pub indexer: Option<TableIndexer>,

  pub state: TableState,
  pub level: TypeLevel,
  pub scope: *mut Scope,
  pub name: Option<String>,

  /// Sometimes we throw a type on a name to make for nicer error messages,
  /// but without creating any entry in the type namespace.
  pub synthetic_name: Option<String>,

  pub instantiated_type_params: Vec<TypeId>,
  pub instantiated_type_pack_params: Vec<TypePackId>,
  pub definition_module_name: ModuleName,
  pub definition_location: Location,

  pub bound_to: Option<TypeId>,
  pub tags: Tags,

  /// Number of as-yet-unadded properties on unsealed tables; some
  /// constraints use this to decide whether they can dispatch.
  pub remaining_props: usize,
}
