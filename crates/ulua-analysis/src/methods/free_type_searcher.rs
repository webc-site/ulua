//! `free_type_searcher` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::{
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_set::DenseHashSet, insertion_ordered_map::InsertionOrderedMap},
};

use crate::{
  enums::polarity::Polarity,
  functions::invert_polarity::invert_polarity,
  records::{
    free_type_searcher::FreeTypeSearcher, scope::Scope, type_visitor::TypeVisitor,
    visit_key::VisitKeyRef,
  },
  type_aliases::type_id::TypeId,
};

impl FreeTypeSearcher {
  pub fn flip(&mut self) {
    self.polarity = invert_polarity(self.polarity);
  }
}

impl FreeTypeSearcher {
  pub fn new(scope: *mut Scope, cached_types: *mut DenseHashSet<TypeId>) -> Self {
    Self {
      base: TypeVisitor::new("FreeTypeSearcher".to_string(), true),
      scope,
      cached_types,
      is_within_function: false,
      polarity: Polarity::Positive,
      seen_positive: DenseHashSet::default(),
      seen_negative: DenseHashSet::default(),
      types: InsertionOrderedMap::new(),
      type_packs: InsertionOrderedMap::new(),
      unsealed_tables: DenseHashSet::default(),
    }
  }
}

impl FreeTypeSearcher {
  pub fn seen_with_current_polarity(&mut self, ty: *const ()) -> bool {
    let key = VisitKeyRef::from_ptr(ty);
    match self.polarity {
      Polarity::Positive => {
        if self.seen_positive.contains(&key) {
          return true;
        }
        self.seen_positive.insert(key);
        false
      }
      Polarity::Negative => {
        if self.seen_negative.contains(&key) {
          return true;
        }
        self.seen_negative.insert(key);
        false
      }
      Polarity::Mixed => {
        if self.seen_positive.contains(&key) && self.seen_negative.contains(&key) {
          return true;
        }
        self.seen_positive.insert(key);
        self.seen_negative.insert(key);
        false
      }
      _ => {
        LUAU_ASSERT!(false);
        false
      }
    }
  }
}
