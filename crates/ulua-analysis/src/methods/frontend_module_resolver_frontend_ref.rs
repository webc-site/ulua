//! `FrontendModuleResolver::frontend` 自引用句柄的唯一解引用 chokepoint。
//!
//! 字段以 `Option<NonNull<Frontend>>` 建模：null 哨兵（对应 C++ 默认构造的
//! `ModuleResolver(nullptr)` 独立使用形态，恒不指向悬垂地址）在类型层显式化，
//! 构造 → `Frontend::wire_self_pointers` 布线之间不存在「悬垂但看似有效」的
//! 占位窗口；解引用集中于本方法一处。

use crate::records::{frontend::Frontend, frontend_module_resolver::FrontendModuleResolver};

impl FrontendModuleResolver {
  /// 宿主 `Frontend` 的受控只读借用；未布线（独立 resolver）时返回 `None`，
  /// 调用方按 C++ 语义走各自的空指针分支。
  pub(crate) fn frontend_ref(&self) -> Option<&Frontend> {
    // Safety: `frontend` 仅由 `Frontend::wire_self_pointers` 写入，指向持有本
    // resolver 的存活 `Frontend`（自引用，与宿主同地址同寿，落位后不得移动
    // 的契约由 unsafe fn 本身承载）；或为 `None`。借用生命周期绑定 `&self`，
    // 单线程序列化驱动（lib.rs 不变量 1）下读方借用期内无并存可变别名，与原
    // `unsafe { &*self.frontend }` 各调用点语义逐项同构。
    unsafe { self.frontend.map(|host| host.as_ref()) }
  }
}
