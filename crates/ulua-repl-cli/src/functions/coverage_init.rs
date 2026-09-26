use alloc::vec::Vec;
use core::cell::RefCell;
use std::thread_local;

use ulua_ast::functions::optional_node::node_opt;
use ulua_vm::{functions::lua_mainthread::lua_mainthread, records::lua_state::LuaState};

use crate::records::coverage::Coverage;

// 对应 cpp `Coverage.cpp` 的文件静态量 `static Coverage gCoverage`。
//
// 全部访问路径（`coverageInit` / `coverageActive` / `coverageTrack` /
// `coverageDump`）只在驱动 VM 的 REPL 主线程上运行，因此改用 thread_local +
// `RefCell` 即可保持行为一致，同时消除 C 风格可变全局。
thread_local! {
  pub(crate) static G_COVERAGE: RefCell<Coverage> = const {
    RefCell::new(Coverage {
      l: None,
      functions: Vec::new(),
    })
  };
}

pub(crate) fn coverage_init(l: *mut LuaState) {
  // Safety: `l` 指向存活的 `LuaState`（repl_main 以守卫持有 VM 状态）。
  let main_thread = unsafe { lua_mainthread(l) };
  G_COVERAGE.with(|coverage| {
    coverage.borrow_mut().l = node_opt(main_thread);
  });
}
