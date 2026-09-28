use crate::{
  enums::normalization_result::NormalizationResult,
  functions::{follow_type, get_type, is_prim::is_nil},
  records::{
    free_type::FreeType, intersection_type::IntersectionType, normalizer::Normalizer, r#type::Type,
    type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};

/// Rust translation of Luau.Analysis::Analysis::TypeInfer.cpp:are_eq_comparable
pub fn are_eq_comparable(
  arena: &mut TypeArena,
  normalizer: &mut Normalizer,
  a: TypeId,
  b: TypeId,
) -> Option<bool> {
  let a = follow_type::follow(a);
  let b = follow_type::follow(b);

  let is_exempt = |t: TypeId| -> bool { is_nil(t) || get_type::get::<FreeType>(t).is_some() };

  if is_exempt(a) || is_exempt(b) {
    return Some(true);
  }

  let c = arena.add_type(Type::new(TypeVariant::Intersection(IntersectionType {
    parts: alloc::vec![a, b],
  })));

  // C++ 中归一化失败返回空指针 → nullopt（isInhabited(nullptr) 亦为 HitLimits）
  let n = normalizer.try_normalize(c)?;

  let nr = normalizer.is_inhabited_normalized_type(n.as_ref());

  match nr {
    NormalizationResult::HitLimits => None,
    NormalizationResult::False => Some(false),
    NormalizationResult::True => Some(true),
  }
}
