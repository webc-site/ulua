use core::ptr::null_mut;

use ulua_analysis::records::{
  data_flow_graph::DataFlowGraph, def_arena::DefArena,
  internal_error_reporter::InternalErrorReporter, refinement_key_arena::RefinementKeyArena,
};
use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, ast_stat_block::AstStatBlock,
};
use ulua_common::fflag;

use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
#[derive(Debug)]
#[repr(C)]
pub struct DataFlowGraphFixture {
  pub dcr: ScopedFastFlag,
  pub def_arena: DefArena,
  pub key_arena: RefinementKeyArena,
  pub handle: InternalErrorReporter,
  pub allocator: Allocator,
  pub names: AstNameTable,
  /// cpp `DataFlowGraphFixture::module`（`DataFlowGraph.test.cpp` 的根块成员）。
  ///
  /// DELIBERATE DEVIATION / 保留理由（review.md §2 缺席态的暂缓项）：语义是「`dfg()`
  /// 之前无根块」，目标形态 `Option<NonNull<AstStatBlock>>`。实测与 `CfgFixture` 的
  /// 两个句柄槽一起换型后，`cargo check -p ulua-unit-test --all-targets` 的报错落在
  /// 9 个非本轮清单文件 / 17 处，其中本字段的消费端有 4 处：
  /// `methods::data_flow_graph_fixture_dfg`（`self.module = result.root` 写入 +
  /// 作为 `DataFlowGraphBuilder::build` 第一参）、
  /// `methods::data_flow_graph_fixture_get_def` 与 `_get_local_def`
  /// （`query::<T>(self.module, nths)`）。`functions::query` 的 `AstNodePtr`
  /// trait 目前只为裸指针实现（配套 `ulua_ast::rtti` 的下转），且
  /// `ulua_analysis::methods::data_flow_graph_builder::DataFlowGraphBuilder::build`
  /// 首参虽已引用化为 `&AstStatBlock`，本字段仍须以 `*mut` 喂 `query`——句柄化要
  /// 跨 crate 同批改，否则只留半截迁移。
  /// 现状本侧解引用仅 `dfg()` 内 build 入参处一次 `&*self.module`（arena 存活契约见该处注释）。
  pub module: *mut AstStatBlock,
  pub graph: Option<DataFlowGraph>,
}

impl Default for DataFlowGraphFixture {
  fn default() -> Self {
    let mut allocator = Allocator::new();
    let names = AstNameTable::new(&mut allocator);

    Self {
      dcr: ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
      def_arena: DefArena::default(),
      key_arena: RefinementKeyArena::default(),
      handle: InternalErrorReporter::default(),
      allocator,
      names,
      module: null_mut(),
      graph: None,
    }
  }
}

impl DataFlowGraphFixture {
  pub fn new() -> Self {
    Self::default()
  }
}
