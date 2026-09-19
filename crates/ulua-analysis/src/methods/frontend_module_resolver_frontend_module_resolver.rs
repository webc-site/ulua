//! C++ `FrontendModuleResolver::FrontendModuleResolver(Frontend* frontend)`
//! (`Analysis/src/Frontend.cpp:1922`): stores the owning frontend; `modules`
//! and `moduleMutex` are default-initialized.
use std::{collections::HashMap, sync::Mutex};

use crate::records::{frontend::Frontend, frontend_module_resolver::FrontendModuleResolver};

impl FrontendModuleResolver {
  pub fn new(frontend: *mut Frontend) -> Self {
    Self {
      frontend,
      module_mutex: Mutex::new(()),
      modules: HashMap::new(),
    }
  }
}
