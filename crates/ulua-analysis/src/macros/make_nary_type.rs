//! TypeFunction n 元类型 arena 构造骨架单点（union/intersection 对偶）。
//!
//! `make_union`/`make_intersection` 逐行同形：把一组成员类型打包成对应记录后
//! `arena.add_type` 落位，只有记录类型与字段名（`options`/`parts`）不同。
//! [`make_nary_type!`] 保留全部对外路径、函数名、签名与构造语义不变。

/// 生成一枚「把 `Vec<TypeId>` 打包成指定 n 元类型记录并 `arena.add_type`」的
/// `pub fn` 入口。
///
/// 用法：
/// ```ignore
/// make_nary_type!(make_union, UnionType, options);
/// ```
macro_rules! make_nary_type {
  ($(#[$attr:meta])* $name:ident, $record:ident, $field:ident $(,)?) => {
    $(#[$attr])*
    pub fn $name(
      arena: &mut crate::records::type_arena::TypeArena,
      types: ::alloc::vec::Vec<crate::type_aliases::type_id::TypeId>,
    ) -> crate::type_aliases::type_id::TypeId {
      arena.add_type($record { $field: types })
    }
  };
}

pub(crate) use make_nary_type;
