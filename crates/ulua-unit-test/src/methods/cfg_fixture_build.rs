use ulua_analysis::records::control_flow_graph::ControlFlowGraph;
use ulua_ast::records::ast_stat_block::AstStatBlock;

use crate::records::cfg_fixture::CfgFixture;
impl CfgFixture {
  pub fn build(&mut self, code: &str) -> *mut ControlFlowGraph {
    use ulua_analysis::{
      functions::{dump_cfg::dump_cfg, dump_cfg_json::dump_cfg_json},
      records::cfg_builder::CfgBuilder,
    };
    use ulua_common::fflag;

    // `parse` 返回借用引用；存入 `*mut` 字段（借用立即结束，AST 在
    // fixture.allocator 中存活至 `make_cfg` 使用结束）。
    self.root = self.parse(code) as *const AstStatBlock as *mut AstStatBlock;

    // SAFETY: cfg 指向 self.cfg_allocator 中存活的 ControlFlowGraph；返回引用
    // 生命周期绑定到 `&'a self`，借用期内 allocator 稳定。
    let cfg = unsafe { CfgBuilder::make_cfg(&mut self.cfg_allocator as *mut _, self.root) };

    if fflag::DebugLuauLogCFG.get() {
      print!("{}", dump_cfg(unsafe { &*cfg }));
    }

    if fflag::DebugLuauDumpCFGJson.get() {
      println!("{}", dump_cfg_json(unsafe { &*cfg }));
    }

    cfg
  }
}
