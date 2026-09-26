use core::ptr::from_ref;

use ulua_analysis::records::control_flow_graph::ControlFlowGraph;

use crate::records::cfg_fixture::CfgFixture;

impl CfgFixture {
  /// 构建 CFG：结果指针存入 `self.cfg_ptr`，经 [`CfgFixture::cfg`] 以借用形式取回。
  /// 拆成「构建（`&mut self`）+ 只读访问（`&self`）」两步，使测试在持有 CFG
  /// 引用的同时仍能调用 fixture 的 `&self` 查询方法（如
  /// `get_definition_at_pos`），全链路免 `unsafe { &*ptr }` 解引用。
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

    // Safety: cfg 指向 self.cfg_allocator 中存活的 ControlFlowGraph。
    let cfg = unsafe { CfgBuilder::make_cfg(&mut self.cfg_allocator as *mut _, self.root) };

    if fflag::DebugLuauLogCFG.get() {
      print!(
        "{}",
        dump_cfg(unsafe {
          // Safety: cfg 指向行 23 make_cfg 刚在 self.cfg_allocator arena 中构造的 ControlFlowGraph（块地址随 allocator 稳定、make_cfg 按契约非空），&* 物化临时只读借用交 dump_cfg 打印，借用随 print 结束。
          &*cfg
        })
      );
    }

    if fflag::DebugLuauDumpCFGJson.get() {
      println!(
        "{}",
        dump_cfg_json(unsafe {
          // Safety: 同 dump_cfg 处论证：cfg 指向 cfg_allocator arena 内存活 ControlFlowGraph（地址不动），&* 只读借用仅供 dump_cfg_json 序列化，帧内结束，单线程无并发写。
          &*cfg
        })
      );
    }

    self.cfg_ptr = cfg;
  }

  /// CFG 只读访问器：ControlFlowGraph 存活于 `self.cfg_allocator` 的 arena 内存
  /// （`build` 布线）；`&self` 借用期内 fixture 不可能再 `build`，引用不悬垂。
  pub fn cfg(&self) -> &ControlFlowGraph {
    assert!(!self.cfg_ptr.is_null(), "cfg() called before build()");
    // Safety: 见上——build 保证 cfg 指向存活 ControlFlowGraph。
    unsafe { &*self.cfg_ptr }
  }
}
