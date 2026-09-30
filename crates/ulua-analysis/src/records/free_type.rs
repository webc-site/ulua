use core::ptr::{null, null_mut};

use crate::{
  enums::polarity::Polarity,
  records::{scope::Scope, type_level::TypeLevel},
  type_aliases::type_id::TypeId,
};
#[derive(Debug, Clone)]
pub struct FreeType {
  pub index: i32,
  pub level: TypeLevel,
  /// §2(b)：cpp `FreeType { Scope* scope; }`。与 `GenericType.scope`、
  /// `FreeTypePack.scope` 同属一整条以**裸 `*mut Scope` 身份**运作的约束/泛型子系统：
  /// 本字段被直接用作 `ConstraintSet.scope_to_function: DenseHashMap<*mut Scope, TypeId>`
  /// 的键（`constraint_solver.rs: self.constraint_set.scope_to_function.find(&free_ty.scope)`），
  /// 在 `subsumes(alias_opt(ft.scope), ..)`、`track_interior_free_type(..)` 等裸形参间流转，
  /// 并被 unifier 以 `self.scope.as_ptr()` 物化写入。单改本字段会在每个 map 键与裸形参边界
  /// 逼出 `.as_ptr()` 倒灌，或被迫重打该 map 及十余处同型 `*mut Scope` 兄弟字段（超出本轮
  /// 12 字段范围），收益仅形态，故连子系统整体保留；空值构造已收口于 `Default`。
  pub scope: *mut Scope,
  /// True if this free type variable is part of a mutually
  /// recursive type alias whose definitions haven't been
  /// resolved yet.
  pub forwarded_type_alias: bool,
  /// Only used under local type inference
  pub lower_bound: TypeId,
  /// Only used under local type inference
  pub upper_bound: TypeId,
  pub polarity: Polarity,
}

impl Default for FreeType {
  fn default() -> Self {
    Self {
      index: 0,
      level: TypeLevel::default(),
      scope: null_mut(),
      forwarded_type_alias: false,
      lower_bound: null(),
      upper_bound: null(),
      polarity: Polarity::Unknown,
    }
  }
}
