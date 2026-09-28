//! `frontend_module_resolver` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::string::String;
use core::ptr::{NonNull, from_ref};

use parking_lot::Mutex;
use ulua_ast::{records::ast_expr::AstExpr, rtti::AstNodePtr};

use crate::{
  records::{
    frontend::Frontend, frontend_module_resolver::FrontendModuleResolver, module_info::ModuleInfo,
  },
  type_aliases::{
    collections::HashMap, module_name_type::ModuleName, module_ptr_module::ModulePtr,
  },
};

impl FrontendModuleResolver {
  /// C++ `FrontendModuleResolver::clearModules` (`Analysis/src/Frontend.cpp:1979`):
  /// clears the cache under the module mutex.
  pub fn clear_modules(&mut self) {
    // parking_lot 互斥量无投毒语义，与 cpp `std::lock_guard` 完全一致。
    let _lock = self.module_mutex.lock();
    self.modules.clear();
  }
}

impl FrontendModuleResolver {
  /// C++ `FrontendModuleResolver::eraseModule` (`Analysis/src/Frontend.cpp:2841`):
  /// drops one cached module under the module mutex.
  pub fn erase_module(&mut self, module_name: &ModuleName) {
    // parking_lot 互斥量无投毒语义，与 cpp `std::lock_guard` 完全一致。
    let _lock = self.module_mutex.lock();
    self.modules.remove(module_name);
  }
}

// C++ `FrontendModuleResolver::FrontendModuleResolver(Frontend* frontend)`
// (`Analysis/src/Frontend.cpp:1922`): stores the owning frontend; `modules`
// and `moduleMutex` are default-initialized.

impl FrontendModuleResolver {
  /// `None` 对应 C++ 的 `ModuleResolver(nullptr)` 独立形态；宿主布线由
  /// `Frontend::wire_self_pointers` 以 `Some` 完成。
  pub fn new(frontend: Option<NonNull<Frontend>>) -> Self {
    Self {
      frontend,
      module_mutex: Mutex::new(()),
      modules: HashMap::new(),
    }
  }
}

// `FrontendModuleResolver::frontend` 自引用句柄的唯一解引用 chokepoint。
//
// 字段以 `Option<NonNull<Frontend>>` 建模：null 哨兵（对应 C++ 默认构造的
// `ModuleResolver(nullptr)` 独立使用形态，恒不指向悬垂地址）在类型层显式化，
// 构造 → `Frontend::wire_self_pointers` 布线之间不存在「悬垂但看似有效」的
// 占位窗口；解引用集中于本方法一处。

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

impl FrontendModuleResolver {
  pub fn get_human_readable_module_name(&self, module_name: &ModuleName) -> String {
    // C++ `if (!frontend) return name;` 的 None 分支；解引用集中于
    // `frontend_ref` chokepoint。
    match self.frontend_ref() {
      Some(frontend) => frontend
        .file_resolver_ref()
        .get_human_readable_module_name(module_name),
      None => module_name.to_string(),
    }
  }
}

impl FrontendModuleResolver {
  /// C++ `FrontendModuleResolver::getModule`：模块不存在时返回 nullptr，映射为 `None`。
  pub fn try_get_module(&self, module_name: &ModuleName) -> Option<ModulePtr> {
    // parking_lot 互斥量无投毒语义，与 cpp `std::lock_guard` 完全一致。
    let _lock = self.module_mutex.lock();
    self.modules.get(module_name).cloned()
  }

  /// 同 cpp `getModule` 语义，但模块不存在时 panic —— 仅用于 cpp 中把
  /// nullptr 升级为 InternalCompilerError/断言的调用点（getCheckResult 等）。
  pub fn get_module(&self, module_name: &ModuleName) -> ModulePtr {
    self
      .try_get_module(module_name)
      .unwrap_or_else(|| panic!("Frontend does not have module: {}", module_name))
  }
}

impl FrontendModuleResolver {
  pub fn module_exists(&self, module_name: &ModuleName) -> bool {
    // C++ `if (!frontend) return false;` 的 None 分支；解引用集中于
    // `frontend_ref` chokepoint。
    self
      .frontend_ref()
      .is_some_and(|frontend| frontend.source_nodes.contains_key(module_name))
  }
}

impl FrontendModuleResolver {
  pub fn resolve_module_info(
    &self,
    current_module_name: &ModuleName,
    path_expr: &AstExpr,
  ) -> Option<ModuleInfo> {
    // C++ `if (!frontend) return nullopt;` 的 None 分支；解引用集中于
    // `frontend_ref` chokepoint。
    let frontend = self.frontend_ref()?;
    let trace = frontend.require_trace.get(current_module_name)?;
    let key = from_ref(path_expr).cast_mut().as_ast_node();

    trace.exprs.find(&key).cloned()
  }
}

impl FrontendModuleResolver {
  /// C++ `FrontendModuleResolver::setModule` (`Analysis/src/Frontend.cpp:1970`):
  /// inserts/replaces under the module mutex, returning whether a prior entry
  /// was replaced.
  pub fn set_module(&mut self, module_name: &ModuleName, module: ModulePtr) -> bool {
    // parking_lot 互斥量无投毒语义，与 cpp `std::lock_guard` 完全一致。
    let _lock = self.module_mutex.lock();

    let replaced = self.modules.contains_key(module_name);
    self.modules.insert(module_name.clone(), module);
    replaced
  }
}
