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
// 空即 0 层）。以下不带 scope 的 ctor 即 cpp 同名重载的对应物。
impl GenericType {
  pub fn new() -> Self {
    let index = fresh_index();
    let name = Name::from(format!("g{}", index).as_str());
    GenericType {
      index,
      level: TypeLevel::default(),
      scope: null_mut(),
      name,
      explicit_name: false,
      polarity: Polarity::Unknown,
    }
  }

  pub fn generic_type_type_level(_level: TypeLevel) -> Self {
    let index = fresh_index();
    let name = Name::from(format!("g{}", index).as_str());
    GenericType {
      index,
      level: _level,
      scope: null_mut(),
      name,
      explicit_name: false,
      polarity: Polarity::Unknown,
    }
  }

  pub fn generic_type_name_polarity(name: &str, polarity: Polarity) -> Self {
    GenericType {
      index: fresh_index(),
      level: TypeLevel::default(),
      scope: null_mut(),
      name: name.to_owned(),
      explicit_name: true,
      polarity,
    }
  }

  pub fn generic_type_scope_polarity(scope: *mut Scope, polarity: Polarity) -> Self {
    GenericType {
      index: fresh_index(),
      level: Default::default(),
      scope,
      name: Default::default(),
      explicit_name: false,
      polarity,
    }
  }

  pub fn generic_type_type_level_name(_level: TypeLevel, _name: &Name) -> Self {
    let index = fresh_index();
    GenericType {
      index,
      level: _level,
      scope: null_mut(),
      name: _name.clone(),
      explicit_name: true,
      polarity: Polarity::Unknown,
    }
  }

  pub fn generic_type_scope_name(scope: *mut Scope, name: &Name) -> Self {
    GenericType {
      index: fresh_index(),
      level: TypeLevel::default(),
      scope,
      name: name.clone(),
      explicit_name: true,
      polarity: Polarity::Unknown,
    }
  }

  pub fn generic_type_scope_name_polarity(
    scope: *mut Scope,
    name: Name,
    polarity: Polarity,
  ) -> Self {
    GenericType {
      index: fresh_index(),
      level: Default::default(),
      scope,
      name,
      explicit_name: true,
      polarity,
    }
  }
}
