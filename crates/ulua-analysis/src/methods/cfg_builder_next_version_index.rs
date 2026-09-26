use crate::records::{cfg_builder::CfgBuilder, symbol::Symbol};

impl CfgBuilder {
  pub fn next_version_index(&mut self, sym: Symbol) -> usize {
    if !self.version_counter.contains(&sym) {
      *self.version_counter.get_or_insert(sym) = 0;
      return 0;
    }

    // 上方 `!contains` 分支已早退，此处 sym 恒已登记，find_mut 命中 Some。
    let ref_mut = self
      .version_counter
      .find_mut(&sym)
      .expect("上方 !contains 早退蕴含 sym 已登记");
    *ref_mut += 1;
    *ref_mut
  }
}
