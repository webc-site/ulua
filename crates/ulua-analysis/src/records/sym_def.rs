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
    self.sym == other.sym && self.version == other.version
  }
}

impl Eq for SymDef {}

impl Hash for SymDef {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.sym.hash(state);
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
}

// Safety: 内嵌 `Symbol` 的 `local: *mut AstLocal` 令自动 Send 失效。该指针仅作身份比较
// （eq/hash）使用、从不解引用，`global` 的 `AstName` 指向全局字符串驻留表；转移裸指针
// 身份值（其本身）到其它线程不破坏任何不变量，故 SymDef 满足 Send。
unsafe impl Send for SymDef {}
// Safety: 同上——`local` 裸指针与驻留 `AstName` 均跨线程只读共享、从不解引用写回，
// `version` 为普通 usize，共享 `&SymDef` 不产生数据竞争，故 SymDef 满足 Sync。
unsafe impl Sync for SymDef {}
