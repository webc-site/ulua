use crate::records::type_pack_var::TypePackVar;

impl TypePackVar {
  pub fn reassign(&mut self, rhs: &TypePackVar) {
    self.ty = rhs.ty.clone();
  }
}
