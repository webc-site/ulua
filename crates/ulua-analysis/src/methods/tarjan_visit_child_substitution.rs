use core::ptr::null_mut;

use crate::{
  records::tarjan::Tarjan,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Tarjan {
  pub fn visit_child_optional_ty<Ty>(&mut self, ty: Option<Ty>)
  where
    Ty: Into<TypeId>,
  {
    if let Some(inner_ty) = ty {
      self.visit_child_type_id(inner_ty.into());
    }
  }

  pub(crate) fn visit_child_type_id(&mut self, ty: TypeId) {
    let ty = unsafe { (*self.log).follow_type_id(ty) };

    self.edges_ty.push(ty);
    self.edges_tp.push(null_mut());
  }

  pub(crate) fn visit_child_type_pack_id(&mut self, tp: TypePackId) {
    let tp = unsafe { (*self.log).follow_type_pack_id(tp) };

    self.edges_ty.push(null_mut());
    self.edges_tp.push(tp);
  }
}
