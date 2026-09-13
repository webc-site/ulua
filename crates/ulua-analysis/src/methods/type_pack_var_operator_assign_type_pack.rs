use crate::{
  records::type_pack_var::TypePackVar, type_aliases::type_pack_variant::TypePackVariant,
};

impl TypePackVar {
  pub fn operator_assign_type_pack_variant(&mut self, tp: TypePackVariant) -> &mut Self {
    self.ty = tp;
    self
  }
}
