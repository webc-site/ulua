use ulua_common::fint;

use crate::records::type_cloner::TypeCloner;

impl TypeCloner {
  pub fn has_exceeded_iteration_limit(&self) -> bool {
    if fint::LuauTypeCloneIterationLimit.get() == 0 {
      return false;
    }

    self.steps as usize + self.queue.len() >= fint::LuauTypeCloneIterationLimit.get() as usize
  }
}
