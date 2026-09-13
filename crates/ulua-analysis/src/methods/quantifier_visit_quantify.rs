use crate::{
  functions::as_mutable_type_id::as_mutable_type_id,
  records::{free_type::FreeType, generic_type::GenericType, quantifier::Quantifier, r#type::Type},
  type_aliases::type_id::TypeId,
};

impl Quantifier {
  pub fn visit_type_id_free_type(&mut self, ty: TypeId, ftv: &FreeType) -> bool {
    self.seen_mutable_type = true;

    if !self.level.subsumes(&ftv.level) {
      return false;
    }

    unsafe {
      *as_mutable_type_id(ty) = Type::from(GenericType::generic_type_type_level(self.level));
    }

    self.generics.push(ty);

    false
  }
}
