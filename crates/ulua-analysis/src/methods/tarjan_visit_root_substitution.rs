use ulua_common::fint;

use crate::{
  enums::tarjan_result::TarjanResult,
  records::{tarjan::Tarjan, tarjan_worklist_vertex::TarjanWorklistVertex},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Tarjan {
  pub(crate) fn visit_root_type_id(&mut self, ty: TypeId) -> TarjanResult {
    self.child_count = 0;
    if self.child_limit == 0 {
      self.child_limit = fint::LuauTarjanChildLimit.get();
    }

    let ty = unsafe { (*self.log).follow_type_id(ty) };

    let (index, _fresh) = self.indexify_type_id(ty);
    self.worklist.push(TarjanWorklistVertex {
      index,
      curr_edge: -1,
      last_edge: -1,
    });

    self.loop_item()
  }

  pub(crate) fn visit_root_type_pack_id(&mut self, tp: TypePackId) -> TarjanResult {
    self.child_count = 0;
    if self.child_limit == 0 {
      self.child_limit = fint::LuauTarjanChildLimit.get();
    }

    let tp = unsafe { (*self.log).follow_type_pack_id(tp) };

    let (index, _fresh) = self.indexify_type_pack_id(tp);
    self.worklist.push(TarjanWorklistVertex {
      index,
      curr_edge: -1,
      last_edge: -1,
    });

    self.loop_item()
  }
}
