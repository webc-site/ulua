//! @interface-stub
use alloc::vec::Vec;
use core::ptr::null_mut;

use crate::{
  records::{
    constraint_generator::ConstraintGenerator, type_pack::TypePack, type_pack_var::TypePackVar,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariant},
};
impl ConstraintGenerator {
  pub fn add_type_pack(&mut self, head: Vec<TypeId>, tail: Option<TypePackId>) -> TypePackId {
    if head.is_empty() {
      if let Some(tail) = tail {
        tail
      } else {
        unsafe { (*self.builtin_types).empty_type_pack }
      }
    } else {
      let pack = TypePack { head, tail };
      let pack_var = TypePackVar {
        ty: TypePackVariant::TypePack(pack),
        persistent: false,
        owning_arena: null_mut(),
      };

      unsafe { (*self.arena).add_type_pack_t(pack_var) }
    }
  }
}
