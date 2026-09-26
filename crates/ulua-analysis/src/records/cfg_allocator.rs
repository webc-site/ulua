//! Source: `Analysis/include/Luau/ControlFlowGraph.h:233` (hand-ported)
//! C++ `struct CFGAllocator`.
use crate::{
  records::{
    block::Block, refinement_arena_control_flow_graph::RefinementArena, sym_def::SymDef,
    typed_allocator::TypedAllocator,
  },
  type_aliases::instruction::Instruction,
};

#[derive(Debug)]
pub struct CfgAllocator {
  // public:
  pub refinement_arena: RefinementArena,
  // private:
  pub(crate) block: TypedAllocator<Block>,
  pub(crate) instructions: TypedAllocator<Instruction>,
  pub(crate) defs: TypedAllocator<SymDef>,
  pub(crate) frozen: bool,
}

impl Default for CfgAllocator {
  fn default() -> Self {
    Self {
      // `RefinementArena` holds a single `TypedAllocator<Refinement>`
      // (default-constructed). Built via struct literal (field is
      // `pub(crate)`, same crate) to avoid a Default impl on the arena.
      refinement_arena: RefinementArena {
        allocator: TypedAllocator::default(),
      },
      block: TypedAllocator::default(),
      instructions: TypedAllocator::default(),
      defs: TypedAllocator::default(),
      // C++ `bool frozen = false;`
      frozen: false,
    }
  }
}

/// # Safety
///
/// 三个 `TypedAllocator<*>` 字段内部为 `Vec<*mut T>` 堆块，令自动 Send 失效。
/// 调用方须保证本分配器独占这些块的唯一所有权（`frozen` 前仅由所有者追加节点），
/// 无其它持有者；满足时转移所有权到其它线程可靠。
// Safety: TypedAllocator 仅暴露 &mut/&self 方法（bump 块只经 self 追加/读取），
// 跨线程移动本结构即移动块的唯一所有权，块地址不移动、无隐藏共享指针，Send
// 契约在上述独占不变量下成立。
unsafe impl Send for CfgAllocator {}
/// # Safety
///
/// 同上所有权不变量：块只由本分配器独占，`frozen` 后仅读；共享 `&CfgAllocator`
/// 时不会与并发的 `*mut` 追加产生数据竞争。
// Safety: 见上——共享 &self 路径下块内容不再追加（frozen 后仅读），且类型上
// 无法经共享引用取得块的可变句柄，并发只读不构成数据竞争，Sync 契约成立。
unsafe impl Sync for CfgAllocator {}
