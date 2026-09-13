use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    are_equivalent::are_equivalent, follow_type::follow_type_id,
    follow_type_pack::follow_type_pack_id, get_type_alt_j::get_type_id,
    get_type_pack::get_type_pack_id,
  },
  records::{
    generic_type::GenericType, generic_type_pack::GenericTypePack,
    internal_type_function_finder::InternalTypeFunctionFinder,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};
impl InternalTypeFunctionFinder {
  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    let mut has_generic = false;

    for p in &tfit.type_arguments {
      if get_type_id::<GenericType>(follow_type_id(*p)).is_some() {
        has_generic = true;
        break;
      }
    }

    if !has_generic {
      for p in &tfit.pack_arguments {
        if get_type_pack_id::<GenericTypePack>(unsafe { follow_type_pack_id(*p) }).is_some() {
          has_generic = true;
          break;
        }
      }
    }

    if has_generic {
      for mentioned in self.mentioned_functions.iter() {
        let mentioned_tfit = get_type_id::<TypeFunctionInstanceType>(*mentioned);
        LUAU_ASSERT!(mentioned_tfit.is_some());
        // C++ `LUAU_ASSERT(mentionedTf)` 必命中，此处 unwrap 安全。
        if are_equivalent(tfit, mentioned_tfit.unwrap()) {
          return true;
        }
      }
      self.internal_functions.insert(ty);
    }

    true
  }
}
