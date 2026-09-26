//! `Demoter::demote`（TypeInfer.cpp:818-824）。

use crate::{records::demoter::Demoter, type_aliases::type_id::TypeId};

impl Demoter {
  pub fn demote(&mut self, expected_types: &mut [Option<TypeId>]) {
    self.install_substitution_vtable();
    for slot in expected_types.iter_mut().flatten() {
      // C++ `ty = substitute(*ty)`；`None` 仅在递归限位等异常路径出现，
      // 此时保留原类型（对应 C++ 断言路径外的保守处理）。
      if let Some(demoted) = self.base.substitute_type_id(*slot) {
        *slot = demoted;
      }
    }
  }
}
