//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryUnifyVariadics, L2432-2493)
use alloc::string::String;
use core::ptr::null_mut;

use crate::{
  functions::{
    begin_type_pack_alt_d::begin, end_type_pack::end, follow_type::follow as follow_type,
    follow_type_pack::follow as follow_pack, get_type_alt_j::get_type_id,
    get_type_pack::get_type_pack_id, is_blocked_unifier_alt_c::is_blocked_txn_log_type_pack_id,
  },
  records::{
    any_type::AnyType, free_type_pack::FreeTypePack, generic_error::GenericError,
    generic_type_pack::GenericTypePack, type_pack::TypePack, type_pack_var::TypePackVar,
    unifier::Unifier, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    error_type_pack::ErrorTypePack, type_error_data::TypeErrorData, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant,
  },
};
impl Unifier {
  /// `void Unifier::tryUnifyVariadics(TypePackId subTp, TypePackId superTp, bool reversed, int subOffset)`
  pub fn unifier_try_unify_variadics(
    &mut self,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    reversed: bool,
    sub_offset: i32,
  ) {
    let super_variadic = self
      .log
      .txn_log_get_mutable::<VariadicTypePack, TypePackId>(super_tp);

    // Null/ICE check BEFORE the deref (C++ checks first; the port had the
    // `follow_type((*super_variadic).ty)` read ahead of this guard).
    if super_variadic.is_null() {
      self.ice_string("passed non-variadic pack to tryUnifyVariadics");
    }
    let variadic_ty = unsafe { follow_type((*super_variadic).ty) };

    let sub_variadic = self.log.txn_log_get::<VariadicTypePack, TypePackId>(sub_tp);
    if !sub_variadic.is_null() {
      let (a, b) = if reversed {
        (variadic_ty, unsafe { (*sub_variadic).ty })
      } else {
        (unsafe { (*sub_variadic).ty }, variadic_ty)
      };
      self.try_unify_type_id_type_id_bool_bool_literal_properties(a, b, false, false, None);
    } else if !self
      .log
      .txn_log_get::<TypePack, TypePackId>(sub_tp)
      .is_null()
    {
      let mut sub_iter = begin(sub_tp, &self.log as *const _);
      let sub_end = end(sub_tp);

      for _ in 0..sub_offset {
        sub_iter.operator_inc();
      }

      while sub_iter.operator_ne(&sub_end) {
        let cur = *sub_iter.operator_deref();
        let (a, b) = if reversed {
          (variadic_ty, cur)
        } else {
          (cur, variadic_ty)
        };
        self.try_unify_type_id_type_id_bool_bool_literal_properties(a, b, false, false, None);
        sub_iter.operator_inc();
      }

      if let Some(maybe_tail) = sub_iter.tail() {
        let tail = unsafe { follow_pack(maybe_tail) };

        if is_blocked_txn_log_type_pack_id(&self.log, tail) {
          self.blocked_type_packs.push(tail);
        } else if !get_type_pack_id::<FreeTypePack>(tail).is_none() {
          // log.replace(tail, BoundTypePack(superTp));
          let bound = TypePackVar {
            ty: TypePackVariant::Bound(super_tp),
            persistent: false,
            owning_arena: null_mut(),
          };
          self.log.replace_type_pack_id_type_pack_var(tail, bound);
        } else if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(tail) {
          self.try_unify_type_id_type_id_bool_bool_literal_properties(
            vtp.ty,
            variadic_ty,
            false,
            false,
            None,
          );
        } else if !get_type_pack_id::<GenericTypePack>(tail).is_none() {
          self.report_error_location_type_error_data(
            self.location,
            TypeErrorData::GenericError(GenericError::new(String::from(
              "Cannot unify variadic and generic packs",
            ))),
          );
        } else if !get_type_pack_id::<ErrorTypePack>(tail).is_none() {
          // Nothing to do here.
        } else {
          self.ice_string("Unknown TypePack kind");
        }
      }
    } else if !get_type_id::<AnyType>(variadic_ty).is_none()
      && !self
        .log
        .txn_log_get::<GenericTypePack, TypePackId>(sub_tp)
        .is_null()
    {
      // Nothing to do.  This is ok.
    } else {
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::GenericError(GenericError::new(String::from(
          "Failed to unify variadic packs",
        ))),
      );
    }
  }
}
