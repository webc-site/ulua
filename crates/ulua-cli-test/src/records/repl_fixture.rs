//! cpp `tests/Repl.test.cpp` 的 `class ReplFixture`（`RequireByString.test.cpp`
//! 的 `ReplWithPathFixture` 是它的别名，见 [`crate::records::repl_with_path_fixture`]）。

use ulua_vm::{functions::lua_close::lua_close, type_aliases::lua_state::lua_State};

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

/// 持有 `lua_State` 生命周期：cpp 用 `unique_ptr<lua_State, void(*)(lua_State*)>`，
/// Rust 侧对应 `Drop` 里 `lua_close`。
#[derive(Debug)]
pub struct ReplFixture {
  /// cpp `ReplFixture::L`
  pub l: *mut lua_State,
}

impl ReplFixture {
  /// 对齐 cpp 构造函数：`luaL_newstate` → `setupState` → `luaL_sandboxthread`
  /// → `runCode(prettyPrintSource)`。
  pub fn new() -> Self {
    // SAFETY: 返回的状态由本 fixture 在 `Drop` 中 `lua_close`；`PRETTY_PRINT_SOURCE`
    // 为合法 Luau 源码，与 cpp 逐字一致。
    let l = unsafe { new_fixture_state(PRETTY_PRINT_SOURCE) };

    Self { l }
  }
}

impl Default for ReplFixture {
  fn default() -> Self {
    Self::new()
  }
}

impl Drop for ReplFixture {
  fn drop(&mut self) {
    if !self.l.is_null() {
      // SAFETY: `l` 由 `new` 创建且尚未关闭。
      unsafe { lua_close(self.l) };
    }
  }
}
