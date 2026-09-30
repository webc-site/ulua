//! TypeFunction n 元 builder 聚合骨架单点（union/intersection 对偶）。
//!
//! `add_union`/`add_intersection` 逐行同形：现场建对应 builder、按列表长度
//! reserve、逐个 add 后 build 出 `TypeId`，只有 builder 类型不同。
//! [`add_nary_builder!`] 保留全部对外路径、函数名、签名与聚合语义不变。

/// 生成一枚「用指定 builder 聚合 `&[TypeId]` 并 build」的 `pub fn` 入口。
///
/// 用法：
/// ```ignore
/// add_nary_builder!(add_union, UnionBuilder);
/// ```
macro_rules! add_nary_builder {
  ($(#[$attr:meta])* $name:ident, $builder:ident $(,)?) => {
    $(#[$attr])*
    pub fn $name(
      arena: crate::records::arena_handle::Handle<crate::records::type_arena::TypeArena>,
      builtin_types: crate::records::arena_handle::Handle<
        crate::records::builtin_types::BuiltinTypes,
      >,
      list: &[crate::type_aliases::type_id::TypeId],
    ) -> crate::type_aliases::type_id::TypeId {
      let mut builder = $builder::new(arena, builtin_types);
      builder.reserve(list.len());
      for &item in list {
        builder.add(item);
      }
      builder.build()
    }
  };
}

pub(crate) use add_nary_builder;
