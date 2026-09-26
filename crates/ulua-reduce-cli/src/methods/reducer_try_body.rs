use ulua_ast::records::node_handle::{Node as ArenaNode, Nodes};

use crate::{
  enums::test_result::TestResult,
  records::{
    node::{Block, Stat},
    reducer::Reducer,
  },
};

impl Reducer {
  /// 全 crate 唯一的 AST 节点写入边界：把 `block` 的 body 临时换成
  /// `statements`，跑一次用例验证（`run` → `write_temp_script` 读整棵树，
  /// 本处对 `block` 的借用窗口已收敛，不存在并存借用）；若 bug 仍可复现则
  /// 提交，否则回滚为原 body。
  ///
  /// cpp 在 `deleteChildStatements` / `tryPromotingChildStatements` 里各自
  /// 手写的 `std::swap` 试提交/回滚两步收敛于此，语义完全一致。cpp 的
  /// 「试跑视图借调用方 Vec + 提交时 `reallocateStatements` 拷入 arena」两
  /// 步在 `Nodes`（堆拥有句柄数组）模型下合一：试跑数组本就直接接管句柄、
  /// 独立于调用方 Vec 存活，提交即不拷、回滚即弃。
  pub(crate) fn try_body(&mut self, block: &mut Block, statements: &[Stat]) -> bool {
    // `Nodes` 克隆只复制句柄数组（元素为坐标值），记录原 body 供回滚。
    let backup = block.get().body.clone();

    // 语句句柄桥接为目标树的 `Nodes`：`Stat` 出自 arena 存活节点（模块契约），
    // `get` 给出引用即非空 + 存活证明，全程无裸指针。
    block.get_mut().body = Nodes::from_vec(
      statements
        .iter()
        .map(|stat| ArenaNode::from_ref(stat.get()))
        .collect(),
    );

    if self.run() != TestResult::BugFound {
      // 被删语句对复现 bug 是关键：回滚原 body，试跑数组随之释放。
      block.get_mut().body = backup;
      return false;
    }

    true
  }
}
