use crate::records::{scope::Scope, scope_registry::resolve_scope};

/// 沿 `right` 的 parent 句柄链上溯，判断链上是否命中 `left`（cpp
/// `subsumesStrict(NotNull<Scope> left, NotNull<Scope> right)` 的严格包含版）。
///
/// 引用化说明（原 `# Safety` 前提改由类型承担）：`left`/`right` 均为 scope 树
/// 内非空存活节点（cpp NotNull），此处仅只读取 `parent` 句柄并比较地址。
pub fn subsumes_strict(left: *mut Scope, right: *mut Scope) -> bool {
  if left.is_null() || right.is_null() {
    return false;
  }

  // 句柄链上溯：地址仅做身份比较，不经指针解引用写入（同迁移前只读遍历）。
  let mut current: *const Scope = right.cast_const();
  while !current.is_null() {
    // SAFETY: `current` 由守卫非空，初值 `right` 与后续自注册表还原的地址均
    // 指向存活 Scope（迁移前同址裸指针的存活前提逐字保留），仅只读 parent。
    let Some(parent_id) = (unsafe { &*current }).parent else {
      break;
    };
    let Some(parent) = resolve_scope(parent_id) else {
      break;
    };
    let parent_raw = parent as *const Scope;
    if parent_raw == left.cast_const() {
      return true;
    }
    current = parent_raw;
  }

  false
}
