//! Source: `Analysis/src/NonStrictTypeChecker.cpp:208-231` (hand-ported)

use core::ptr::{null, null_mut};

use crate::{
  enums::polarity::Polarity,
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, emplace_type_pack::emplace_type_pack,
    finite::finite, first::first, follow_type_pack, fresh_index::fresh_index, get_type_pack::get,
    size_type_pack::size,
  },
  records::{
    free_type::FreeType, free_type_pack::FreeTypePack,
    non_strict_type_checker::NonStrictTypeChecker, type_level::TypeLevel, type_pack::TypePack,
  },
  type_aliases::{
    error_type_pack::ErrorTypePack, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant,
  },
};
impl NonStrictTypeChecker {
  /// C++ `TypeId NonStrictTypeChecker::flattenPack(TypePackId pack)`.
  pub(crate) fn flatten_pack(&mut self, pack: TypePackId) -> TypeId {
    let pack = follow_type_pack::follow(pack);

    if let Some(fst) = first(pack, /*ignoreHiddenVariadics*/ false) {
      return fst;
    }

    if let Some(ftp) = get::<FreeTypePack>(pack) {
      let scope = ftp.scope;
      // Safety: `self.arena.as_ptr()` 对应 C++ `NotNull<Arena*>`，构造期接线、非空且比
      // 持有者长寿；类型 arena 的 bump 块地址不移动。此处仅向 arena 顺序追加
      // 新 FreeType 节点（cpp `&arena->addType(FreeType{...})` 取址等价），
      // 单线程串行、无其他可变句柄并存。
      let result = {
        self.arena.get_mut().add_type(FreeType {
          index: fresh_index(),
          level: TypeLevel::default(),
          scope,
          forwarded_type_alias: false,
          lower_bound: null(),
          upper_bound: null(),
          polarity: Polarity::Unknown,
        })
      };

      // Safety: 同上方 add_type——arena 非空存活、bump 地址稳定，本次仅追加
      // 一个 FreeTypePack 节点。
      let free_tail = {
        self.arena.get_mut().add_type_pack_t(FreeTypePack {
          index: fresh_index(),
          level: TypeLevel::default(),
          scope,
          polarity: Polarity::Unknown,
        })
      };

      let result_pack = {
        // Safety: `pack` 是函数头 follow 后确认存活的 FreeTypePack arena 节点，
        // as_mutable_type_pack 只是同一基址的 const→mut 视图；emplace 的
        // placement 覆写与 cpp `emplace(Arena&, TypePackId, ...)` 对非持久
        // arena 包节点的原地写语义一致，单线程下写穿无并发访问。
        unsafe {
          emplace_type_pack(
            as_mutable_type_pack(pack),
            TypePackVariant::TypePack(TypePack::new(alloc::vec![result], Some(free_tail))),
          )
        }
      };
      let _ = result_pack;

      return result;
    }

    if get::<ErrorTypePack>(pack).is_some() {
      return self.builtin_types_ref().error_type;
    }

    if unsafe {
      // Safety: `pack` 为 follow 后的存活 arena 包节点；log 传 null_mut()
      // 即 C++ `finite(tp, TxnLog* = nullptr)` 默认实参形态，函数内判空后
      // 走无事务覆盖的只读遍历。
      finite(pack, null_mut())
    } && size(pack, None) == 0
    {
      return self.builtin_types_ref().nil_type;
    }

    // Safety: `self.ice.as_ptr()` 对应 C++ `NotNull<InternalErrorReporter&>`，构造期
    // 接线、非空且比持有者长寿；ice_string 仅上报字符串。
    {
      self.ice.get().ice_string("flattenPack got a weird pack!");
    }
    self.builtin_types_ref().error_type
  }
}
