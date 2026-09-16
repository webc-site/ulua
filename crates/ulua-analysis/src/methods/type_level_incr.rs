use crate::records::type_level::TypeLevel;

impl TypeLevel {
  pub fn incr(&self) -> TypeLevel {
    TypeLevel {
      level: self.level + 1,
      sub_level: 0,
    }
  }
}
