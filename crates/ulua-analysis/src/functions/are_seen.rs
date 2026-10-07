use core::ptr::from_ref;

use crate::type_aliases::seen_set_structural_type_equality::SeenSet;

/// 以地址身份判定「节点对是否已入 seen 集」，防结构比较成环。
///
/// 指针身份收口在此一处：调用点传共享引用，内部把引用降为 `*const ()`
/// 地址作 `SeenSet` 键。键仅参与比较与集合存取、从不解引用；引用本身
/// 即保证非空与存活，无额外安全性依赖。
pub fn are_seen<T: ?Sized>(seen: &mut SeenSet, lhs: &T, rhs: &T) -> bool {
  let lhs_key: *const () = from_ref(lhs).cast();
  let rhs_key: *const () = from_ref(rhs).cast();

  if lhs_key == rhs_key {
    return true;
  }

  if seen.contains(&(lhs_key, rhs_key)) {
    return true;
  }

  seen.insert((lhs_key, rhs_key));
  false
}
