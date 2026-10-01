use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::rtti::{AstNodeClass, AstNodePtr, ast_node_try_as};

use crate::records::{find_nth_occurence_of::FindNthOccurenceOf, nth::Nth};

/// 沿 `nths` 路径逐层下钻，取第 n 个指定类型节点；路径断裂或判型不命中返回 null。
///
/// 出参保持 `*mut T` 而非 `Option<&T>`：本函数是 cpp 夹具 `query<T>` DSL 的对应物，
/// 返回值既可能继续作为下一层 `query` 的 `impl AstNodePtr` 入参，也会直接喂给
/// DFG/`DefId` 这类句柄形态的下游 API，引用会把这三类用法都逼回裸指针转换。
/// 节点本体活在夹具 arena（`Box`/arena 分配后地址不动），null 即「未命中」哨兵。
/// 下钻断裂返回 `null_mut()` 系指针值语义/arena 句柄字段既有约定（review.md §2）。
///
/// r11 独立复核（rg 交叉核对全部消费点后维持论证）：
/// - `tests/ast_query.rs`：`query` 输出直接喂下一层 `query`，且 cpp oracle 行为钉
///   要求「断裂的 null 继续喂入下一层仍得 null」——`Option<&T>` 无法表示空入参；
///   `AstNodePtr` 按 `*mut T`/`*const T` 实现（ulua-ast rtti），引用入参必逼回转指针。
/// - `src/methods/data_flow_graph_fixture_get_def.rs` / `..get_local_def.rs`：输出经
///   `cast_const` 直接喂 `ulua_analysis::DataFlowGraph::get_def(*const AstExpr)` 等
///   句柄形态下游（该签名为 crate 外消费面，不可在本 crate 内单方改掉）。
///
/// 三类用法逐点证实，改 `Option` 形态只会把裸指针转换下推到各消费点，违背 §2 收口方向。
pub fn query<T: AstNodeClass>(node: impl AstNodePtr, nths: Vec<Nth>) -> *mut T {
  let mut node = node.as_ast_node();
  for nth in nths {
    if node.is_null() {
      return null_mut();
    }

    let mut finder = FindNthOccurenceOf::new(nth);
    finder.visit_ast_node(node);

    node = finder.the_node;
  }

  // cpp `node->as<T>()` 同款 RTTI 判定下转：ast_node_try_as 门面判空与判型合一。
  // Safety: node 为 null 或指向存活 repr(C) AST 节点（夹具 arena）。
  unsafe {
    node
      .as_ref()
      .and_then(ast_node_try_as::<T>)
      .map_or(null_mut(), |p| p as *const T as *mut T)
  }
}
