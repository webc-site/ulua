use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_type_pack::get_type_pack_id,
  records::{
    blocked_type_pack::BlockedTypePack, free_type_pack::FreeTypePack,
    generic_type_pack::GenericTypePack, iterative_type_visitor::IterativeTypeVisitor,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack, type_pack::TypePack,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    bound_type_pack::BoundTypePack, error_type_pack::ErrorTypePack, type_pack_id::TypePackId,
  },
};
impl IterativeTypeVisitor {
  pub fn process_type_pack_id(&mut self, tp: TypePackId) {
    if self.iterative_type_visitor_has_seen(tp as *const c_void) {
      return;
    }

    if let Some(btv) = get_type_pack_id::<BoundTypePack>(tp) {
      if self.visit_type_pack_id_bound_type_pack(tp, btv) {
        self.traverse_type_pack_id(btv.bound_to);
      }
    } else if let Some(ftv) = get_type_pack_id::<FreeTypePack>(tp) {
      self.visit_type_pack_id_free_type_pack(tp, ftv);
    } else if let Some(gtv) = get_type_pack_id::<GenericTypePack>(tp) {
      self.visit_type_pack_id_generic_type_pack(tp, gtv);
    } else if let Some(etv) = get_type_pack_id::<ErrorTypePack>(tp) {
      self.visit_type_pack_id_error_type_pack(tp, etv);
    } else if let Some(pack) = get_type_pack_id::<TypePack>(tp) {
      if self.visit_type_pack_id_type_pack(tp, pack) {
        // 克隆后再遍历：遍历过程中 visit 可能改写该 pack 的 head
        let head = pack.head.clone();
        for ty in head {
          self.traverse_type_id(ty);
        }

        if let Some(tail) = pack.tail {
          self.traverse_type_pack_id(tail);
        }
      }
    } else if let Some(pack) = get_type_pack_id::<VariadicTypePack>(tp) {
      if self.visit_type_pack_id_variadic_type_pack(tp, pack) {
        self.traverse_type_id(pack.ty);
      }
    } else if let Some(btp) = get_type_pack_id::<BlockedTypePack>(tp) {
      self.visit_type_pack_id_blocked_type_pack(tp, btp);
    } else if let Some(tfitp) = get_type_pack_id::<TypeFunctionInstanceTypePack>(tp) {
      if self.visit_type_pack_id_type_function_instance_type_pack(tp, tfitp) {
        let type_arguments = tfitp.type_arguments.clone();
        for t in type_arguments {
          self.traverse_type_id(t);
        }

        let pack_arguments = tfitp.pack_arguments.clone();
        for t in pack_arguments {
          self.traverse_type_pack_id(t);
        }
      }
    } else {
      LUAU_ASSERT!(
        false /* "GenericTypeVisitor::traverse(TypePackId) is not exhaustive!" */
      );
    }

    self.iterative_type_visitor_unsee(tp as *const c_void);
  }
}
