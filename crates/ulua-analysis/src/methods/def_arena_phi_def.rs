use crate::{records::def_arena::DefArena, type_aliases::def_id_def::DefId};

impl DefArena {
  pub fn phi_def_id_def_id(&mut self, a: DefId, b: DefId) -> DefId {
    self.phi_vector_def_id(&[a, b])
  }
}
