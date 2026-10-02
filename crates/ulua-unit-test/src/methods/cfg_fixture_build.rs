use core::ptr::from_ref;

use ulua_analysis::records::{arena_handle::Handle, control_flow_graph::ControlFlowGraph};

use crate::records::cfg_fixture::CfgFixture;

impl CfgFixture {
  /// 构建 CFG：所有权结果（`make_cfg` 按值交付，cpp `unique_ptr` 返回的 Rust
  /// 对应）存入 `self.cfg`，经 [`CfgFixture::cfg`] 以借用形式取回。
  /// 拆成「构建（`&mut self`）+ 只读访问（`&self`）」两步，使测试在持有 CFG
  /// 引用的同时仍能调用 fixture 的 `&self` 查询方法（如
  /// `get_definition_at_pos`），全链路免解引用样板。
  pub fn build(&mut self, code: &str) {
    use ulua_analysis::{
      functions::{dump_cfg::dump_cfg, dump_cfg_json::dump_cfg_json},
      records::cfg_builder::CfgBuilder,
    };
    use ulua_common::fflag;

    // `parse` 返回借用引用；引用 → 裸地址用 `from_ref + cast_mut`（存入 `*mut`
    // 字段后借用立即结束，AST 在 fixture.allocator 中存活至 `make_cfg` 使用
    // 结束），免 `as *const _ as *mut _` 双重 `as` 反模式。
    self.root = from_ref(self.parse(code)).cast_mut();

    // `make_cfg` 入口已全 safe：arena 侧经 `Handle::from_mut` 编码非空与存活
    // （借用止于调用表达式，cfg_allocator 作为夹具字段比返回的 CFG 长寿），
    // block 侧仍是 `root` 裸字段的首层解引用（句柄化在下一步收敛）。
    // Safety: root 由上一行从 `parse` 返回引用写入，指向 fixture allocator
    // arena 内活 AstStatBlock，借用半径止于本次调用表达式。
    let cfg = CfgBuilder::make_cfg(Handle::from_mut(&mut self.cfg_allocator), unsafe {
      &*self.root
    });

    if fflag::DebugLuauLogCFG.get() {
      print!("{}", dump_cfg(&cfg));
    }

    if fflag::DebugLuauDumpCFGJson.get() {
      println!("{}", dump_cfg_json(&cfg));
    }

    // 所有权显式入位（§2 转手动作）：CFG 容器由夹具持有，节点内存留在 arena。
    self.cfg = Some(cfg);
  }

  /// CFG 只读访问器：`build` 后经所有权字段取回；`&self` 借用期内夹具不可能
  /// 再 `build`/移动，引用不悬垂。
  pub fn cfg(&self) -> &ControlFlowGraph {
    self.cfg.as_ref().expect("cfg() called before build()")
  }
}
