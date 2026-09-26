//! Source: `Analysis/src/ControlFlowGraph.cpp:135-143` (hand-ported)
//! C++ `std::unique_ptr<ControlFlowGraph> CFGBuilder::makeCFG(NotNull<CFGAllocator> allocator, AstStatBlock* block)`.
use alloc::boxed::Box;

use ulua_ast::records::ast_stat_block::AstStatBlock;
use ulua_common::fflag;

use crate::records::{
  cfg_allocator::CfgAllocator, cfg_builder::CfgBuilder, control_flow_graph::ControlFlowGraph,
};
impl CfgBuilder {
  /// # Safety
  /// 对应 C++ `CFGBuilder::makeCFG(NotNull<CFGAllocator> allocator, AstStatBlock* block)`：
  /// `allocator` 须非空且在返回的 `ControlFlowGraph` 整个使用期内存活（CFG 只存裸句柄，
  /// 不接管其所有权）；`block` 须指向存活非空的 `AstStatBlock`（Module 的 AST arena 节点）。
  /// 返回 `Box::into_raw` 产出的裸所有权指针，由调用方负责唯一释放。
  pub unsafe fn make_cfg(
    allocator: *mut CfgAllocator,
    block: *mut AstStatBlock,
  ) -> *mut ControlFlowGraph {
    // C++:
    //   CFGBuilder builder(allocator);
    //   builder.lower(block);
    let mut builder = CfgBuilder::new(allocator);
    // `block` is `AstStatBlock*`; C++ `lower(block)` dispatches to the
    // `AstStatBlock*` overload.
    // Safety: `allocator`/`block` 的合法性即上面的函数级契约（C++ 侧 `NotNull<CFGAllocator>`
    // 与 module->root 的非空 AST 根）。此处一次 `&*` 把 arena 根块换成存活引用
    // （`block` 指向 parse 阶段在 AST arena 内分配的 AstStatBlock，块地址不移动，
    // 遍历在 builder 生命周期内完成）；builder 是本函数栈上唯一持有者，
    // 对同一 allocator 的可变访问串行发生。
    builder.lower_ast_stat_block(unsafe { &*block });

    // auto cfg = std::move(builder.cfg);
    // 不变式：`CfgBuilder::new` 构造期即置入 cfg（对应 C++ 构造里
    // `cfg(allocator->acquireCFG())`），lowering 全程只经 `as_mut` 借用、
    // 从不置 None，故 take() 必为 Some。
    let cfg = builder
      .cfg
      .take()
      .expect("CfgBuilder 构造期置入 cfg，lowering 不清空，take 必 Some");

    // if (FFlag::DebugLuauFreezeArena) allocator->freeze();
    if fflag::DebugLuauFreezeArena.get() {
      // Safety: `allocator` 按契约非空且存活；`cfg` 已由 `take()` 移出并只保存该指针的
      // 裸句柄，此刻 `&mut builder`（其 `allocator` 字段指向同一对象）已结束使用，
      // 单线程内不存在第二个可变借用，故以 `(*allocator).freeze()` 冻结是独占写。
      // 冻结后 TypedAllocator 只拒绝新的分配，已发出的节点地址依旧有效（bump 块不搬家）。
      unsafe { (*allocator).freeze() };
    }

    // return cfg;  (unique_ptr -> raw owning pointer)
    Box::into_raw(Box::new(cfg))
  }
}
