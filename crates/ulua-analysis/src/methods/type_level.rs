//! `type_level` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::records::type_level::TypeLevel;

impl TypeLevel {
  pub fn incr(&self) -> TypeLevel {
    TypeLevel {
      level: self.level + 1,
      sub_level: 0,
    }
  }
}

impl TypeLevel {
  pub fn subsumes(&self, rhs: &TypeLevel) -> bool {
    if self.level < rhs.level {
      return true;
    }
    if self.level > rhs.level {
      return false;
    }
    if self.sub_level == rhs.sub_level {
      return true; // if level == rhs.level and sub_level == rhs.sub_level, then they are the exact same TypeLevel
    }

    // Sibling TypeLevels (that is, TypeLevels that share a level but have a different sub_level) are not considered to subsume one another
    false
  }
}

impl TypeLevel {
  pub fn subsumes_strict(&self, rhs: &TypeLevel) -> bool {
    if self.level == rhs.level && self.sub_level == rhs.sub_level {
      false
    } else {
      self.subsumes(rhs)
    }
  }
}
