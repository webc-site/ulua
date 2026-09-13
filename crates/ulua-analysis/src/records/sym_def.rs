use alloc::string::String;
use core::hash::{Hash, Hasher};

use crate::records::symbol::Symbol;
#[derive(Debug, Clone)]
pub struct SymDef {
  pub sym: Symbol,
  pub version: usize,
}

impl PartialEq for SymDef {
  fn eq(&self, other: &Self) -> bool {
    self.sym.operator_eq_symbol(&other.sym) && self.version == other.version
  }
}

impl Eq for SymDef {}

impl Hash for SymDef {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.sym.hash_luau_symbol_operator_call().hash(state);
    self.version.hash(state);
  }
}

impl SymDef {
  pub fn new(sym: Symbol, version: usize) -> Self {
    Self { sym, version }
  }

  pub fn name(&self) -> &str {
    self.sym.name()
  }

  pub fn versioned_name(&self) -> String {
    format!("{}-{}", self.name(), self.version)
  }

  pub fn operator_eq_sym_def(&self, other: &Self) -> bool {
    self.sym.operator_eq_symbol(&other.sym) && self.version == other.version
  }
}

unsafe impl Send for SymDef {}
unsafe impl Sync for SymDef {}
