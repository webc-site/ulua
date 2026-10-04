//! Source: `Analysis/include/Luau/Subtyping.h` (hand-ported; fields only)
//!
//! cpp 的 `SubtypingEnvironment* parent` 裸链在此扁平化为帧栈：cpp 每进入一层
//! nested generic bounds 就默认构造子环境并仅挂 parent
//! （`Subtyping.cpp:3088-3089`），五张表逐层新建、读沿链外行、写只落当前层，
//! 与 `scopes` 栈（栈顶 = cpp 当前传入的子环境）逐字段同构。

use alloc::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    generic_bounds::GenericBounds, mapped_generic_environment::MappedGenericEnvironment,
    subtyping_result::SubtypingResult, type_pair_hash::TypePairHash,
  },
  type_aliases::type_id::TypeId,
};

/// 单个 generic 作用域私有的映射表，对应 cpp `SubtypingEnvironment` 除
/// `parent` 外的全部字段（含 `iterationCount`：cpp `Subtyping.cpp:709` 在
/// 当前 env 上自增，子环境从 0 起算，迭代上限按层计）。
#[derive(Debug)]
pub struct GenericScope {
  pub mapped_generics: DenseHashMap<TypeId, Vec<GenericBounds>>,
  pub mapped_generic_packs: MappedGenericEnvironment,
  pub substitutions: DenseHashMap<TypeId, TypeId>,
  pub seen_set_cache: DenseHashMap<(TypeId, TypeId), SubtypingResult, TypePairHash>,
  pub iteration_count: i32,
}

/// 不变量：`scopes` 至少含根帧（仅 `new` 构造、`pop_scope` 绝不弹空），
/// 栈顶即 cpp 语义下的「当前环境」。
#[derive(Debug)]
pub struct SubtypingEnvironment {
  pub scopes: Vec<GenericScope>,
}
