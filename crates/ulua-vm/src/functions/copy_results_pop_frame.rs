//! C 调用返回收尾公共样板（cpp lvmexecute.cpp `luaD_precall` C 路径与
//! `luau_poscall`/`LOP_CALL`/`LOP_RETURN` 同款拷回-补 nil-弹帧三连）的单源。
//!
//! 原先在 `luau_precall`、`luau_poscall`、`luau_execute` 各持一份同构实现；
//! 两层接口按 `l->top` 收口协议拆分：C 路径恒收在结果尾，Lua 路径 MULTRET 收
//! 结果尾、否则恢复父帧 top。
//!
//! B2-2b 帧协议族裁决（复核 B2-1 判定，结论=保留裸 StkId 步进形态）：
//! 既有约定（review.md §2）：跨帧槽步进搬运——源 `vali..valend` 与目标
//! `((*ci).func)..` 同栈异区且**可重叠**（RETURN 臂 `ra == base` 时目标起点即源前
//! 一格，cpp 前向拷贝语义要求 dst<src 的移位序），切片形态（`from_raw_parts` 造
//! `&mut [Slot]`/`copy_from_slice`/zip）要么需不成立的无别名证明、要么把步进
//! 折叠成 from_raw→as_ptr 零收益往返（同 B2-1 判定）；`nresults = -1`（MULTRET）
//! 是 `i != 0` 循环节哨兵，计数语义须保持 i32，不宜折 `0..n` usize 区间。
//! §9.4：本三连承担每一次 Lua/C 帧返回（VM 最热收尾路径），指针步进循环与切片
//! 游标 codegen 等价，切片化不增益反而扩大别名论证面。

use crate::{
  macros::{lua_multret::LUA_MULTRET, setnilvalue::setnilvalue, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// 把 `[vali, valend)` 的值按 `nresults` 上限拷回 `(*ci).func`（不足补 nil），
/// 弹出当前帧（`ci`/`base`），返回补齐后的「结果尾」指针。不动 `(*l).top`——
/// 由调用方按各自协议收口（C 路径收结果尾，Lua 路径见 [`pop_frame_copy_results`]）。
///
/// # Safety（由调用侧 unsafe 上下文承担）
///
/// `l` 指向存活 `LuaState` 且其 `ci` 为当前 Lua/C 帧（CALL 布局保证 func..top 可写）；
/// `vali <= valend` 且两指针指向同一存活栈区间；`nresults` 为 -1/0/正数声明的结果数。
#[inline(always)]
pub(crate) unsafe fn copy_results_pop_frame(
  l: *mut LuaState,
  vali: StkId,
  valend: StkId,
  nresults: i32,
) -> StkId {
  // SAFETY: 契约保证 l/ci 有效、vali..valend 为存活栈区间，拷贝目标 (*ci).func 起同样在栈内可写
  unsafe {
    // ci is our callinfo, cip is our parent
    let ci = (*l).ci;
    let cip = ci.sub(1);

    let dst = (*ci).func;

    // copy return values into parent stack (but only up to nresults!),
    // fill the rest with nil
    // note: in MULTRET context nresults starts as -1 so i != 0 condition
    // never activates intentionally (cpp: lvmexecute.cpp:1300)
    //
    // 单返回值与零返回值快路径特化：占全部函数调用的绝对主流，消灭循环状态机与双重分支
    let res = match nresults {
      1 => {
        if vali < valend {
          setobj_2_s!(l, dst, vali);
        } else {
          setnilvalue!(dst);
        }
        dst.add(1)
      }
      0 => dst,
      _ => {
        let mut res = dst;
        let mut vali = vali;
        let mut i = nresults;
        while i != 0 && vali < valend {
          setobj_2_s!(l, res, vali);
          res = res.add(1);
          vali = vali.add(1);
          i -= 1;
        }
        while i > 0 {
          setnilvalue!(res);
          res = res.add(1);
          i -= 1;
        }
        res
      }
    };

    // pop the stack frame（统一弹栈帧单源收口，恢复父帧 base）
    (*l).ci = cip;
    (*l).base = (*cip).base;
    res
  }
}

/// Lua 帧返回收口（cpp `luau_poscall` 尾句）：[`copy_results_pop_frame`] 之上把
/// `l->top` 收口——MULTRET 时收在结果尾，否则恢复父帧 top。
///
/// # Safety（同 [`copy_results_pop_frame`] 契约）
#[inline(always)]
pub(crate) unsafe fn pop_frame_copy_results(
  l: *mut LuaState,
  vali: StkId,
  valend: StkId,
  nresults: i32,
) {
  // SAFETY: 契约保证 l/ci 有效、vali..valend 为存活栈区间，拷贝目标 (*ci).func 起同样在栈内可写
  unsafe {
    let cip = (*l).ci.sub(1);
    let res = copy_results_pop_frame(l, vali, valend, nresults);

    (*l).reanchor_top(if nresults == LUA_MULTRET {
      res
    } else {
      (*cip).top
    });
  }
}
