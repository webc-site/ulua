use core::ptr::{addr_of_mut, null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{gcstep::gcstep, markroot::markroot, shrinkbuffersfull::shrinkbuffersfull},
  macros::{
    gc_percent_base::GC_PERCENT_BASE, gc_satomic::GCSSWEEP, gc_spause::GCSPAUSE,
    keepinvariant::keepinvariant, upisopen::upisopen,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活主 `LuaState`，`(*l).global` 指向其存活 `global_State` 且 GC 处于 `GCSPAUSE`/`GCSSWEEP` 阶段
/// （LUAU_ASSERT 前置）；本函数直接读写 `*g` 的 gray/grayagain/weak/sweepgcopage/allgcopages/uvhead 链表与
/// gc_threshold/gcstats 字段，遍历 `uvhead` 时每个 upval 须为已打开的合法环成员；`gcstep`/`markroot` 会再入标记-清扫，
/// 须在无并发、非收集回调中调用，不得在其它 GC 阶段（如 atomic/propagate）中途进入。
/// cpp VM/src/lgc.cpp:1372
pub unsafe fn lua_c_fullgc(l: *mut LuaState) {
  unsafe {
    let g = (*l).global;

    if keepinvariant(g) {
      (*g).sweepgcopage = (*g).allgcopages;
      (*g).gray = null_mut();
      (*g).grayagain = null_mut();
      (*g).weak = null_mut();
      (*g).gcstate = GCSSWEEP as u8;
    }

    LUAU_ASSERT!((*g).gcstate as i32 == GCSPAUSE || (*g).gcstate as i32 == GCSSWEEP);
    // 条件经 gcstep 在循环外改写 *g，clippy while_immutable_condition 无法
    // 识别裸指针别称写入，保留 loop+break 形态
    loop {
      if (*g).gcstate as i32 == GCSPAUSE {
        break;
      }
      LUAU_ASSERT!((*g).gcstate as i32 == GCSSWEEP);
      gcstep(l, usize::MAX);
    }

    let uvhead = addr_of_mut!((*g).uvhead);
    let mut uv = (*g).uvhead.u.open.next;
    while uv != uvhead {
      LUAU_ASSERT!(upisopen!(uv));
      (*uv).markedopen = 0;
      uv = (*uv).u.open.next;
    }

    markroot(l);
    loop {
      if (*g).gcstate as i32 == GCSPAUSE {
        break;
      }
      gcstep(l, usize::MAX);
    }

    shrinkbuffersfull(l);

    let heapgoalsizebytes = ((*g).totalbytes / GC_PERCENT_BASE) * (*g).gcgoal as usize;
    // cpp lgc.cpp:1405：按无符号回绕语义计算，避免极端 gcgoal/gcstepmul 配置下的溢出 panic
    // gcgoal/gcstepmul 以百分数存储、scale 全程落在 i32 域，故基准取 `as i32`（100 恒无截断）
    let scale = (*g)
      .gcgoal
      .wrapping_mul((*g).gcstepmul)
      .wrapping_div(GC_PERCENT_BASE as i32)
      .wrapping_sub(GC_PERCENT_BASE as i32);
    (*g).gc_threshold = (*g)
      .totalbytes
      .wrapping_mul(scale as usize)
      .wrapping_div((*g).gcstepmul as usize);

    if (*g).gc_threshold < (*g).totalbytes {
      (*g).gc_threshold = (*g).totalbytes;
    }

    (*g).gcstats.heapgoalsizebytes = heapgoalsizebytes;
  }
}
