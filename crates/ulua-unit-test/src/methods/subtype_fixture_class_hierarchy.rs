use crate::records::{
  subtype_fixture::SubtypeFixture, subtype_fixture_class_hierarchy::SubtypeFixtureClassHierarchy,
};

impl SubtypeFixture {
  pub fn class_hierarchy(&mut self) -> SubtypeFixtureClassHierarchy {
    let root_class = self.cls_string_optional_type_id("Root", None);
    let child_class = self.cls_string_optional_type_id("Child", Some(root_class));
    let grandchild_one_class = self.cls_string_optional_type_id("GrandchildOne", Some(child_class));
    let grandchild_two_class = self.cls_string_optional_type_id("GrandchildTwo", Some(child_class));
    let another_child_class = self.cls_string_optional_type_id("AnotherChild", Some(root_class));
    let another_grandchild_one_class =
      self.cls_string_optional_type_id("AnotherGrandchildOne", Some(another_child_class));
    let another_grandchild_two_class =
      self.cls_string_optional_type_id("AnotherGrandchildTwo", Some(another_child_class));

    SubtypeFixtureClassHierarchy {
      root_class,
      child_class,
      grandchild_one_class,
      grandchild_two_class,
      another_child_class,
      another_grandchild_one_class,
      another_grandchild_two_class,
    }
  }
}
