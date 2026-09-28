use core::ptr::{from_ref, null_mut};

use crate::{
  enums::polarity::Polarity,
  records::{
    builtin_types::BuiltinTypes, free_type::FreeType, scope::Scope, type_arena::TypeArena,
  },
  type_aliases::type_id::TypeId,
};

/// `scope` 对应 cpp `freshType` 的可空 `Scope*`（无主 fresh 类型传 nullptr），
/// 以 `Option<&Scope>` 表达；FreeType 记录字段保持裸指针布局，此处为唯一还原点。
pub fn fresh_type(
  arena: &mut TypeArena,
  builtin_types: &BuiltinTypes,
  scope: Option<&Scope>,
  polarity: Polarity,
) -> TypeId {
  let free_type = FreeType::free_type_scope_type_id_type_id_polarity(
    scope.map_or(null_mut(), |s| from_ref(s).cast_mut()),
    builtin_types.never_type,
    builtin_types.unknown_type,
    polarity,
  );
  arena.add_type(free_type)
}
