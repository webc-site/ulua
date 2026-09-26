/// DataFlowGraphBuilder visit 家族的共享指针门面（原 7 份逐字相同的私有 helper 收口单点）。
///
/// # Safety 承接说明
/// 各调用点均先经类索引命中或 parser 非空不变量守卫，`ptr` 指向 parse arena
/// 存活节点且分析期只读（无并存 `&mut`），`as_ref` 产出的共享引用满足节点完整
/// 初始化与对齐要求；违约即 `what` 标注的 parser 契约破坏，直接 panic。
pub(crate) fn arena_ref<T>(ptr: *mut T, what: &str) -> &'static T {
  // SAFETY: 见上；契约由各调用点成立。
  unsafe { ptr.as_ref() }
    .unwrap_or_else(|| panic!("{what} 非空为 parser 契约，分析器产出了空子节点指针"))
}
