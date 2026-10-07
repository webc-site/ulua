use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{records::visit_key::VisitKey, type_aliases::collections::HashSet};

/// 已访问集合清除：键由节点地址折算为 [`VisitKey`]，仅作身份、从不解引用。
pub fn unsee<T>(seen: &mut HashSet<VisitKey>, tv: *const T) {
  seen.remove(&VisitKey::from_ptr(tv));
}

/// [`DenseHashSet`] 版 [`unsee`]： Dense 集不支持擦除，且 `has_seen` 仅在
/// `visitOnce` 时插入，此处恒为无操作（与原实现一致）。
pub fn unsee_visit_key<T>(_seen: &mut DenseHashSet<VisitKey>, _tv: *const T) {
  // When DenseHashSet is used for 'visitTypeOnce', where don't forget visited elements
}
