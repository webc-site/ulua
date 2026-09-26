//! Source: `VM/src/laux.cpp:529-582` (hand-ported)

use core::{ffi::c_char, slice::from_raw_parts};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_addlstring::lua_l_addlstring, lua_l_addvalue::lua_l_addvalue,
    lua_l_tolstring::lua_l_tolstring_ref, lua_toboolean::lua_toboolean,
    lua_tointeger_64::lua_tointeger_64, lua_tolstring::lua_tolstring_ref,
    lua_tonumberx::lua_tonumberx, lua_type::lua_type, luai_int_2_str::luai_int2str,
    luai_num_2_str::luai_num2str_buf,
  },
  macros::{luai_maxint_2_str::LUAI_MAXINT2STR, luai_maxnum_2_str::LUAI_MAXNUM2STR},
  records::lua_l_strbuf::LuaLStrbuf,
};

/// C++ `void luaL_addvalueany(luaL_Strbuf *b, int idx)` —
/// converts the value at stack index `idx` to its string representation
/// and appends it to buffer `b`.
/// # Safety
///
/// `b` 必须指向已在存活 `lua_State` 上初始化、尚未 `pushresult` 的 `LuaLStrbuf`，`idx`
/// 为其栈上合法索引；串化各分支可分配（数字转串、`lua_tolstring` 就地转换）并触发 GC，
/// 非串非数值类型走 `lua_l_tolstring`+`lua_l_addvalue`，要求该路径可安全改动栈顶。
/// cpp laux.cpp:525 `luaL_addvalueany`。
pub(crate) unsafe fn lua_l_addvalueany(b: &mut LuaLStrbuf, idx: i32) {
  // Safety: 契约保证 b 与其 lua_State 一致、idx 栈槽可读；定长数字缓冲按 luai_num2str/int2str
  // 返回长度截取读取，界内
  unsafe {
    let l = b.l;

    match lua_type(l, idx) {
      // cpp release 构建 LUAU_ASSERT 编译掉后 break 直落：不追加任何内容
      x if x == LuaType::None as i32 => {}
      x if x == LuaType::Nil as i32 => {
        lua_l_addlstring(b, c"nil".to_bytes());
      }
      x if x == LuaType::Boolean as i32 => {
        if lua_toboolean(l, idx) != 0 {
          lua_l_addlstring(b, c"true".to_bytes());
        } else {
          lua_l_addlstring(b, c"false".to_bytes());
        }
      }
      x if x == LuaType::Number as i32 => {
        // Number 类型槽必可转换；旧形忽略 isnum、失败时格式化 0.0，unwrap_or(0.0) 等价
        let n = lua_tonumberx(l, idx).unwrap_or(0.0);
        let mut s = [0 as c_char; LUAI_MAXNUM2STR as usize];
        let len = luai_num2str_buf(&mut s, n);
        lua_l_addlstring(b, from_raw_parts(s.as_ptr() as *const u8, len));
      }
      x if x == LuaType::String as i32 => {
        // 类型闸门已确认为串，`lua_tolstring_ref` 恒 `Some`（`None` 为 cpp 不可达路径）
        if let Some(s) = lua_tolstring_ref(l, idx) {
          lua_l_addlstring(b, s);
        }
      }
      x if x == LuaType::Integer as i32 => {
        let n = lua_tointeger_64(l, idx);
        let mut s = [0 as c_char; LUAI_MAXINT2STR as usize];
        let len = luai_int2str(&mut s, n);
        lua_l_addlstring(b, from_raw_parts(s.as_ptr() as *const u8, len));
      }
      _ => {
        // note: luaL_addlstring assumes box is stored at top of stack, so we can't call it here
        // instead we use luaL_addvalue which will take the string from the top of the stack and add that
        // 结果串留在栈顶即目的（`lua_l_addvalue` 负责弹出并追加），切片引用不外传
        let _ = lua_l_tolstring_ref(l, idx);
        lua_l_addvalue(b);
      }
    }
  }
}
