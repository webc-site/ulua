use core::ptr::copy;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_absindex::lua_absindex, lua_g_readonlyerror::check_writable, lua_pushnil::lua_pushnil,
    lua_rawgeti::lua_rawgeti, lua_rawiter::lua_rawiter, lua_rawseti::lua_rawseti,
    lua_type::lua_type,
  },
  macros::{
    lua_c_barrierfast::lua_c_barrierfast, lua_newtable::lua_newtable, lua_pop::lua_pop,
    lua_tointeger::lua_tointeger, lua_tonumber::lua_tonumber,
  },
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};
/// cpp `ltablib.cpp:tovalidintkey`：`idx` 处为落在 `[f, e]` 内的整数键时返回该键。
///
/// cpp 用 `int* result` 出参 + bool 返回值，Rust 版折叠为 `Option<i32>`。
///
/// # Safety
///
/// `l` 必须指向存活的 `LuaState`，`idx` 为其上可读的合法栈索引
/// （`lua_type`/`lua_tonumber!` 直接解引用该状态读取槽位）。
unsafe fn tovalidintkey(l: *mut LuaState, idx: i32, f: i32, e: i32) -> Option<i32> {
  // Safety: 契约保证 l 存活且 idx 合法，lua_type/tonumber 只读该栈槽的 tag/数值
  unsafe {
    if lua_type(l, idx) == LuaType::Number as i32 {
      let nkey = lua_tonumber!(l, idx);
      if nkey >= f as f64 && nkey <= e as f64 {
        let result = nkey as i32;
        if (result as f64) == nkey {
          return Some(result);
        }
      }
    }
    None
  }
}

/// 数组区间拷贝：源元素均经 setobj2t 写入（构造时已验证 liveness），
/// 正/逆序分支仅为手写 memmove，此处直接用 ptr::copy 语义等价且单次向量化拷贝。
///
/// # Safety
/// `[srcarray + f - 1, srcarray + f - 1 + n)` 与 `[dstarray + t - 1, dstarray + t - 1 + n)`
/// 必须落在各自表的 sizearray 内，调用方已保证。
#[inline]
unsafe fn copy_array_range(srcarray: *mut TValue, dstarray: *mut TValue, f: i32, t: i32, n: i32) {
  // Safety: 契约保证两 offset 区间各在 sizearray 界内；ptr::copy 即 memmove，允许源/目区间重叠
  unsafe {
    copy(
      srcarray.offset((f - 1) as isize),
      dstarray.offset((t - 1) as isize),
      n as usize,
    );
  }
}

/// # Safety
///
/// `l` 必须指向存活 `LuaState` 且栈顶预留 >=1 槽（rawgeti 逐元素中转压栈）；`srct`/`dstt`
/// 为指向两表的合法栈索引，`[f, f+n)`/`[t, t+n)` 键在 raw 语义下可读/可写。
/// 栈槽区间搬移：经 lua_rawgeti/lua_rawseti 逐元素搬运，方向语义同数组拷贝
#[inline]
unsafe fn move_stack_range(
  l: *mut LuaState,
  srct: i32,
  dstt: i32,
  f: i32,
  t: i32,
  n: i32,
  reverse: bool,
) {
  // Safety: 契约保证 l/两表索引有效；每轮 rawgeti+rawseti 成对保持栈深不变
  unsafe {
    // f+i/t+i 即两表整数键：改为源/目标两条等差键区间 zip 游走，消除手工 i 与逐轮基址加法；
    // 端点用 i64 计算，极端键下 i32 端点会溢出，键值本身恒在 i32 域内回截无损
    let src_keys = i64::from(f)..i64::from(f) + i64::from(n);
    let dst_keys = i64::from(t)..i64::from(t) + i64::from(n);
    if reverse {
      // 区间重叠时按降序搬运，语义同 memmove 逆向分支
      for (srckey, dstkey) in src_keys.rev().zip(dst_keys.rev()) {
        lua_rawgeti(l, srct, srckey as i32);
        lua_rawseti(l, dstt, dstkey as i32);
      }
    } else {
      for (srckey, dstkey) in src_keys.zip(dst_keys) {
        lua_rawgeti(l, srct, srckey as i32);
        lua_rawseti(l, dstt, dstkey as i32);
      }
    }
  }
}

/// # Safety
///
/// `l` 必须指向当前执行 C 函数的存活 `LuaState`（读 `(*l).base + srct-1/dstt-1` 两栈槽）；
/// `srct`/`dstt` 解析出的栈值必须是表（hvalue 直接取 LuaTable 指针），`f <= e + 1`。
pub(crate) unsafe fn moveelements(
  l: *mut LuaState,
  srct: i32,
  dstt: i32,
  f: i32,
  e: i32,
  t: i32,
  sparsemove: bool,
) {
  // Safety: 契约保证 base+srct-1/base+dstt-1 落在当前帧栈内且槽值为表，hvalue 解引用合法
  unsafe {
    let src = (*(*l).base.offset((srct - 1) as isize)).as_table_ptr();
    let dst = (*(*l).base.offset((dstt - 1) as isize)).as_table_ptr();

    check_writable(l, dst);

    let n = e - f + 1;
    let f_index = (f as u32).wrapping_sub(1);
    let t_index = (t as u32).wrapping_sub(1);
    let n_unsigned = n as u32;

    if f_index < (*src).sizearray as u32
      && t_index < (*dst).sizearray as u32
      && f_index.wrapping_add(n_unsigned) <= (*src).sizearray as u32
      && t_index.wrapping_add(n_unsigned) <= (*dst).sizearray as u32
    {
      let srcarray = (*src).array;
      let dstarray = (*dst).array;

      // ptr::copy 自身处理区间重叠（memmove 语义），无需再按方向分支
      copy_array_range(srcarray, dstarray, f, t, n);

      lua_c_barrierfast!(l, dst);
    } else if sparsemove {
      let srcta = lua_absindex(l, srct);
      let dstta = lua_absindex(l, dstt);
      let te = t + (n - 1);

      lua_newtable(l);

      let mut iter = 0;
      loop {
        iter = lua_rawiter(l, srcta, iter);
        if iter == -1 {
          break;
        }
        match tovalidintkey(l, -2, f, e) {
          // 命中：lua_rawseti 自行弹出值；未命中：手动弹出值
          Some(ikey) => lua_rawseti(l, -3, ikey),
          None => lua_pop(l, 1),
        }
        lua_pop(l, 1); // 弹出键
      }

      iter = 0;
      loop {
        iter = lua_rawiter(l, dstta, iter);
        if iter == -1 {
          break;
        }
        if let Some(ikey) = tovalidintkey(l, -2, t, te) {
          lua_pushnil(l);
          lua_rawseti(l, dstta, ikey);
        }
        lua_pop(l, 2);
      }

      iter = 0;
      loop {
        iter = lua_rawiter(l, -1, iter);
        if iter == -1 {
          break;
        }
        let ikey = lua_tointeger!(l, -2);
        lua_rawseti(l, dstta, ikey - f + t);
        lua_pop(l, 1);
      }

      lua_pop(l, 1);
    } else {
      // 同表且目标区间前向重叠（t<=e && t>f）须逆向搬运，其余正向
      move_stack_range(l, srct, dstt, f, t, n, t <= e && t > f && dst == src);
    }
  }
}
