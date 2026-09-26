use ulua_common::records::f_value::{FValue, FValueOverridable};

/// cpp `tests/ScopedFlags.h:7-46` 的 `template<typename T> struct ScopedFValue`：
/// 构造时接管一个进程期存活的 `FValue<T>`，析构时恢复。
///
/// 上游字段是 `FValue<T>* value = nullptr`，`nullptr` 只为“被移动走的对象析构时不再
/// 恢复”服务；Rust 的移动不会触发源对象的 `Drop`，因此这里直接用 `&'static
/// FValue<T>` 表达同一个不变量 —— 不需要裸指针、不需要 `PhantomData` 维持自动 trait、
/// 也不需要构造/析构两处 `unsafe`。
///
/// 与上游的另一处差异：恢复走 `FValue` 的线程本地覆盖层（`push_test_override` /
/// `pop_test_override`），因为 libtest/nextest 会并行跑用例，直接改全局值会互相串扰。
#[must_use = "作用域守卫被立即丢弃，flag 覆盖会马上失效；请用 `let _name = ...` 绑定"]
pub struct ScopedFValue<T: FValueOverridable + 'static> {
  /// 被本作用域接管的那个 flag（进程期存活，构造时已压入线程本地覆盖）。
  pub(crate) value: &'static FValue<T>,
}
