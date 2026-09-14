use crate::{
  functions::get_type_pack::get_type_pack_id,
  records::{apply_type_function::ApplyTypeFunction, generic_type_pack::GenericTypePack},
  type_aliases::type_pack_id::TypePackId,
};

impl ApplyTypeFunction {
  pub fn ignore_children_type_pack_id(&mut self, tp: TypePackId) -> bool {
    let gt = get_type_pack_id::<GenericTypePack>(tp);
    !gt.is_none()
  }
}
