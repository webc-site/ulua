//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Frontend.h:133:frontend_module_resolver`
//! Source: `Analysis/include/Luau/Frontend.h`
//!
//! C++ `struct FrontendModuleResolver : ModuleResolver` (Frontend.h:133-151).
//! The base `ModuleResolver` interface is pure-virtual; its overrides live as
//! `FrontendModuleResolver` methods. The data members are ported here.

use core::fmt::{Debug, Formatter, Result};
use std::{collections::HashMap, sync::Mutex};

use crate::{
  records::{frontend::Frontend, module_resolver::ModuleResolver},
  type_aliases::{module_name_type::ModuleName, module_ptr_module::ModulePtr},
};
#[repr(C)]
pub struct FrontendModuleResolver {
  pub base: ModuleResolver,

  /// `Frontend* frontend;`
  pub frontend: *mut Frontend,

  /// `mutable std::mutex moduleMutex;`
  pub module_mutex: Mutex<()>,

  /// `std::unordered_map<ModuleName, ModulePtr> modules;`
  pub modules: HashMap<ModuleName, ModulePtr>,
}

impl Debug for FrontendModuleResolver {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("FrontendModuleResolver")
      .field("modules", &self.modules)
      .finish_non_exhaustive()
  }
}
