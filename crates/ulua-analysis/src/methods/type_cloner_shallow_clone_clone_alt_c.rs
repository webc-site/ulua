use crate::{
  records::{property_type::Property, type_cloner::TypeCloner},
  type_aliases::type_id::TypeId,
};

impl TypeCloner {
  pub fn shallow_clone_property(&mut self, p: &Property) -> Property {
    let mut clone_read_ty: Option<TypeId> = None;
    if let Some(ty) = p.read_ty {
      clone_read_ty = Some(self.shallow_clone_type_id(ty));
    }

    let mut clone_write_ty: Option<TypeId> = None;
    if let Some(ty) = p.write_ty {
      clone_write_ty = Some(self.shallow_clone_type_id(ty));
    }

    let mut cloned = Property::create(clone_read_ty, clone_write_ty);
    cloned.deprecated = p.deprecated;
    cloned.deprecated_suggestion = p.deprecated_suggestion.clone();
    cloned.location = p.location;
    cloned.tags = p.tags.clone();
    cloned.documentation_symbol = p.documentation_symbol.clone();
    cloned.type_location = p.type_location;
    cloned
  }
}
