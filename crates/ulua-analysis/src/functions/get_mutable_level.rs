use core::ptr::null_mut;

use crate::{
  functions::get_level_type::get_level, records::type_level::TypeLevel,
  type_aliases::type_id::TypeId,
};
/// §2 判定注（B 型）：cpp `Type.cpp` `getMutableLevel` 的未命中即 `nullptr`，返回值指向
/// arena 结点内部（无诚实 lifetime 可标），与本 crate `get_mutable_type_function_runtime`
/// 的 `get_if` 折回面同族——空指针 = 「该 TypeId 无可变 level」，消费方判读不折 Option。
pub fn get_mutable_level(ty: TypeId) -> *mut TypeLevel {
  if let Some(level) = get_level(ty) {
    level as *const TypeLevel as *mut TypeLevel
  } else {
    null_mut()
  }
}
