use core::ptr::null_mut;

use crate::{
  enums::polarity::Polarity,
  functions::fresh_index::fresh_index,
  records::{scope::Scope, type_level::TypeLevel},
};
#[derive(Debug, Clone)]
pub struct FreeTypePack {
  pub(crate) index: i32,
  pub(crate) level: TypeLevel,
  /// §2(b)：cpp `FreeTypePack { Scope* scope; }`。与 `GenericType.scope`、
  /// `FreeType.scope` 同属一整条以**裸 `*mut Scope` 身份**运作的约束/泛型子系统：本字段
  /// 经 `weird_iter.rs`（`if !free_pack.scope.is_null() { self.scope = free_pack.scope }`）
  /// 在裸 `*mut Scope` 字段间拷贝，被 `free_type_searcher` 的
  /// `subsumes(alias_opt(ftp.scope), ..)` 等裸形参消费，并由 type_cloner/replace_generics
  /// 以裸指针读写。单改本字段会在这些拷贝/形参边界逼出 `.as_ptr()` 倒灌，或被迫重打整条
  /// 裸 `*mut Scope` 兄弟字段与 map（超出本轮 12 字段范围），收益仅形态，故连子系统整体
  /// 保留；空值构造已收口于 `new`/setter。
  pub(crate) scope: *mut Scope,
  pub(crate) polarity: Polarity,
}

impl FreeTypePack {
  pub fn new(level: TypeLevel) -> Self {
    Self {
      index: fresh_index(),
      level,
      scope: null_mut(),
      polarity: Polarity::Unknown,
    }
  }
}
