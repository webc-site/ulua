//! 元方法名字面量的单点表。
//!
//! cpp 在 `TypeInfer.cpp:194` 与 `ConstraintGenerator.cpp:2281` 各写了一份
//! **逐字相同**的 20 项 `name == "__index" || name == "__newindex" || …` 短路链，
//! Rust 直译于是把同一张名单抄了两遍（`is_metamethod` / `is_metamethod_mut`，
//! 后者还先调用前者、再对同一批字面量重判一次，命中集恒空）。本模块把名单收为
//! 一枚编译期常量数组，两个入口共享同一判定，增删元方法只改这里。

/// cpp `Luau::isMetamethod` 的完整元方法名单（顺序与 cpp 短路链一致）。
pub(crate) const METAMETHODS: &[&str] = &[
  "__index",
  "__newindex",
  "__call",
  "__concat",
  "__unm",
  "__add",
  "__sub",
  "__mul",
  "__div",
  "__mod",
  "__pow",
  "__tostring",
  "__metatable",
  "__eq",
  "__lt",
  "__le",
  "__mode",
  "__iter",
  "__len",
  "__idiv",
];
