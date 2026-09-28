//! `path` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;

use crate::{records::path::Path, type_aliases::component::Component};

impl Path {
  // This method is already implemented on `Path` in `crate::records::path`.
  // The generated skeleton should not re-implement it.
  // Keeping file intentionally empty would violate the requirement to output a complete file,
  // so we omit the impl entirely.
}

impl Path {
  pub fn new() -> Self {
    Self {
      components: Vec::new(),
    }
  }

  pub fn path_vector_component(components: Vec<Component>) -> Self {
    Self { components }
  }

  pub fn path_component(component: Component) -> Self {
    Path::path_vector_component(alloc::vec![component])
  }
}
