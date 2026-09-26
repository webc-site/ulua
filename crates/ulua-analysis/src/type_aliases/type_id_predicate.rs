//! 上游 `TypeIdPredicate = std::function<std::optional<TypeId>(TypeId)>`
//! (`Analysis/include/Luau/Type.h:1257`)：predicate 是捕获了 `this` 的 lambda，
//! 过滤联合类型时会回调 `canUnify` 等可变操作。
//!
//! Rust 侧不能让闭包长期持有 `&mut TypeChecker`（或为绕过借用检查而伪造
//! `*mut TypeChecker`）——`refine_l_value`/`filter_map` 自身还要用 `&mut self`，
//! 两者并存即构成可变别名。这里把 TypeChecker 改为回调的入参：可别名窗口只存在于
//! 单次调用之内，语义与上游捕获 `this` 完全一致。

use crate::{records::type_checker::TypeChecker, type_aliases::type_id::TypeId};

/// 类型筛选谓词：给定候选类型，返回保留（可能被改写）的类型，或 `None` 表示淘汰。
pub trait TypeIdPredicate: FnMut(&mut TypeChecker, TypeId) -> Option<TypeId> {}

impl<F: FnMut(&mut TypeChecker, TypeId) -> Option<TypeId>> TypeIdPredicate for F {}
