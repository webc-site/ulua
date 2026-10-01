use core::ptr::null_mut;

use crate::{
  enums::polarity::Polarity,
  functions::fresh_index::fresh_index,
  records::{generic_type::GenericType, scope::Scope, type_level::TypeLevel},
  type_aliases::name_type::Name,
};

// B 型（arena 结点布局契约）：`GenericType.scope` 为 `*mut Scope`
// （records/generic_type.rs，TypeArena bump 结点字段），空指针实义为
// 「全局泛型、无量化作用域」（cpp Type.h:124 字段默认 `Scope* scope = nullptr`）；
// 写入方除本组 ctor 外还有 clone/type_cloner（接口导出时把已提升为全局的泛型
// 清回无作用域态），消费方如 type_stringifier `emit_level`（沿 scope 链上溯，
// 空即 0 层）。以下 ctor 即 cpp 同名重载的对应物；重复的结点字面量收口为
// 私有构造门面，`fresh_index` 恰一次、空指针哨兵仅存于 `global` 一处字段
// 写入点（既有约定，本批次不改 `scope` 字段类型，带 scope ctor 的裸指针
// 形参直存字段亦属该约定）。
impl GenericType {
  /// 「全局泛型、无量化作用域」构造：B 型 `scope` 字段哨兵的唯一写入收口。
  fn global(
    index: i32,
    level: TypeLevel,
    name: Name,
    explicit_name: bool,
    polarity: Polarity,
  ) -> Self {
    GenericType {
      index,
      level,
      scope: null_mut(),
      name,
      explicit_name,
      polarity,
    }
  }

  /// 带量化作用域构造：`scope` 为调用方自 arena 传入的 `*mut Scope` 身份指针，
  /// 直存记录字段（同上既有约定）。
  fn scoped(
    index: i32,
    level: TypeLevel,
    scope: *mut Scope,
    name: Name,
    explicit_name: bool,
    polarity: Polarity,
  ) -> Self {
    GenericType {
      index,
      level,
      scope,
      name,
      explicit_name,
      polarity,
    }
  }

  /// 自动命名（`g{index}`）的全局泛型：index 与名字同源，恰一次 `fresh_index`。
  fn auto_named(level: TypeLevel) -> Self {
    let index = fresh_index();
    let name = Name::from(format!("g{}", index).as_str());
    Self::global(index, level, name, false, Polarity::Unknown)
  }

  pub fn new() -> Self {
    Self::auto_named(TypeLevel::default())
  }

  pub fn generic_type_type_level(level: TypeLevel) -> Self {
    Self::auto_named(level)
  }

  pub fn generic_type_name_polarity(name: &str, polarity: Polarity) -> Self {
    Self::global(
      fresh_index(),
      TypeLevel::default(),
      name.to_owned(),
      true,
      polarity,
    )
  }

  pub fn generic_type_scope_polarity(scope: *mut Scope, polarity: Polarity) -> Self {
    Self::scoped(
      fresh_index(),
      TypeLevel::default(),
      scope,
      Name::default(),
      false,
      polarity,
    )
  }

  pub fn generic_type_type_level_name(level: TypeLevel, name: &Name) -> Self {
    Self::global(fresh_index(), level, name.clone(), true, Polarity::Unknown)
  }

  pub fn generic_type_scope_name(scope: *mut Scope, name: &Name) -> Self {
    Self::scoped(
      fresh_index(),
      TypeLevel::default(),
      scope,
      name.clone(),
      true,
      Polarity::Unknown,
    )
  }

  pub fn generic_type_scope_name_polarity(
    scope: *mut Scope,
    name: Name,
    polarity: Polarity,
  ) -> Self {
    Self::scoped(
      fresh_index(),
      TypeLevel::default(),
      scope,
      name,
      true,
      polarity,
    )
  }
}
