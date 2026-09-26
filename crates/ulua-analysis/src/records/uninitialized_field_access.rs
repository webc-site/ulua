//! C++ `Luau::UninitializedFieldAccess`（`Error.h:615-620`）。
//!
//! classdef 构造函数里在字段初始化完成之前访问 `self` 的报错。
//! `field_name` 为 `None` 时表示 `self` 整体在非空字段全部初始化前被用作右值。

use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UninitializedFieldAccess {
  pub field_name: Option<String>,
}

impl UninitializedFieldAccess {
  pub const fn new(field_name: Option<String>) -> Self {
    Self { field_name }
  }
}
