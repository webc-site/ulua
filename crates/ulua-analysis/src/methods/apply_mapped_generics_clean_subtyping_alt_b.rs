use ulua_common::{functions::get_if_variant::get_if, macros::luau_assert::LUAU_ASSERT};

use crate::{
  records::apply_mapped_generics::ApplyMappedGenerics, type_aliases::type_pack_id::TypePackId,
};

impl ApplyMappedGenerics {
  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    let env = unsafe { &*self.env };
    let result = env.lookup_generic_pack(tp);
    if let Some(mapped_gen) = get_if::<TypePackId, _>(&result) {
      return *mapped_gen;
    }
    LUAU_ASSERT!(false);
    unsafe { (*self.builtin_types).any_type_pack }
  }
}
