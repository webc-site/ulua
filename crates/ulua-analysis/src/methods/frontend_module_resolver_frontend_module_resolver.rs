//! C++ `FrontendModuleResolver::FrontendModuleResolver(Frontend* frontend)`
//! (`Analysis/src/Frontend.cpp:1922`): stores the owning frontend; `modules`
//! and `moduleMutex` are default-initialized.
use core::ptr::NonNull;

use parking_lot::Mutex;

use crate::{
  records::{frontend::Frontend, frontend_module_resolver::FrontendModuleResolver},
  type_aliases::collections::HashMap,
};

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
