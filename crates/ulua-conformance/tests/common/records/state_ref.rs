use core::ptr::NonNull;

use ulua_vm::{functions::lua_close::lua_close, records::lua_state::LuaState};

/// 对 `LuaState*` 的 RAII 包装，作用域结束时自动 `lua_close`。
/// 对应 cpp 的 `std::unique_ptr<LuaState, void (*)(LuaState*)>`。
///
/// 内部指针刻意私有：句柄的唯一来源是 [`StateRef::new`]，字段一旦 `pub`，用例就能对
/// 同一个 `LuaState*` 再包一层，两次 `Drop` 即两次 `lua_close`（double free）。
#[repr(transparent)]
pub struct StateRef(NonNull<LuaState>);

impl StateRef {
  /// 接管一个由 `lua_newstate`/`luaL_newstate` 创建的状态；传入空指针返回 `None`，
  /// 非空即视为调用方把所有权交给本句柄（cpp `unique_ptr` 的构造语义）。
  pub fn new(state: *mut LuaState) -> Option<Self> {
    NonNull::new(state).map(Self)
  }

  /// 借用裸指针，供 C API 调用使用；不转移所有权。
  pub fn as_ptr(&self) -> *mut LuaState {
    self.0.as_ptr()
  }
}

impl Drop for StateRef {
  fn drop(&mut self) {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`self` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe {
      lua_close(self.as_ptr());
    }
  }
}
