use alloc::vec::Vec;

use crate::records::lint_local_hygiene::LintLocalHygiene;
impl LintLocalHygiene {
  pub fn report(&mut self) {
    let locals = self
      .locals
      .iter()
      .map(|(local, info)| (*local, info.clone()))
      .collect::<Vec<_>>();

    for (local, info) in locals {
      if info.used {
        unsafe { self.report_used_local(local, &info) };
      } else if !info.defined.is_null() {
        unsafe { self.report_unused_local(local, &info) };
      }
    }
  }
}
