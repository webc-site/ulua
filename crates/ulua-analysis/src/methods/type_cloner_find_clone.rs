use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::follow_option::FollowOption,
  functions::{follow_type, follow_type_pack},
  records::type_cloner::TypeCloner,
  type_aliases::{type_id::TypeId, type_or_pack::TypeOrPack, type_pack_id::TypePackId},
};

impl TypeCloner {
  pub(crate) fn find_type_id(&self, ty: TypeId) -> Option<TypeId> {
    let ty = follow_type::follow_with_option(ty, FollowOption::DisableLazyTypeThunks);

    // Safety: `self.types` 是克隆会话入口以 `&mut` 注入的 SeenTypes 映射裸化
    // 句柄（C++ 引用成员），会话期内存活；find/get 为只读访问，与 `ty` 指向
    // 的 arena Type 节点无重叠。
    if let Some(it) = unsafe { (*self.types).get(&ty) } {
      return Some(*it);
    } else if unsafe {
      // Safety: `ty` 经上方 follow 收敛为存活 arena Type 节点（C++ TypeId 直译，
      // 非持久即普通 arena 驻留），仅只读 persistent 标志位。
      (*ty).persistent
    } && ty != self.force_ty
    {
      return Some(ty);
    }

    None
  }

  pub(crate) fn find_type_pack_id(&self, tp: TypePackId) -> Option<TypePackId> {
    let tp = follow_type_pack::follow(tp);

    // Safety: `self.packs` 为克隆会话入口 `&mut` 注入的 SeenTypePacks 映射
    // 裸化句柄，会话期存活，只读 get。
    if let Some(it) = unsafe { (*self.packs).get(&tp) } {
      return Some(*it);
    } else if unsafe {
      // Safety: `tp` 经 follow 指向存活 arena TypePackVar 节点，仅只读
      // persistent 标志（与 find_type_id 的 TypeId 分支同构）。
      (*tp).persistent
    } && tp != self.force_tp
    {
      return Some(tp);
    }

    None
  }

  pub fn find_type_or_pack(&self, kind: TypeOrPack) -> Option<TypeOrPack> {
    if let Some(ty) = TypeOrPack::get_if::<TypeId>(&kind) {
      self.find_type_id(*ty).map(TypeOrPack::V0)
    } else if let Some(tp) = TypeOrPack::get_if::<TypePackId>(&kind) {
      self.find_type_pack_id(*tp).map(TypeOrPack::V1)
    } else {
      LUAU_ASSERT!(false);
      None
    }
  }
}
