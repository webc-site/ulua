use ulua_common::FFlag;

use crate::records::{
  builtin_types::BuiltinTypes, subtyping_result::SubtypingResult, type_arena::TypeArena,
};
pub fn assert_reasoning_valid<TID>(
  _sub_ty: TID,
  _super_ty: TID,
  result: &SubtypingResult,
  _builtin_types: *mut BuiltinTypes,
  _arena: *mut TypeArena,
) {
  if !FFlag::DebugLuauSubtypingCheckPathValidity.get() {
    return;
  }
  for _reasoning in result.reasoning.iter() {
    // LUAU_ASSERT!(traverse(sub_ty, reasoning.sub_path, builtin_types, arena));
    // LUAU_ASSERT!(traverse(super_ty, reasoning.super_path, builtin_types, arena));
  }
}
