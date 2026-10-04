use core::ptr::copy;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_g_readonlyerror::check_writable, lua_rawgeti::lua_rawgeti, lua_rawiter::lua_rawiter,
    lua_rawseti::lua_rawseti,
  },
  macros::{abs_index::abs_index, lua_c_barrierfast::lua_c_barrierfast},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// cpp `ltablib.cpp:tovalidintkey`：`idx` 处为落在 `[f, e]` 内的整数键时返回该键。
///
/// cpp 用 `int* result` 出参 + bool 返回值，Rust 版折叠为 `Option<i32>`。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&LuaState` 接收者类型承载；r12-w6
/// 收形后判型/读数全经安全 `type_of`/`to_number` 门面，体内已无裸操作，故本体降为安全
/// `fn`）：`idx` 为 `l` 上可读的合法栈索引。
fn tovalidintkey(l: &LuaState, idx: i32, f: i32, e: i32) -> Option<i32> {
  if l.type_of(idx) == LuaType::Number {
    let nkey = l.to_number(idx).unwrap_or(0.0);
    if nkey >= f as f64 && nkey <= e as f64 {
      let result = nkey as i32;
      if (result as f64) == nkey {
        return Some(result);
      }
    }
  }
  None
}

/// 数组区间拷贝：源元素均经 setobj2t 写入（构造时已验证 liveness），
/// 正/逆序分支仅为手写 memmove，此处直接用 ptr::copy 语义等价且单次向量化拷贝。
///
/// # Safety
/// `[srcarray + f - 1, srcarray + f - 1 + n)` 与 `[dstarray + t - 1, dstarray + t - 1 + n)`
/// 必须落在各自表的 sizearray 内，调用方已保证。
#[inline]
unsafe fn copy_array_range(srcarray: *mut TValue, dstarray: *mut TValue, f: i32, t: i32, n: i32) {
  // SAFETY: 契约保证两 offset 区间各在 sizearray 界内；ptr::copy 即 memmove，允许源/目区间重叠
  unsafe {
    copy(
      srcarray.offset((f - 1) as isize),
      dstarray.offset((t - 1) as isize),
      n as usize,
    );
  }
}

/// 栈槽区间搬移：经 lua_rawgeti/lua_rawseti 逐元素搬运，方向语义同数组拷贝。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活/独占已由 `&mut LuaState` 承载；r12-w6 收形
/// 后 rawgeti/rawseti 均引用形安全门面，体内已无裸操作，故本体降为安全 `fn`）：`srct`/`dstt`
/// 为界内可读的表索引，每轮 rawgeti+rawseti 成对保持栈深不变。
#[inline]
fn move_stack_range(l: &mut LuaState, srct: i32, dstt: i32, f: i32, t: i32, n: i32, reverse: bool) {
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

/// 调用序契约（正确性，非内存安全——r12-w6 收形：`l` 前移 `&mut LuaState` 引用形，签名不再
/// 携带调用方裸指针；本票沿 `tmove` r16-v41 判例，体内裸操作收进窄 `unsafe` 块后本体降为安全
/// `fn`）：`l` 为当前执行 C 函数的存活状态且栈顶预留 >=1 槽（rawgeti 逐元素中转压栈）；
/// `srct`/`dstt` 解析出的栈值必须是表，`f <= e + 1`。
///
/// 体内保留的窄 `unsafe` 面均有既立裁决锚点：两处帧槽取表指针经 `slot`/`as_table_ptr`
/// 句柄门面（r13-w1a 逐点定性见块内注）；`(*src).sizearray`/`(*dst).array` 等表头裸读与
/// `copy_array_range` 的 memmove 属 ltable 自有字段的不变量保护面（E1 窗化裁决保留形）；
/// `check_writable` 与 `as_mut_ptr()` 裸转手按 r16-v21 判例保留，借用窗止于当句。
pub(crate) fn moveelements(
  l: &mut LuaState,
  srct: i32,
  dstt: i32,
  f: i32,
  e: i32,
  t: i32,
  sparsemove: bool,
) {
  // r13-w1a 逐点定性（w6d 口径保留面；原普查 13 行）：两处帧槽 hvalue 式取表指针
  // 收编为 records/slot.rs 既有 api 索引域构造方法 slot(idx)（非新增门面，读面
  // 同形判例 lua_v_settable 的 get+as_table_ptr）——全部调用方（tinsert/tremove
  // 恒传 1、tmove 传 1/5）为契约内正帧索引，index_2_addr 正索引分支界内路径恰为
  // base+idx-1，地址逐位恒等、读数位点不变；api_check 与 nil 哨兵分支在契约域
  // 不可达。CallInfo 裸字段红线不动。check_writable 指针取参与
  // lua_rawgeti/lua_rawseti/lua_rawiter/abs_index 引用形取参系自由函数调用点，
  // 非裸解引用面，保留。其余命中——new_table/pop×5/push_nil/to_integer 与
  // tovalidintkey 的 type_of/to_number——皆 records/lua_state 既有门面收编形态，
  // 零翻案、零新造门面、零动作。
  // SAFETY: 契约保证 srct/dstt 为界内正帧索引，slot 换算落当前帧栈内且槽值为表，hvalue 解引用合法
  unsafe {
    let src = l.slot(srct).get().as_table_ptr();
    let dst = l.slot(dstt).get().as_table_ptr();

    check_writable(l.as_mut_ptr(), dst);

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
      let srcta = abs_index(l, srct);
      let dstta = abs_index(l, dstt);
      let te = t + (n - 1);

      l.new_table();

      let mut iter = 0;
      loop {
        iter = lua_rawiter(l, srcta, iter);
        if iter == -1 {
          break;
        }
        match tovalidintkey(l, -2, f, e) {
          // 命中：lua_rawseti 自行弹出值；未命中：手动弹出值
          Some(ikey) => lua_rawseti(l, -3, ikey),
          None => l.pop(1),
        }
        l.pop(1); // 弹出键
      }

      iter = 0;
      loop {
        iter = lua_rawiter(l, dstta, iter);
        if iter == -1 {
          break;
        }
        if let Some(ikey) = tovalidintkey(l, -2, t, te) {
          l.push_nil();
          lua_rawseti(l, dstta, ikey);
        }
        l.pop(2);
      }

      iter = 0;
      loop {
        iter = lua_rawiter(l, -1, iter);
        if iter == -1 {
          break;
        }
        let ikey = l.to_integer(-2).unwrap_or(0);
        lua_rawseti(l, dstta, ikey - f + t);
        l.pop(1);
      }

      l.pop(1);
    } else {
      // 同表且目标区间前向重叠（t<=e && t>f）须逆向搬运，其余正向
      move_stack_range(l, srct, dstt, f, t, n, t <= e && t > f && dst == src);
    }
  }
}
