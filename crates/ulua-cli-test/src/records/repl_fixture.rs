//! cpp `tests/Repl.test.cpp` 的 `class ReplFixture`（`RequireByString.test.cpp`
//! 的 `ReplWithPathFixture` 是它的别名，见 [`crate::records::repl_with_path_fixture`]）。

use ulua_vm::records::{lua_state::LuaState, lua_state_guard::LuaStateGuard};

use crate::functions::new_fixture_state::new_fixture_state;

/// cpp 两个 fixture 逐行相同的 `prettyPrintSource`：简化版 pretty printer，
/// 把输出累积到全局 `capturedoutput` 以便断言。
const PRETTY_PRINT_SOURCE: &str = r#"
-- Accumulate pretty printer output in `capturedoutput`
capturedoutput = ""

function arraytostring(arr)
    local strings = {}
    table.foreachi(arr, function(k,v) table.insert(strings, pptostring(v)) end )
    return "{" .. table.concat(strings, ", ") .. "}"
end

function pptostring(x)
    if type(x) == "table" then
        -- Just assume array-like tables for now.
        return arraytostring(x)
    elseif type(x) == "string" then
        return '"' .. x .. '"'
    else
        return tostring(x)
    end
end

-- Note: Instead of calling print, the pretty printer just stores the output
-- in `capturedoutput` so we can check for the correct results.
function _PRETTYPRINT(...)
    local args = table.pack(...)
    local strings = {}
    for i=1, args.n do
        local item = args[i]
        local str = pptostring(item, customoptions)
        if i == 1 then
            capturedoutput = capturedoutput .. str
        else
            capturedoutput = capturedoutput .. "\t" .. str
        end
    end
end
"#;

/// 持有 `LuaState` 生命周期：cpp 用 `unique_ptr<LuaState, void(*)(LuaState*)>`
/// 成员 `luaState` 关闭状态、裸成员 `L`（`luaState.get()`）供各方法使用。
/// Rust 侧同一职责交给共用的 [`LuaStateGuard`]（`Drop` 里 `lua_close`），
/// 且只保留守卫这一份指针：把 `L` 也存成字段等于让同一状态有两个可变句柄，
/// 借用检查器就无法把「写 VM」的调用和读输出的调用排他，故收为 [`Self::l`]。
#[derive(Debug)]
pub struct ReplFixture {
  /// cpp `ReplFixture::luaState`
  state: LuaStateGuard,
}

impl ReplFixture {
  /// 对齐 cpp 构造函数：`luaL_newstate` → `setupState` → `luaL_sandboxthread`
  /// → `runCode(prettyPrintSource)`。
  ///
  /// 搭建失败（内存耗尽、pretty printer 报错）时带原因显式失败：夹具没有可用的
  /// `LuaState`，继续跑只会在无关断言处以更难定位的方式炸掉（cpp 侧由 doctest 的
  /// 全局前置同样表现为该用例失败）。
  pub fn new() -> Self {
    // 守卫由 `new_fixture_state` 就地建立并移交；`PRETTY_PRINT_SOURCE` 为合法
    // Luau 源码，与 cpp 逐字一致。
    let state = new_fixture_state(PRETTY_PRINT_SOURCE)
      .unwrap_or_else(|reason| panic!("ReplFixture 初始化失败: {reason}"));

    Self { state }
  }

  /// cpp `ReplFixture::L`：夹具持有的主线程状态，生命周期由 `state` 守卫保证。
  /// 仅供 repl-cli 的指针形接口（`runCode`/`getCompletions`）与集成测试使用；
  /// crate 内写路径一律走 [`Self::state_mut`]。
  pub fn l(&self) -> *mut LuaState {
    self.state.0
  }

  /// 守卫所存活状态的可变引用：裸指针解引用在夹具层收口于此一处。
  pub fn state_mut(&mut self) -> &mut LuaState {
    // Safety: `state.0` 恒指向守卫建立契约下的非空活跃主线程状态，
    // 且 `&mut self` 保证本借用窗口内无其他可变别名。
    unsafe { &mut *self.state.0 }
  }
}

impl Default for ReplFixture {
  fn default() -> Self {
    Self::new()
  }
}
