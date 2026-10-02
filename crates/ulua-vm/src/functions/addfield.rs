//! Source: `VM/src/ltablib.cpp:305`
//!
//! Helper for `table.concat` — append element `i` of table `t` to the buffer.
//! Fast path reads a string directly from the array part; otherwise it goes
//! through `lua_rawgeti` and rejects non-string/number values.
//!
//! r12-w5s T9 切片化：快路径真实逻辑落在切片核心 [`addfield_fast_ref`]——数组段经
//! `LuaTable::array_window` E1 共享窗消费（旧 `(*t.array.add(..))` 手工裸指针点位
//! 全部退役，越界降级为 `get` 归 None 走慢路）；串 payload 窗收敛为
//! [`tstring_payload`] 单点派生（与 `lua_tolstring_ref` 栈槽窗同入约模型），
//! 慢路径（rawgeti/类型闸/错误文本）留在外调度面，取参序不动。

use core::slice::from_raw_parts;

use ulua_common::functions::c_str::cstr_cow;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_addlstring::lua_l_addlstring, lua_l_addvalue::lua_l_addvalue,
    lua_l_typename::lua_l_typename, lua_rawgeti::lua_rawgeti,
  },
  macros::lua_l_error::luaL_error,
  records::{
    lua_l_strbuf::LuaLStrbuf, lua_state::LuaState, lua_t_value::TValue, lua_table::LuaTable,
    t_string::TString,
  },
};

/// 本模块串 payload 窗唯一派生点：存活 TString 共享借用的前 `len` 字节可读窗
/// （不含终止 NUL）——与 `lua_tolstring_ref` 取栈槽串窗同入约模型（cpp
/// `getstr(ts), ts->len` 的读界）：Lua 串不可变、不搬移，返回切片寿命经省略
/// 规则钉在入参 `&TString` 借用上，回收窗口与串对象一致。
///
/// # Safety
/// `ts` 须为存活 TString（调用方以 `TValue::is_string` 判定 + `as_string` 借用
/// 建立前提）；返回切片随入参借用存活，串须在切片使用期内不被回收。
#[inline]
unsafe fn tstring_payload(ts: &TString) -> &[u8] {
  // SAFETY: 契约保证 `ts` 存活；`data` 为柔性数组成员首址，tstring 布局保证
  // `len` 字节可读（空串 len 0 即空窗，触不到哨兵槽）
  unsafe { from_raw_parts(ts.data.as_ptr().cast::<u8>(), ts.len as usize) }
}
/// 快路径拼接核心（切片形，真实逻辑）：数组段窗 `arr`（cpp
/// `t->array[0..sizearray]`，经 `LuaTable::array_window` 派生）槽 `i - 1` 命中
/// 字符串时把其 payload 追加进拼接缓冲并返回 true；窗外或非串返回 false，调用方
/// 落 `lua_rawgeti` 慢路径（cpp `unsigned(i - 1) < unsigned(t->sizearray) &&
/// ttisstring(&t->array[i - 1])` 双判同形）。
///
/// # Safety
/// `b` 契约同 [`lua_l_addlstring`]（已 init 未提交的缓冲游标态）；`arr` 窗内槽
/// 读由 `&LuaTable` 共享借用钉住存活期；串 payload 窗经 [`tstring_payload`] 于
/// `is_string` 闸后派生。
unsafe fn addfield_fast_ref(b: &mut LuaLStrbuf, arr: &[TValue], i: i32) -> bool {
  // C++ does `cast_to(unsigned, i - 1)` here; for i = INT_MIN the `i - 1`
  // is signed-overflow UB upstream (ltablib.cpp:232). wrapping_sub matches
  // the two's-complement value C++ relies on: it wraps to INT_MAX, fails the
  // `< sizearray` bound, and falls through to the rawgeti slow path——窗 `get`
  // 归 None 同形。`u32 → usize` 恒宽化不绕回（u32 域 ≤ 地址空间；32 位目标上为
  // 恒等转换，窗外由 `get` 收敛），100% 安全。
  let Some(field) = arr
    .get(i.wrapping_sub(1) as u32 as usize)
    .filter(|field| field.is_string())
  else {
    return false;
  };
  // SAFETY: `is_string` 闸保证槽 gc 分支即存活 TString（lobject `ttisstring` 形）
  let ts = unsafe { field.as_string() };
  // SAFETY: 槽借用期内串不回收，[`tstring_payload`] 入约成立
  let s = unsafe { tstring_payload(ts) };
  // SAFETY: `b` 契约经本核心 `# Safety` 由调用方（`addfield`）承接；`s` 的读取
  // 界由切片自带且不与 `b` 内部缓冲重叠（cpp `luaL_addlstring` 同形前置）
  unsafe { lua_l_addlstring(b, s) };
  true
}

/// # Safety
///
/// `l` 为存活 `LuaState`；`t` 为共享只读借用，`None` 对应 C++ 空表慢路径（JIT 以 NULL 表指针表示空表），
/// `Some` 时其 `array[0..sizearray]` 可读；`b` 为其上初始化的缓冲。
pub(crate) unsafe fn addfield(l: *mut LuaState, b: &mut LuaLStrbuf, i: i32, t: Option<&LuaTable>) {
  // 快路径：数组段切 E1 共享窗喂 [`addfield_fast_ref`]（窗借用随判定结束，
  // 拼接写入经 addlstring 缓冲协议扩展）
  if let Some(t) = t {
    // SAFETY: 契约保证 `t` 的数组窗可读（E1 `array_window` 入约）
    if unsafe { addfield_fast_ref(b, t.array_window(), i) } {
      return;
    }
  }

  // 慢路径：rawgeti 取值 + 非串非数报错 + addvalue，抛出序与错误文本对齐
  // cpp ltablib.cpp:313-318（`t` 为 None 的 JIT 空表亦走此路）
  unsafe {
    let tt = lua_rawgeti(&mut *l, 1, i);
    if tt != LuaType::String as i32 && tt != LuaType::Number as i32 {
      let tn = cstr_cow(lua_l_typename(&*l, -1));
      luaL_error!(
        l,
        "invalid value ({}) at index {} in table for 'concat'",
        tn,
        i
      );
    }
    lua_l_addvalue(b);
  }
}
