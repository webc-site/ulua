use alloc::vec::Vec;
use core::cell::RefCell;
use std::thread_local;

use ulua_ast::functions::optional_node::node_opt;
use ulua_vm::{functions::lua_mainthread::lua_mainthread, records::lua_state::LuaState};

use crate::records::counters::Counters;

// 对应 cpp `Counters.cpp` 的文件静态量 `static Counters gCounters`。
//
// 全部访问路径（`countersInit` / `countersActive` / `countersTrack` /
// `countersDump`）只在驱动 VM 的 REPL 主线程上运行，因此改用 thread_local +
// `RefCell` 即可保持行为一致，同时消除 C 风格可变全局。
thread_local! {
  pub(crate) static G_COUNTERS: RefCell<Counters> = const {
    RefCell::new(Counters {
      l: None,
      module_refs: Vec::new(),
      module_counters: Vec::new(),
    })
  };
}

/// 对应 cpp `Counters.cpp` 的 `void countersInit(LuaState* L)`。
/// review.md §2/§3 收形：`l` 由裸 `*mut LuaState` 收编为借用 `&mut LuaState`（真实
/// 物化点在 repl_main 入口的守卫句柄一次），`lua_mainthread` 系安全引用形，本函数体
/// 内不再有 `unsafe` 面；其回溯同 VM 主线程的调用序契约降级为文档约定。
pub(crate) fn counters_init(l: &mut LuaState) {
  let main_thread = lua_mainthread(l);
  G_COUNTERS.with(|counters| {
    counters.borrow_mut().l = node_opt(main_thread);
  });
}
