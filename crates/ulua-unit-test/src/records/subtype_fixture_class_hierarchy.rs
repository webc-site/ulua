use ulua_analysis::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy)]
pub struct SubtypeFixtureClassHierarchy {
  pub root_class: TypeId,
  pub child_class: TypeId,
  pub grandchild_one_class: TypeId,
  pub grandchild_two_class: TypeId,
  pub another_child_class: TypeId,
  pub another_grandchild_one_class: TypeId,
  pub another_grandchild_two_class: TypeId,
}
