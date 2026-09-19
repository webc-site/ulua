use alloc::sync::Arc;

use ulua_ast::records::location::Location;

use crate::{
  records::{class_user_data::ClassUserData, table_indexer::TableIndexer},
  type_aliases::{
    module_name_type::ModuleName, name_type::Name, nominal_relation::NominalRelation,
    props_type::Props, tags::Tags, type_id::TypeId,
  },
};

#[derive(Debug, Clone)]
pub struct ExternType {
  pub name: Name,
  pub props: Props,
  pub parent: Option<TypeId>,
  pub metatable: Option<TypeId>,
  pub tags: Tags,
  pub user_data: Option<Arc<ClassUserData>>,
  pub definition_module_name: ModuleName,
  pub definition_location: Option<Location>,
  pub indexer: Option<TableIndexer>,
  /// This field represents a bidirectional relationship between classes and object types
  /// Given a Class, this relation should be a Obj in the variant, representing an instantiation of the class
  /// Given a Object, this relation should be a Klass in the variant, representing the class prototype
  /// Other sources of Extern Types will not have this relation set - this is for the classes fixture so that
  /// we can go between class and object easily, given just the extern type
  pub relation: Option<NominalRelation>,
}
