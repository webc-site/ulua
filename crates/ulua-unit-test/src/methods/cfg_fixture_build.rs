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

    // cpp `root = parse(code)`：`parse` 返回借用引用，经 `Handle::from_ref`
    // 折叠为 arena 别名句柄入位 `self.root`（AST 由 `allocator` 字段保活、
    // bump 块地址稳定，句柄不拥有不释放），借用随语句结束。
    let root = Handle::from_ref(self.parse(code));
    self.root = Some(root);

    // `make_cfg` 入口全 safe：arena 侧经 `Handle::from_mut` 编码非空与存活
    // （借用止于调用表达式，cfg_allocator 作为夹具字段比返回的 CFG 长寿），
    // block 侧由 `root.get()` 直接物化只读借用（Handle 模块契约）。
    let cfg = CfgBuilder::make_cfg(Handle::from_mut(&mut self.cfg_allocator), root.get());

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
