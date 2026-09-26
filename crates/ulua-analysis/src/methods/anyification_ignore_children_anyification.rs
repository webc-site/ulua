use crate::{
  functions::{
    clone_clone::{pack_is_persistent, type_is_persistent},
    get_type,
  },
  records::{anyification::Anyification, extern_type::ExternType},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Anyification {
  /// extern 类型不参与替换遍历。对应 C++ `bool Anyification::ignoreChildren(TypeId ty)`
  /// （Anyification.cpp:89-94）。降 safe：`ty` 为 arena 句柄（同 `get_type_id` 门面
  /// 纪律），`(*ty).persistent` 解引用收进窄 `unsafe` 块。
  pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
    let et = get_type::get::<ExternType>(ty);
    if et.is_some() {
      return true;
    }

    // type_is_persistent（clone_clone 的 pub(crate) 探针）内部窄块证成句柄前提。
    type_is_persistent(ty)
  }

  /// 类型包孪生版；降 safe 理由同上。
  /// `bool Anyification::ignoreChildren(TypePackId ty)` (Anyification.cpp:96-101).
  pub fn ignore_children_type_pack_id(&mut self, ty: TypePackId) -> bool {
    // 同 TypeId 版：pack_is_persistent 探针内已证成 arena 句柄前提。
    pack_is_persistent(ty)
  }
}
