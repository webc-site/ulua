use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{get_mutable_type, get_type, get_type_pack},
  records::{
    arena_id::ArenaId, free_type::FreeType, free_type_pack::FreeTypePack,
    intersection_type::IntersectionType, occurs_check_failed::OccursCheckFailed, r#type::Type,
    type_pack::TypePack, type_pack_var::TypePackVar, unifier::Unifier, union_type::UnionType,
  },
  type_aliases::{
    error_type::ErrorType, error_type_pack::ErrorTypePack, type_error_data::TypeErrorData,
    type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
    type_variant::TypeVariant,
  },
};

impl Unifier {
  /// `bool Unifier::occursCheck(TypeId needle, TypeId haystack, bool reversed)`
  pub(crate) fn occurs_check_type_id_type_id_bool(
    &mut self,
    needle: TypeId,
    haystack: TypeId,
    reversed: bool,
  ) -> bool {
    // `self.shared_state` 是 `Handle` 单例句柄（契约见 records/arena_handle.rs），
    // 指向 TypeCheckResult 持有的共享状态、比本 Unifier 长寿；单线程串行下
    // 本可变借用只触及 temp_seen_ty，随后把 seen 传入递归 occursCheck 的调用
    // 返回即结束，期间无任何其它路径可变触碰该共享状态。
    let shared_state = self.shared_state.get_mut();
    shared_state.temp_seen_ty.clear();

    let occurs = self.occurs_check_dense_hash_set_type_id_type_id_type_id(
      &mut shared_state.temp_seen_ty,
      needle,
      haystack,
    );

    if occurs {
      let mut inner_state = self.unifier_make_child_unifier();
      if let Some(ut) = get_type::get::<UnionType>(haystack) {
        if reversed {
          // `ut` 来自 get_type_id 对 arena 驻留 haystack 节点的命中（Some 即证
          // 存活共享借用），被调方签名已 Rust 化为 `&'static UnionType`，无 unsafe。
          inner_state.unifier_try_unify_union_with_type(haystack, ut, needle);
        } else {
          // `ut` 为 get_type_id 命中的 arena 驻留 UnionType 共享借用，被调方
          // 签名已 Rust 化为 `&'static UnionType`，无 unsafe。
          inner_state.unifier_try_unify_type_with_union(needle, haystack, ut, false, false);
        }
      } else if let Some(it) = get_type::get::<IntersectionType>(haystack) {
        if reversed {
          // `it` 为 get_type_id 命中的 arena 驻留 IntersectionType 共享借用，
          // 被调方签名已 Rust 化为 `&'static IntersectionType`，无 unsafe。
          inner_state.unifier_try_unify_intersection_with_type(haystack, it, needle, false, false);
        } else {
          // 同上——被调方对应 tryUnifyTypeWithIntersection 的只读入参。
          inner_state.unifier_try_unify_type_with_intersection(needle, haystack, it);
        }
      } else {
        inner_state.failure = true;
      }

      if inner_state.failure {
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::OccursCheckFailed(OccursCheckFailed::default()),
        );
        // C++: log.replace(needle, BoundType{builtinTypes->error_type});
        let error_ty = self.builtin_types_ref().error_type;
        self
          .log
          .replace_type_id_t(needle, Type::new(TypeVariant::Bound(error_ty)));
      }
    }

    occurs
  }

  /// `bool Unifier::occursCheck(DenseHashSet<TypeId>& seen, TypeId needle, TypeId haystack)`
  pub(crate) fn occurs_check_dense_hash_set_type_id_type_id_type_id(
    &mut self,
    seen: &mut DenseHashSet<TypeId>,
    mut needle: TypeId,
    mut haystack: TypeId,
  ) -> bool {
    let mut occurrence = false;

    needle = self.log.follow_type_id(needle);
    haystack = self.log.follow_type_id(haystack);

    if seen.find(&haystack).is_some() {
      return false;
    }

    seen.insert(haystack);

    if get_mutable_type::get_mutable::<ErrorType>(needle).is_some() {
      return false;
    }

    if get_mutable_type::get_mutable::<FreeType>(needle).is_none() {
      self.ice_string("Expected needle to be free");
    }

    if needle == haystack {
      return true;
    }

    if get_mutable_type::get_mutable::<FreeType>(haystack).is_some() {
      return false;
    } else if let Some(a) = get_mutable_type::get_mutable::<UnionType>(haystack) {
      let options = a.options.clone();
      for ty in options {
        if self.occurs_check_dense_hash_set_type_id_type_id_type_id(seen, needle, ty) {
          occurrence = true;
        }
      }
    } else if let Some(a) = get_mutable_type::get_mutable::<IntersectionType>(haystack) {
      let parts = a.parts.clone();
      for ty in parts {
        if self.occurs_check_dense_hash_set_type_id_type_id_type_id(seen, needle, ty) {
          occurrence = true;
        }
      }
    }

    occurrence
  }

  pub fn occurs_check_type_pack_id_type_pack_id_bool(
    &mut self,
    needle: TypePackId,
    haystack: TypePackId,
    _reversed: bool,
  ) -> bool {
    // 同类型版 occursCheck：共享状态句柄的可变借用仅触及 temp_seen_tp，
    // 随其后递归调用返回即结束，单线程串行无并存的其它可变触碰。
    let shared_state = self.shared_state.get_mut();
    shared_state.temp_seen_tp.clear();

    let occurs = self.occurs_check_dense_hash_set_type_pack_id_type_pack_id_type_pack_id(
      &mut shared_state.temp_seen_tp,
      needle,
      haystack,
    );

    if occurs {
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::OccursCheckFailed(OccursCheckFailed::default()),
      );
      // C++: log.replace(needle, BoundTypePack{builtinTypes->error_type_pack});
      // The Bound variant stores the bound-to pack id directly.
      let error_tp = self.builtin_types_ref().error_type_pack;
      let bound = TypePackVar {
        ty: TypePackVariant::Bound(error_tp),
        persistent: false,
        owning_arena: ArenaId::NONE,
      };
      self.log.replace_type_pack_id_type_pack_var(needle, bound);
    }

    occurs
  }

  pub(crate) fn occurs_check_dense_hash_set_type_pack_id_type_pack_id_type_pack_id(
    &mut self,
    seen: &mut DenseHashSet<TypePackId>,
    mut needle: TypePackId,
    mut haystack: TypePackId,
  ) -> bool {
    needle = self.log.follow_type_pack_id(needle);
    haystack = self.log.follow_type_pack_id(haystack);

    if seen.find(&haystack).is_some() {
      return false;
    }

    seen.insert(haystack);

    if get_type_pack::get::<ErrorTypePack>(needle).is_some() {
      return false;
    }

    if get_type_pack::get::<FreeTypePack>(needle).is_none() {
      self.ice_string("Expected needle pack to be free");
    }

    while get_type_pack::get::<ErrorTypePack>(haystack).is_none() {
      if needle == haystack {
        return true;
      }

      if let Some(pack) = get_type_pack::get::<TypePack>(haystack)
        && let Some(tail) = pack.tail
      {
        haystack = self.log.follow_type_pack_id(tail);
        continue;
      }

      break;
    }

    false
  }
}
