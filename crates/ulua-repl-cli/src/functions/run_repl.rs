use ulua_vm::{
  functions::{lua_l_newstate::lua_l_newstate, lua_l_sandboxthread::lua_l_sandboxthread},
  records::lua_state_guard::LuaStateGuard,
};

use crate::functions::{run_repl_impl::run_repl_impl, setup_state::setup_state, sigint_setup};

/// 交互式 REPL 进程入口：新建状态、初始化、挂载 Ctrl-C、沙箱化线程并运行交互循环。
///
/// 调用契约（本 fn 为 `pub(crate)` 安全 fn，crate 内唯一调用方 repl_main 保证）：仅在
/// 单线程 REPL 模式调用——本函数改写进程级信号处理并管理一个 Lua VM 状态，进程内不得
/// 有其它 REPL/VM 驱动者。
// DELIBERATE DEVIATION（review.md §9.3）：REPL 进程入口驱动一个 `*mut LuaState` 全
// 生命周期（newstate→setup_state→install 信号→sandboxthread→run_repl_impl→withdraw），
// 状态句柄由 LuaStateGuard 持有、本帧只别名传递（信号 AtomicPtr 与 ReplHelper 两个长期
// 登记点只能收裸句柄，故此处不提前折成长寿借用）；解引用收在各消费点带 `// Safety:` 的
// 最小 `unsafe` 块内，故本编排入口自身收编为安全 fn、原 `unsafe fn` 契约降级为文档约定。
// Faithful port of `runRepl` from Repl.cpp: create a fresh state, set it up,
// arm Ctrl-C handling, sandbox the thread and run the interactive loop.
pub(crate) fn run_repl() {
  // lua_l_newstate 是安全封装（OOM 返回 null 的边界与 cpp 相同，下方各消费点
  // 契约同样覆盖）；守卫在作用域退出时 lua_close（panic/unwind 同样生效）。
  // cpp Repl.cpp:553 `unique_ptr<lua_State, void (*)(lua_State*)>`
  // 返回的 C-ABI 状态句柄由守卫持有，本帧只做别名传递，不做任何算术。
  let global_state = LuaStateGuard(lua_l_newstate());
  let l = global_state.0;

  // Safety: l 为上一行 lua_l_newstate 交出、由本帧守卫持有至函数末尾的新状态；仅进程级
  // OOM 才为 null，此时行为与 cpp oracle（unique_ptr(lua_newstate()) 同样不判空）一致。
  // REPL 进程入口单线程驱动，下面每次 `&mut *l` 物化的借用窗都止于当句调用、窗内无并存
  // 可变别名；setup_state 的「刚创建、有效」前置即此契约。
  unsafe { setup_state(&mut *l) };

  // setup Ctrl+C handling: replState = l; signal(SIGINT, sigintHandler);
  // 状态发布与 libc handler 注册（含与信号上下文交换的 null 协议值）收在
  // sigint_setup 门面里，这里只保留调用时机与存活契约。
  // `install` 为安全 fn（仅原子存入裸句柄、不解引用），其调用序契约——l 为本帧
  // 新建、在整个交互式循环期间存活且单线程驱动——由本入口保证；循环结束后由 withdraw 先摘状态再 close。
  sigint_setup::install(l);

  // 冻结线程全局表：`lua_l_sandboxthread` 为 ulua-vm 安全引用形，在边界处即时物化
  // 本次调用窗的借用（review.md §2 收口），本点不再长寿别名 `l`。
  // Safety: 同上契约，借用窗止于当句。
  unsafe { lua_l_sandboxthread(&mut *l) };
  // run_repl_impl 现为 crate 内安全编排 fn；裸句柄交由其内部的 ReplHelper 长期登记
  // （rustyline 回调只能以 `&self` 触发），l 在整个交互式循环期间有效且单线程驱动
  // （其文档契约）由本入口守卫保证。
  run_repl_impl(l);

  // cpp 只在 unique_ptr 析构处关闭；这里先把全局信号处理引用的状态摘掉，
  // 再由守卫在作用域退出时 lua_close（panic/unwind 路径同样生效）。
  sigint_setup::withdraw();
}
