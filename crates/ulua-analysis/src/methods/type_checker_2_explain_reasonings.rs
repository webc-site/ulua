use alloc::{string::String, vec::Vec};

use ulua_ast::records::location::Location;
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::subtyping_variance::SubtypingVariance,
  functions::{
    to_string_human::to_string_human,
    to_string_to_string::to_string_type_or_pack,
    traverse_type_path::{self, traverse_type_pack_root as traverse},
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
    traverse_type_path::traverse_type_root(self, path, builtin_types, arena)
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

      // self.builtin_types/self.subtyping（及其 arena 句柄）均已句柄化/Handle 化，
      // 构造期接线、非空存活；explain_traverse 依 sub_path 在 arena 上求解返回 Option 叶节点，
      // &mut self 独占该步骤、无并存可变别名（借用止于本语句）。
      let opt_sub_leaf = sub_ty.explain_traverse(
        &reasoning.sub_path,
        self.builtin_types.get(),
        self.subtyping_mut().arena.get_mut(),
      );
      // 同上——builtin_types/subtyping.arena 均为构造期接线的非空句柄；
      // explain_traverse 依 super_path 在 arena 上求解返回 Option 叶节点，单线程 &mut self
      // 独占，前一个 sub 叶子借用已在上一语句结束、不与此并存别名。
      let opt_super_leaf = super_ty.explain_traverse(
        &reasoning.super_path,
        self.builtin_types.get(),
        self.subtyping_mut().arena.get_mut(),
      );

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

      if fflag::LuauPropertyModifierMismatchErrors.get() && reasoning.is_property_modifier_violation
      {
        // The leaf types at the end of the paths are the same type, so a
        // plain "X is not a subtype of X" message would be misleading.
        let mut prop_name = String::from("a property");
        let mut is_read_only = true;
        let last = reasoning
          .sub_path
          .last()
          .expect("cpp LUAU_ASSERT：进入本支时 sub_path 末元素必为 Property 分量");
        LUAU_ASSERT!(matches!(last, Component::Property(_)));
        if let Component::Property(prop) = last {
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
      } else if reasoning.sub_path == reasoning.super_path {
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
          suppressed &= self.is_error_suppressing_location_type_pack_id(
            location,
            sub_leaf_tp.expect("TypeOrPack leaf 型/包二选一：ty 不双全的 else 支必为包"),
          ) || self.is_error_suppressing_location_type_pack_id(
            location,
            super_leaf_tp.expect("TypeOrPack leaf 型/包二选一：ty 不双全的 else 支必为包"),
          );
        }
      }
    }

    Reasonings {
      reasons,
      suppressed,
    }
  }

  // C++ `Reasonings TypeChecker2::explainReasonings(TypeId, TypeId, Location,
  // const SubtypingResult&)` (TypeChecker2.cpp:3136-3139) — forwards to the
  // templated `explainReasonings_`.
  pub fn explain_reasonings_type_id_type_id_location_subtyping_result(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    location: Location,
    r: &SubtypingResult,
  ) -> Reasonings {
    self.explain_reasonings_generic(sub_ty, super_ty, location, r)
  }

  // C++ `Reasonings TypeChecker2::explainReasonings(TypePackId, TypePackId,
  // Location, const SubtypingResult&)` — forwards to the templated
  // `explainReasonings_`.
  pub fn explain_reasonings_type_pack_id_type_pack_id_location_subtyping_result(
    &mut self,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    location: Location,
    r: &SubtypingResult,
  ) -> Reasonings {
    self.explain_reasonings_generic(sub_tp, super_tp, location, r)
  }
}
