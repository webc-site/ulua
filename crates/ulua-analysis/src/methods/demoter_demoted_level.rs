use crate::records::{demoter::Demoter, type_level::TypeLevel};

impl Demoter {
  pub fn demoted_level(&mut self, level: TypeLevel) -> TypeLevel {
    TypeLevel {
      level: level.level + 5000,
      sub_level: level.sub_level,
    }
  }
}
