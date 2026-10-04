//! Source: `VM/src/laux.cpp:616-679` (hand-ported)

use crate::{
  enums::lua_type::LuaType,
  functions::{
    c_slice, cstr_cow, lua_encodepointer::lua_encodepointer, lua_l_callmeta::lua_l_callmeta_bytes,
    lua_l_typename::lua_l_typename, lua_pushfstring_l::lua_pushfstring_l,
    lua_pushlstring::lua_pushlstring_bytes, lua_tointeger_64::lua_tointeger_64,
    lua_tolstring::lua_tolstring_ref, lua_tonumberx::lua_tonumberx, lua_topointer::lua_topointer,
    lua_tovector::lua_tovector, luai_int_2_str::luai_int2str, luai_num_2_str::luai_num2str_buf,
  },
  macros::{
    lua_l_error::luaL_error, lua_vector_size::LUA_VECTOR_SIZE, luai_maxint_2_str::LUAI_MAXINT2STR,
    luai_maxnum_2_str::LUAI_MAXNUM2STR,
  },
  records::lua_state::LuaState,
};

/// Rust 内部核心（§2/§3 出参收口）：cpp `luaL_tolstring`（`laux.cpp:612`）——按
/// `__tostring` 元方法/内建规则把 `idx` 槽值串化并**压栈**，返回结果串的全部字节
/// （含内嵌 `\0`，长度即切片长度）。
///
/// C 形 `size_t* len` 出参收口为 `Option<&'a [u8]>`，全消费面（含 C-ABI 侧）实测直用
/// 本切片形，旧出参垫片已随零消费删除（r12-w6 §7 复核）。栈语义与 cpp 逐字保持：结果串
/// 留在栈顶（由调用方弹出），`__tostring` 返回非串时经 `luaL_error` 抛错发散。
///
/// w6e 入侧收形：`l` 由裸 `*mut LuaState` 折为 `&mut` 引用形参（判例家族同
/// `lua_pushlstring_bytes`/`lua_l_register_bytes`），真实裸操作（`lua_tolstring_ref`
/// 切片出口、`lua_l_error_l` 发散、Vector 分量窗、`cstr_cow`、`lua_pushfstring_l`）
/// 落逐句窄 `unsafe` 块。
///
/// # Safety
/// `l` 的存活已由引用形参承载；残留义务为调用方自选的输出寿命 `'a`（与
/// [`lua_tolstring_ref`] 同形）：返回切片借用自压入栈顶的结果串内部，仅在该栈下次
/// 栈操作前有效，调用方不得跨 push/截断持有；`idx` 须为其合法栈索引；`__tostring`
/// 元方法回跑可改栈、可抛错，返回后不得继续持旧栈槽指针。
/// cpp laux.cpp:612。
pub unsafe fn lua_l_tolstring_ref<'a>(l: &mut LuaState, idx: i32) -> Option<&'a [u8]> {
  if lua_l_callmeta_bytes(l, idx, b"__tostring") != 0 {
    // SAFETY: `lua_tolstring_ref` 自身 `# Safety` 契约（栈索引合法、返回借用仅存续至
    // 下一次栈操作）由本函数 `# Safety` 承接转呈；`l` 经引用→裸指针隐式折形透传
    let s = unsafe { lua_tolstring_ref(l, -1) };
    if s.is_none() {
      // SAFETY: `lua_l_error_l` raise 后发散（`!`），静态消息无 fmt 实参
      unsafe { luaL_error!(l, "'__tostring' must return a string") }
    }
    return s;
  }

  match l.type_of(idx) {
    LuaType::Nil => {
      lua_pushlstring_bytes(l, b"nil");
    }
    LuaType::Boolean => {
      if l.to_boolean(idx) {
        lua_pushlstring_bytes(l, b"true");
      } else {
        lua_pushlstring_bytes(l, b"false");
      }
    }
    LuaType::Number => {
      // Number 类型槽必可转换；旧形忽略 isnum、失败时格式化 0.0，unwrap_or(0.0) 等价
      let n = lua_tonumberx(&*l, idx).unwrap_or(0.0);
      let mut s = [0u8; LUAI_MAXNUM2STR as usize];
      let len = luai_num2str_buf(&mut s, n);
      lua_pushlstring_bytes(l, &s[..len]);
    }
    LuaType::Vector => {
      let v = lua_tovector(&*l, idx);
      let mut s = [0u8; (LUAI_MAXNUM2STR as usize) * (LUA_VECTOR_SIZE as usize)];
      let mut pos = 0;
      // SAFETY: Vector 型槽上 `lua_tovector` 返回 TValue vector 载荷区指针（record 自持
      // 布局不变量），取 `LUA_VECTOR_SIZE` 分量恒在界内
      for (i, &comp) in unsafe { c_slice(v, LUA_VECTOR_SIZE as usize) }
        .iter()
        .enumerate()
      {
        if i != 0 {
          s[pos] = b',';
          s[pos + 1] = b' ';
          pos += 2;
        }
        pos += luai_num2str_buf(&mut s[pos..], comp as f64);
      }
      lua_pushlstring_bytes(l, &s[..pos]);
    }
    LuaType::String => {
      l.push_value(idx);
    }
    LuaType::Integer => {
      let val = lua_tointeger_64(&*l, idx);
      let mut s = [0u8; LUAI_MAXINT2STR as usize];
      let len = luai_int2str(&mut s, val);
      lua_pushlstring_bytes(l, &s[..len]);
    }
    _ => {
      let ptr = lua_topointer(l, idx);
      let enc = lua_encodepointer(&*l, ptr as usize);
      // SAFETY: `cstr_cow` 契约——`lua_l_typename` 返回 NUL 终止可读的类型名静态串
      unsafe {
        let name = cstr_cow(lua_l_typename(&*l, idx));
        // SAFETY: `lua_pushfstring_l` 家族契约——格式化并压栈，可分配、可 GC；
        // 返回指针即结果串槽位，按 cpp 同形弃值
        lua_pushfstring_l(l, format_args!("{}: 0x{:016x}", name, enc));
      }
    }
  }

  // SAFETY: 与首处 `lua_tolstring_ref` 消费点同形（`# Safety` 契约承接，'a 义务由
  // 本函数调用方经上文承担）
  unsafe { lua_tolstring_ref(l, -1) }
}
