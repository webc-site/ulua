//! Faithful port of `TypeChecker2::explainReasonings_<TID>` (TypeChecker2.cpp:3026-3134).
//!
//! The C++ method is templated over `TID` (either `TypeId` or `TypePackId`).
//! In Rust this is realised as a method generic over a small `ExplainRoot`
//! trait that supplies the only TID-specific operation: rooting a `traverse`
//! against a `TypePath`. The two public `explainReasonings` overloads forward
//! to this generic method.
use alloc::string::String;
/// Roots a `TypePath` traversal for the concrete `TID` (TypeId / TypePackId).
use alloc::vec::Vec;

use ulua_ast::records::location::Location;
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::subtyping_variance::SubtypingVariance,
  functions::{
    to_string_human::to_string_human, to_string_to_string_alt_h::to_string_type_or_pack,
    traverse_type_path_alt_b::traverse, traverse_type_path_alt_c,
  },
  records::{
    builtin_types::BuiltinTypes, internal_error::InternalError, path::Path, reasonings::Reasonings,
    subtyping_result::SubtypingResult, type_arena::TypeArena, type_checker_2::TypeChecker2,
  },
  type_aliases::{
    component::Component,
    type_id::TypeId,
    type_or_pack::{TypeOrPack, TypeOrPackMember},
    type_pack_id::TypePackId,
  },
};
pub trait ExplainRoot: Copy {
  fn explain_traverse(
    self,
    path: &Path,
    builtin_types: &BuiltinTypes,
    arena: &mut TypeArena,
  ) -> Option<TypeOrPack>;
}

impl ExplainRoot for TypeId {
  fn explain_traverse(
    self,
    path: &Path,
    builtin_types: &BuiltinTypes,
    arena: &mut TypeArena,
  ) -> Option<TypeOrPack> {
    traverse_type_path_alt_c::traverse(self, path, builtin_types, arena)
  }
}

impl ExplainRoot for TypePackId {
  fn explain_traverse(
    self,
    path: &Path,
    builtin_types: &BuiltinTypes,
    arena: &mut TypeArena,
  ) -> Option<TypeOrPack> {
    traverse(self, path, builtin_types, arena)
  }
}

impl TypeChecker2 {
  pub fn explain_reasonings_generic<TID: ExplainRoot>(
    &mut self,
    sub_ty: TID,
    super_ty: TID,
    location: Location,
    r: &SubtypingResult,
  ) -> Reasonings {
    if r.reasoning.empty() {
      return Reasonings::default();
    }

    let mut reasons: Vec<String> = Vec::new();
    let mut suppressed = true;
    for reasoning in r.reasoning.iter() {
      if reasoning.sub_path.path_empty() && reasoning.super_path.path_empty() {
        continue;
      }

      let opt_sub_leaf = unsafe {
        sub_ty.explain_traverse(
          &reasoning.sub_path,
          &*self.builtin_types,
          &mut *(*self.subtyping).arena,
        )
      };
      let opt_super_leaf = unsafe {
        super_ty.explain_traverse(
          &reasoning.super_path,
          &*self.builtin_types,
          &mut *(*self.subtyping).arena,
        )
      };

      let (sub_leaf, super_leaf) = match (opt_sub_leaf, opt_super_leaf) {
        (Some(s), Some(sup)) => (s, sup),
        _ => {
          self.report_error_type_error_data_location(
            InternalError::new(String::from(
              "Subtyping test returned a reasoning with an invalid path",
            ))
            .into(),
            &location,
          );
          return Reasonings::default();
        }
      };

      let sub_leaf_ty = TypeId::get_if(&sub_leaf).copied();
      let super_leaf_ty = TypeId::get_if(&super_leaf).copied();
      let sub_leaf_tp = TypePackId::get_if(&sub_leaf).copied();
      let super_leaf_tp = TypePackId::get_if(&super_leaf).copied();

      if sub_leaf_ty.is_none()
        && super_leaf_ty.is_none()
        && sub_leaf_tp.is_none()
        && super_leaf_tp.is_none()
      {
        self.report_error_type_error_data_location(
                    InternalError::new(String::from(
                        "Subtyping test returned a reasoning where one path ends at a type and the other ends at a pack.",
                    ))
                    .into(),
                    &location,
                );
        return Reasonings::default();
      }

      let relation = match reasoning.variance {
        SubtypingVariance::Invariant => "exactly",
        SubtypingVariance::Contravariant => "a supertype of",
        _ => "a subtype of",
      };

      let mut sub_leaf_as_string = to_string_type_or_pack(&sub_leaf);
      // if the string is empty, it must be an empty type pack
      if sub_leaf_as_string.is_empty() {
        sub_leaf_as_string = String::from("()");
      }

      let mut super_leaf_as_string = to_string_type_or_pack(&super_leaf);
      if super_leaf_as_string.is_empty() {
        super_leaf_as_string = String::from("()");
      }

      let base_reason = alloc::format!(
        "`{}` is not {} `{}`",
        sub_leaf_as_string,
        relation,
        super_leaf_as_string
      );

      let reason: String;

      if FFlag::LuauPropertyModifierMismatchErrors.get() && reasoning.is_property_modifier_violation
      {
        // The leaf types at the end of the paths are the same type, so a
        // plain "X is not a subtype of X" message would be misleading.
        let mut prop_name = String::from("a property");
        let mut is_read_only = true;
        let last = reasoning.sub_path.last();
        LUAU_ASSERT!(last.is_some() && matches!(last.as_ref().unwrap(), Component::Property(_)));
        if let Some(Component::Property(prop)) = last {
          prop_name = alloc::format!("`{}`", prop.name());
          is_read_only = prop.is_read();
        }

        if is_read_only {
          reason = alloc::format!(
            "{} is a read-only property in the latter type, but the former type requires a read-write property",
            prop_name
          );
        } else {
          reason = alloc::format!(
            "{} is a write-only property in the latter type, but the former type requires a read-write property",
            prop_name
          );
        }
      } else if reasoning.sub_path.operator_eq(&reasoning.super_path) {
        reason = alloc::format!(
          "{}`{}` in the latter type and `{}` in the former type, and {}",
          to_string_human(&reasoning.sub_path),
          sub_leaf_as_string,
          super_leaf_as_string,
          base_reason
        );
      } else if !reasoning.sub_path.path_empty() && !reasoning.super_path.path_empty() {
        reason = alloc::format!(
          "{}`{}` and {}`{}`, and {}",
          to_string_human(&reasoning.sub_path),
          sub_leaf_as_string,
          to_string_human(&reasoning.super_path),
          super_leaf_as_string,
          base_reason
        );
      } else if !reasoning.sub_path.path_empty() {
        reason = alloc::format!(
          "{}`{}`, which is not {} `{}`",
          to_string_human(&reasoning.sub_path),
          sub_leaf_as_string,
          relation,
          super_leaf_as_string
        );
      } else {
        reason = alloc::format!(
          "{}`{}`, and {}",
          to_string_human(&reasoning.super_path),
          super_leaf_as_string,
          base_reason
        );
      }

      reasons.push(reason);

      // if we haven't already proved this isn't suppressing, we have to keep checking.
      if suppressed {
        if let (Some(sl), Some(supl)) = (sub_leaf_ty, super_leaf_ty) {
          suppressed &= self.is_error_suppressing_location_type_id(location, sl)
            || self.is_error_suppressing_location_type_id(location, supl);
        } else {
          suppressed &= self
            .is_error_suppressing_location_type_pack_id(location, sub_leaf_tp.unwrap())
            || self.is_error_suppressing_location_type_pack_id(location, super_leaf_tp.unwrap());
        }
      }
    }

    Reasonings {
      reasons,
      suppressed,
    }
  }
}
