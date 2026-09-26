use alloc::vec::Vec;

use crate::records::{
  node::{Block, Stat},
  reducer::Reducer,
};

impl Reducer {
  /// cpp `Reducer::tryPromotingChildStatements(b, index)`：把 `b` body 第
  /// `index` 条语句原位换成它内部嵌套的语句（晋升），试跑经 `try_body`
  /// 提交/回滚。`index` 是 splice 位置数据，保留下标形态。
  fn promote_at(&mut self, b: &mut Block, index: usize) -> bool {
    let mut temp_stats: Vec<Stat> = b.get().body.iter().map(Stat::from_ref).collect();

    // 取出第 index 条语句，原位换入其内部嵌套的语句
    let removed = temp_stats.remove(index);
    let nested_stats = self.get_nested_stats(removed);
    temp_stats.splice(index..index, nested_stats);

    self.try_body(b, &temp_stats)
  }
}

impl Reducer {
  /// cpp 重载 `tryPromotingChildStatements(AstStatBlock*)`：自前往后逐条尝试
  /// 晋升。晋升提交后 body 变长，新进来的语句还要继续尝试（cpp 每轮重读
  /// `b->body.size` 的语义在此保留）。cpp 恒返回 false（原地推进由外层
  /// `walk` 的 do-while 承担），行为一致。
  pub(crate) fn try_promoting_child_statements(&mut self, b: &mut Block) -> bool {
    let mut i: usize = 0;
    while i < b.get().body.len() {
      if !self.promote_at(b, i) {
        i += 1;
      }
    }

    false
  }
}
