use core::ptr::null_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    type_function_type::TypeFunctionType, type_function_type_pack_var::TypeFunctionTypePackVar,
  },
  type_aliases::{
    type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariantMember,
    type_function_type_variant::TypeFunctionTypeVariantMember,
  },
};

/// B 型（类型 arena 结点的 cpp 互操作面）：按变体 tag 取可变字段的查找结果，
/// 折回 cpp `get_if<T>` 可变重载的裸指针形态。空指针的含义与消费方：
/// 变体不匹配（或入参 id 为空，属契约违例的确定性兜底）=「未命中」，消费方
/// （type_function_cloner/deserializer 与 set_* 魔术函数系）逐处 `is_null()`/
/// `as_mut()` 判命中，命中后指针指回 arena 结点内部、随结点存活。
/// 不折成 `Option<&mut T>`：借用生命周期与入参指针无关（源自 arena 而非某次
/// 借用），`&mut` 无法诚实地标注该 lifetime，故按 cpp `get_if` 形态透传裸指针
/// （与 `Handle::get` 刻意不受约束的借用同构）。
///
/// # Safety
/// `tv` 须为空、或指向 `TypeFunctionRuntime` 的 type_pack_arena 分配的
/// `TypeFunctionTypePackVar` 结点（地址随 arena 存活期不迁移）；本函数仅在
/// 类型检查器单线程、顺序改写路径中调用（cpp `get_if` 可变重载同前提），
/// 返回的可变别名在调用点独占使用、不得与同一结点的其它并存借用重叠。
pub unsafe fn get_mutable_type_function_type_pack_id<T: TypeFunctionTypePackVariantMember>(
  tv: TypeFunctionTypePackId,
) -> *mut T {
  // Safety: 入参判空兜底后仅对非空 `tv` 解引用取 `type_variant` 的可变借用，
  // 独占性由本函数 `# Safety` 的单线程顺序改写契约担保；命中返回内部字段指针，
  // 未命中折回空指针（见函数头的消费契约）。
  unsafe {
    LUAU_ASSERT!(!tv.is_null());

    if tv.is_null() {
      return null_mut();
    }
    // C++ `get_if<T>(&const_cast<TypeFunctionTypePackVar*>(tv)->type)`.
    match T::get_if_mut(&mut (*(tv as *mut TypeFunctionTypePackVar)).type_variant) {
      Some(r) => r as *mut T,
      None => null_mut(),
    }
  }
}

/// B 型（类型 arena 结点的 cpp 互操作面）：按变体 tag 取可变字段的查找结果，
/// 折回 cpp `get_if<T>` 可变重载的裸指针形态。空指针的含义与消费方见
/// [`get_mutable_type_function_type_pack_id`] 的函数头（本函数为其 type 侧对偶，
/// 结点是 type_arena 的 `TypeFunctionType`）。
///
/// # Safety
/// `tv` 须为空、或指向 type_arena 分配的 `TypeFunctionType` 结点（地址随 arena
/// 存活期不迁移）；仅在单线程、顺序改写路径中调用，返回的可变别名在调用点独占
/// 使用、不得与同一结点的其它并存借用重叠。
pub unsafe fn get_mutable_type_function_type_id<T: TypeFunctionTypeVariantMember>(
  tv: TypeFunctionTypeId,
) -> *mut T {
  // Safety: 与 pack 侧对偶——入参判空兜底后仅对非空 `tv` 解引用取
  // `type_variant` 的可变借用，独占性由 `# Safety` 契约担保；命中返回内部
  // 字段指针，未命中折回空指针。
  unsafe {
    LUAU_ASSERT!(!tv.is_null());

    if tv.is_null() {
      return null_mut();
    }
    // C++ `get_if<T>(&const_cast<TypeFunctionType*>(tv)->type)`.
    match T::get_if_mut(&mut (*(tv as *mut TypeFunctionType)).type_variant) {
      Some(r) => r as *mut T,
      None => null_mut(),
    }
  }
}
