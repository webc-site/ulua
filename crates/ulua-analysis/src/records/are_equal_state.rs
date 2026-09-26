use std::collections::BTreeSet;
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AreEqualState {
  pub(crate) seen: BTreeSet<(*const (), *const ())>,
  pub(crate) recursion_count: i32,
}

/// # Safety
///
/// `seen` 中的 `*const ()` 仅作为结构性相等 memo 的**身份键**使用，从不
/// 解引用；其指向的类型/arena 节点在本 `AreEqualState` 存活期内保持有效。满足此
/// 不变量时，将该 memo 集合转移到其它线程（Send）是可靠的。
// Safety: seen 里存的 *const () 仅被插入/比较身份（BTreeSet 的 Ord 作用于
// 指针值本身），从不调用 drop、从不解引用为引用；recursion_count 为纯 i32。
// 转移 AreEqualState 即转移该 memo 集合的唯一所有权，所指 arena 节点在本状态
// 存活期内有效的前提下，跨线程独占使用不破坏不变量，Send 成立。
unsafe impl Send for AreEqualState {}
/// # Safety
///
/// 同上：裸指针只作只读身份比较，跨线程共享 `&AreEqualState` 不产生数据竞争，
/// 只要所指 arena 节点在该状态存活期内有效。
// Safety: &AreEqualState 共享路径只能对指针值做只读 Ord 比较与 i32 读取，
// 不触及所指对象内容，也不存在 &mut 出口，并发共享为纯读，Sync 成立。
unsafe impl Sync for AreEqualState {}
