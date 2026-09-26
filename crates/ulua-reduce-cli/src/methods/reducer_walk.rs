use std::collections::VecDeque;

use ulua_ast::visit::ast_stat_visit_ref;

use crate::records::{enqueuer::Enqueuer, node::Block, reducer::Reducer};

impl Reducer {
  /// cpp `Reducer::walk`：BFS 队列逐块跑全部归约（删语句 + 子语句晋升），
  /// 收敛后把子块经 `Enqueuer` 入队继续。
  pub(crate) fn walk(&mut self, block: Block) {
    let mut queue: VecDeque<Block> = VecDeque::new();

    queue.push_back(block);

    while let Some(mut b) = queue.pop_front() {
      loop {
        let mut result = self.delete_child_statements(&mut b);
        result |= self.try_promoting_child_statements(&mut b);

        if !result {
          break;
        }
      }

      let mut enqueuer = Enqueuer { queue: &mut queue };
      for stat in b.get_mut().body.iter_nodes_mut() {
        // `iter_nodes_mut` 沿 `&mut Block` 传独占；visitor `Enqueuer` 只记录
        // 子块句柄、不写节点，等价 cpp `astStatVisit(stat, &enqueuer)`。
        ast_stat_visit_ref(stat.get_mut(), &mut enqueuer);
      }
    }
  }
}
