use crate::{
  functions::{get_type, get_type_pack},
  records::{
    apply_type_function::ApplyTypeFunction, extern_type::ExternType, generic_type::GenericType,
    generic_type_pack::GenericTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ApplyTypeFunction {
  pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
    if get_type::get::<GenericType>(ty).is_some() {
      true
    } else {
      get_type::get::<ExternType>(ty).is_some()
    }
  }

  pub fn ignore_children_type_pack_id(&mut self, tp: TypePackId) -> bool {
    let gt = get_type_pack::get::<GenericTypePack>(tp);
    gt.is_some()
  }
}
