use crate::{
  records::scope::Scope,
  type_aliases::{name_type::Name, type_pack_id::TypePackId},
};

impl Scope {
  pub fn lookup_pack(&self, name: &Name) -> Option<TypePackId> {
    let mut scope: &Scope = self;
    loop {
      if let Some(type_pack_id) = scope.private_type_pack_bindings.get(name) {
        return Some(*type_pack_id);
      }

      {
        let parent = scope.parent.as_ref()?;
        scope = parent.as_ref();
      }
    }
  }
}
