//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryUnifyVariadics, L2432-2493)
use alloc::string::String;

use crate::{
  functions::{
    begin_type_pack::begin_type_pack_id_txn_log as begin, end_type_pack::end,
    follow_type::follow as follow_type, follow_type_pack::follow as follow_pack, get_type,
    get_type_pack, is_blocked_unifier::is_blocked_txn_log_type_pack_id,
  },
  records::{
    any_type::AnyType, arena_id::ArenaId, free_type_pack::FreeTypePack,
    generic_error::GenericError, generic_type_pack::GenericTypePack, type_pack::TypePack,
    type_pack_var::TypePackVar, unifier::Unifier, variadic_type_pack::VariadicTypePack,
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
    // C++ `tryUnifyVariadics`（Unifier.cpp）先做 `getMutable<VariadicTypePack>`
    // 判空、ICE 后提前 `return false`；Rust 侧取 `Option` 形态，ICE 臂直接返回，
    // 消除原「判空后仍解引用」的 UB 路径。
    let Some(super_variadic) = self
      .log
      .txn_log_get::<VariadicTypePack, TypePackId>(super_tp)
    else {
      self.ice_string("passed non-variadic pack to tryUnifyVariadics");
      return;
    };
    let variadic_ty = follow_type(super_variadic.ty);

    if let Some(sub_variadic) = self.log.txn_log_get::<VariadicTypePack, TypePackId>(sub_tp) {
      let (a, b) = if reversed {
        (variadic_ty, sub_variadic.ty)
      } else {
        (sub_variadic.ty, variadic_ty)
      };
      self.try_unify_type_id_type_id_bool_bool_literal_properties(a, b, false, false, None);
    } else if self
      .log
      .txn_log_get::<TypePack, TypePackId>(sub_tp)
      .is_some()
    {
      let mut sub_iter = begin(sub_tp, &self.log as *const _);
      let sub_end = end(sub_tp);

      // sub_offset 是被前置消费的头部元素数（语义量）；sub_iter 是 TxnLog 上的
      // 类型域游标，逐个 advance 前推。
      for _ in 0..sub_offset {
        sub_iter.advance();
      }

      while sub_iter != sub_end {
        let cur = *sub_iter.current();
        let (a, b) = if reversed {
          (variadic_ty, cur)
        } else {
          (cur, variadic_ty)
        };
        self.try_unify_type_id_type_id_bool_bool_literal_properties(a, b, false, false, None);
        sub_iter.advance();
      }

      if let Some(maybe_tail) = sub_iter.tail() {
        let tail = follow_pack(maybe_tail);

        if is_blocked_txn_log_type_pack_id(&self.log, tail) {
          self.blocked_type_packs.push(tail);
        } else if get_type_pack::get::<FreeTypePack>(tail).is_some() {
          // log.replace(tail, BoundTypePack(superTp));
          let bound = TypePackVar {
            ty: TypePackVariant::Bound(super_tp),
            persistent: false,
            owning_arena: ArenaId::NONE,
          };
          self.log.replace_type_pack_id_type_pack_var(tail, bound);
        } else if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(tail) {
          self.try_unify_type_id_type_id_bool_bool_literal_properties(
            vtp.ty,
            variadic_ty,
            false,
            false,
            None,
          );
        } else if get_type_pack::get::<GenericTypePack>(tail).is_some() {
          self.report_error_location_type_error_data(
            self.location,
            TypeErrorData::GenericError(GenericError::new(String::from(
              "Cannot unify variadic and generic packs",
            ))),
          );
        } else if get_type_pack::get::<ErrorTypePack>(tail).is_none() {
          self.ice_string("Unknown TypePack kind");
        }
      }
    } else if get_type::get::<AnyType>(variadic_ty).is_none()
      || self
        .log
        .txn_log_get::<GenericTypePack, TypePackId>(sub_tp)
        .is_none()
    {
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::GenericError(GenericError::new(String::from(
          "Failed to unify variadic packs",
        ))),
      );
    }
  }
}
