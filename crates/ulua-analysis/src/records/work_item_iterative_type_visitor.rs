use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};

/// 迭代遍历工作队列的入队项：`C++ WorkItem { const void* t; bool isType; int parent; }`
/// 的 enum 化——裸指针槽 + 手写 bool 判别改为类型安全变体，`i32 parent`
/// 是父队列项下标（-1 为根）。
#[derive(Debug, Clone)]
pub enum WorkItem {
  Type(TypeId, i32),
  Pack(TypePackId, i32),
}

impl WorkItem {
  pub fn parent(&self) -> i32 {
    match self {
      WorkItem::Type(_, parent) | WorkItem::Pack(_, parent) => *parent,
    }
  }

  pub fn type_id(&self) -> Option<TypeId> {
    match self {
      WorkItem::Type(ty, _) => Some(*ty),
      WorkItem::Pack(..) => None,
    }
  }

  pub fn type_pack_id(&self) -> Option<TypePackId> {
    match self {
      WorkItem::Type(..) => None,
      WorkItem::Pack(tp, _) => Some(*tp),
    }
  }
}
