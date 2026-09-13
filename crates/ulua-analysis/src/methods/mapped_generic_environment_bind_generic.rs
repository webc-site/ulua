use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::variant::Variant3};

use crate::{
  functions::get_type_pack::get_type_pack_id,
  records::{
    generic_type_pack::GenericTypePack, mapped_generic_environment::MappedGenericEnvironment,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl MappedGenericEnvironment {
  pub fn bind_generic(&mut self, generic_tp: TypePackId, bindee_tp: TypePackId) -> bool {
    // We shouldn't bind generic type packs to themselves
    if generic_tp == bindee_tp {
      return true;
    }

    if get_type_pack_id::<GenericTypePack>(generic_tp).is_none() {
      LUAU_ASSERT!(false);
      return false;
    }

    let lookup_result = self.lookup_generic_pack(generic_tp);
    if let Variant3::V1(unmapped) = lookup_result {
      *self.frames[unmapped.scope_index]
        .mappings
        .get_or_insert(generic_tp) = Some(bindee_tp);
      true
    } else {
      false
    }
  }
}
