use crate::{records::tarjan::Tarjan, type_aliases::type_id::TypeId};

impl Tarjan {
  pub fn visit_child_optional_ty<Ty>(&mut self, ty: Option<Ty>)
  where
    Ty: Into<TypeId>,
  {
    if let Some(inner_ty) = ty {
      self.visit_child_type_id(inner_ty.into());
    }
  }
}
