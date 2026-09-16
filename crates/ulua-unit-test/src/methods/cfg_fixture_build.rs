use ulua_analysis::records::control_flow_graph::ControlFlowGraph;

use crate::records::cfg_fixture::CfgFixture;
impl CfgFixture {
  pub fn build(&mut self, code: &str) -> *mut ControlFlowGraph {
    use ulua_analysis::{
      functions::{dump_cfg::dump_cfg, dump_cfg_json::dump_cfg_json},
      records::cfg_builder::CfgBuilder,
    };
    use ulua_common::FFlag;

    self.root = self.parse(code);

    let cfg = unsafe { CfgBuilder::make_cfg(&mut self.cfg_allocator as *mut _, self.root) };

    if FFlag::DebugLuauLogCFG.get() {
      print!("{}", dump_cfg(unsafe { &*cfg }));
    }

    if FFlag::DebugLuauDumpCFGJson.get() {
      println!("{}", dump_cfg_json(unsafe { &*cfg }));
    }

    cfg
  }
}
