use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_type_pack::get_type_pack_id,
  records::{
    tarjan::Tarjan, type_pack::TypePack, type_pack_var::TypePackVar,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl Tarjan {
  pub fn visit_children_type_pack_id_i32(&mut self, tp: TypePackId, _index: i32) {
    let mut tp = tp;
    unsafe {
      LUAU_ASSERT!(tp == (*self.log).follow_type_pack_id(tp));
    }

    if self.ignore_children_visit_type_pack_id(tp) {
      return;
    }

    let ptp = unsafe { (*self.log).pending_type_pack_id(tp) };
    if !ptp.is_null() {
      tp = unsafe { &(*ptp).pending as *const TypePackVar };
    }

    if let Some(tpp) = get_type_pack_id::<TypePack>(tp) {
      for tv in tpp.head.iter() {
        self.visit_child_type_id(*tv);
      }
      if let Some(tail) = tpp.tail {
        self.visit_child_type_pack_id(tail);
      }
      return;
    }

    if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(tp) {
      self.visit_child_type_id(vtp.ty);
    }
  }
}
