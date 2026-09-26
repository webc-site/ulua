use core::ptr::null;

use crate::records::{quantifier::Quantifier, scope::Scope, scope_registry::resolve_scope};
impl Quantifier {
  pub fn subsumes(&mut self, outer: *mut Scope, inner: *mut Scope) -> bool {
    // 沿 parent 句柄链上溯，与原裸指针遍历同构：句柄仅经 resolve_scope 只读
    // 还原为地址做身份比较，不解引用写入。
    let mut current: *const Scope = inner.cast_const();
    while !current.is_null() {
      if current == outer.cast_const() {
        return true;
      }
      // SAFETY: `current` 由 while 守卫保证非空，且初值 `inner`、后续值均为
      // 注册表还原的存活 Scope 地址（迁移前同址裸指针的存活前提逐字保留），
      // 此处仅只读取 `parent` 句柄做链上溯。
      let Some(parent_id) = (unsafe { &*current }).parent else {
        return false;
      };
      current = resolve_scope(parent_id).map_or(null(), |scope| scope as *const Scope);
    }
    false
  }
}
