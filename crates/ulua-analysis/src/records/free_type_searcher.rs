use ulua_common::records::{
  dense_hash_set::DenseHashSet, insertion_ordered_map::InsertionOrderedMap,
};

use crate::{
  enums::polarity::Polarity,
  records::{
    generalization_params::GeneralizationParams, scope::Scope, type_visitor::TypeVisitor,
    visit_key::VisitKeyRef,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
#[derive(Debug)]
pub struct FreeTypeSearcher<'a> {
  pub base: TypeVisitor,
  /// 泛化作用域的非拥有共享借用（原 `*mut Scope` 裸指针字段，读取仅经 `subsumes`）。
  pub scope: &'a Scope,
  /// 调用方（ConstraintSolver 的 cache 集）注入的可变借用；searcher 遍历期内仅读取
  /// `contains`，遍历结束后由 `generalize_impl` 移交 `TypeCacher` 写入。
  pub cached_types: &'a mut DenseHashSet<TypeId>,
  pub is_within_function: bool,
  pub polarity: Polarity,
  pub seen_positive: DenseHashSet<VisitKeyRef>,
  pub seen_negative: DenseHashSet<VisitKeyRef>,
  pub types: InsertionOrderedMap<TypeId, GeneralizationParams>,
  pub type_packs: InsertionOrderedMap<TypePackId, GeneralizationParams>,
  pub unsealed_tables: DenseHashSet<TypeId>,
}
