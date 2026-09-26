use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    are_equivalent::are_equivalent, follow_type, follow_type_pack, get_type, get_type_pack,
  },
  records::{
    generic_type::GenericType, generic_type_pack::GenericTypePack,
    internal_type_function_finder::InternalTypeFunctionFinder,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl InternalTypeFunctionFinder {
  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    let mut has_generic = false;

    for p in &tfit.type_arguments {
      if get_type::get::<GenericType>(follow_type::follow(*p)).is_some() {
        has_generic = true;
        break;
      }
    }

    if !has_generic {
      for p in &tfit.pack_arguments {
        if get_type_pack::get::<GenericTypePack>(follow_type_pack::follow(*p)).is_some() {
          has_generic = true;
          break;
        }
      }
    }

    if has_generic {
      for mentioned in self.mentioned_functions.iter() {
        let mentioned_tfit = get_type::get::<TypeFunctionInstanceType>(*mentioned);
        LUAU_ASSERT!(mentioned_tfit.is_some());
        if are_equivalent(
          tfit,
          mentioned_tfit.expect("C++ `LUAU_ASSERT(mentionedTf)` 紧邻断言蕴含必命中"),
        ) {
          return true;
        }
      }
      self.internal_functions.insert(ty);
    }

    true
  }

  pub fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    tp: TypePackId,
    tfitp: &TypeFunctionInstanceTypePack,
  ) -> bool {
    let mut has_generic = false;

    for p in &tfitp.type_arguments {
      if get_type::get::<GenericType>(follow_type::follow(*p)).is_some() {
        has_generic = true;
        break;
      }
    }

    if !has_generic {
      for p in &tfitp.pack_arguments {
        if get_type_pack::get::<GenericTypePack>(follow_type_pack::follow(*p)).is_some() {
          has_generic = true;
          break;
        }
      }
    }

    if has_generic {
      for mentioned in self.mentioned_function_packs.iter() {
        let mentioned_tfitp = get_type_pack::get::<TypeFunctionInstanceTypePack>(*mentioned);
        LUAU_ASSERT!(mentioned_tfitp.is_some());
        if are_equivalent(
          tfitp,
          mentioned_tfitp.expect("C++ `LUAU_ASSERT(mentionedTf)` 紧邻断言蕴含必命中"),
        ) {
          return true;
        }
      }
      self.internal_pack_functions.insert(tp);
    }

    true
  }
}
