//! Source: `tests/ControlFlowGraph.test.cpp`

use core::ptr::null_mut;

use ulua_analysis::records::{cfg_allocator::CfgAllocator, control_flow_graph::ControlFlowGraph};
use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, ast_stat_block::AstStatBlock,
};
use ulua_common::fflag;

use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
#[derive(Debug)]
pub struct CfgFixture {
  pub allocator: Allocator,
  pub names: AstNameTable,
  pub cfg_allocator: CfgAllocator,
  /// cpp `CfgFixture` 的 `root` 成员（`root = parse(code)`，`ControlFlowGraph.test.cpp`）。
  ///
  /// DELIBERATE DEVIATION / 保留理由（review.md §2 缺席态的暂缓项）：目标形态是
  /// `Option<NonNull<AstStatBlock>>`（`None` = `build()` 之前无根块）。实测把本字段、
  /// `cfg_ptr` 与 `DataFlowGraphFixture::module` 一起换型后，`cargo check -p
  /// ulua-unit-test --all-targets` 的错误落在 **9 个非本轮清单文件 / 17 处**：
  /// `methods::cfg_fixture_build`（5：`self.root = from_ref(self.parse(code))
  /// .cast_mut()` 写入、`make_cfg` 入参、`cfg()` 的 `is_null` 断言 + `&*self.cfg_ptr`）、
  /// `methods::cfg_fixture_get_definition_at_pos`（2：`assert!(!self.root.is_null())` +
  /// `&*self.root`）、`methods::data_flow_graph_fixture_{dfg,get_def,get_local_def}`
  /// （4）、以及跨 crate 的签名面 `functions::query`（2，`AstNodePtr` 只为 `*mut T`
  /// 实现）、`ulua_ast::rtti`（2）、`ulua_analysis::methods::cfg_builder`
  /// （`CfgBuilder::make_cfg(*mut CfgAllocator, *mut AstStatBlock) ->
  /// *mut ControlFlowGraph`）、`ulua_analysis::methods::data_flow_graph_builder`
  /// （`DataFlowGraphBuilder::build` 第一参）。这些文件本轮禁改，只动声明端会留下
  /// 不可编译的半截迁移；句柄化需与 `make_cfg`/`build`/`AstNodePtr` 同批改签名。
  pub root: *mut AstStatBlock,
  /// `build` 布线的 CFG 指针（指向 `cfg_allocator` arena），经 `CfgFixture::cfg`
  /// 安全读取；与 `root` 同理需 `Debug` 可打印的裸指针形态。
  /// 保留理由同 `root`：唯一写入点与唯一判空点都在 `methods::cfg_fixture_build`
  /// （非本轮清单），换 `Option<NonNull<ControlFlowGraph>>` 需与之同批改
  /// （目标写法：`self.cfg_ptr = NonNull::new(cfg)`；`cfg()` 用
  /// `self.cfg_ptr.expect("cfg() called before build()").as_ref()`，`Safety` 前提
  /// 不变），否则只是把断言从 `is_null()` 换成 `is_none()`。
  pub cfg_ptr: *mut ControlFlowGraph,
  pub freeze_arena: ScopedFastFlag,
}

impl Default for CfgFixture {
  fn default() -> Self {
    let mut allocator = Allocator::new();
    let names = AstNameTable::new(&mut allocator);

    Self {
      allocator,
      names,
      cfg_allocator: CfgAllocator::default(),
      // 两个 arena 句柄槽位的「尚未 build」形态：`None` 语义在本轮无法用
      // Option<NonNull<_>> 表达（消费端与下游签名越出清单，理由见字段文档），
      // 故沿用 cpp `CfgFixture` 的 nullptr 成员初值。
      root: null_mut(),
      cfg_ptr: null_mut(),
      freeze_arena: ScopedFastFlag::new(&fflag::DebugLuauFreezeArena, true),
    }
  }
}

impl CfgFixture {
  pub fn new() -> Self {
    Self::default()
  }
}
