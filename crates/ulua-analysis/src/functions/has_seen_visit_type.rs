use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{records::visit_key::VisitKey, type_aliases::collections::HashSet};

/// 已访问集合探测：键由节点地址（裸指针别名 `TypeId`/`TypePackId` 或任意
/// 节点指针）折算为 [`VisitKey`]，仅作身份、从不解引用。
pub fn has_seen<T>(seen: &mut HashSet<VisitKey>, tv: *const T) -> bool {
  !seen.insert(VisitKey::from_ptr(tv))
}

/// [`DenseHashSet`] 版 [`has_seen`]，键折算同源。
pub fn has_seen_visit_key<T>(seen: &mut DenseHashSet<VisitKey>, tv: *const T) -> bool {
  let key = VisitKey::from_ptr(tv);

  if seen.contains(&key) {
    return true;
  }

  seen.insert(key);
  false
}
