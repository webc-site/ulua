use core::ptr::null_mut;

use crate::{
  enums::polarity::Polarity,
  records::{scope::Scope, type_level::TypeLevel},
  type_aliases::name_type::Name,
};
#[derive(Debug, Clone)]
pub struct GenericType {
  pub index: i32,
  pub level: TypeLevel,
  /// §2(b)：cpp `GenericType { Scope* scope = nullptr; }`（Type.h，空即「全局泛型、
  /// 无量化作用域」）。本字段与 `FreeType.scope`、`FreeTypePack.scope` 同属一整条以
  /// **裸 `*mut Scope` 身份**运作的约束/泛型子系统：它被直接用作
  /// `ConstraintSet.scope_to_function: DenseHashMap<*mut Scope, TypeId>` 的键
  /// （`constraint_solver.rs: find(&free_ty.scope)`），在 `subsumes(alias_opt(x.scope), ..)`、
  /// `track_interior_free_type(ttv.scope, ..)` 等裸形参间流转，被 type_cloner/
  /// replace_generics/weird_iter 在裸 `*mut Scope` 字段间互相拷贝，并由 type_stringifier
  /// `emit_level` 沿 scope 链解引用上溯。单把本字段改 `Option<NonNull>` 会在每个 map 键、
  /// `subsumes`/`track_interior_*` 形参和链式解引用边界逼出 `.as_ptr()` 倒灌，或被迫重打
  /// 这些 map/形参及十余处同型 `*mut Scope` 兄弟字段（均超出本轮 12 字段范围），收益仅
  /// 形态，故连同该子系统整体保留。空值构造已收口于 `Default`/具名 ctor。
  pub scope: *mut Scope,
  pub name: Name,
  pub explicit_name: bool,
  pub polarity: Polarity,
}

impl Default for GenericType {
  fn default() -> Self {
    Self {
      index: 0,
      level: TypeLevel::default(),
      scope: null_mut(),
      name: Name::default(),
      explicit_name: false,
      polarity: Polarity::Unknown,
    }
  }
}
