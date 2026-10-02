use core::ffi::c_void;

use ulua_common::fflag;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_d_rawrunprotected_ldo::lua_d_rawrunprotected, lua_d_seterrorobj::lua_d_seterrorobj,
    lua_isyieldable::lua_isyieldable, resume_findhandler::resume_findhandler,
    resume_handle::resume_handle,
  },
  macros::expandstacklimit::expandstacklimit,
  records::lua_state::LuaState,
};

/// 协程恢复的收尾：逐个驱动 handler 帧细粒度恢复，最后归位 ccount 与状态
/// （cpp `resume_finish`，与 `resume_start` 成对调用）。
///
/// DELIBERATE DEVIATION：本地 oracle（cpp/VM/src/ldo.cpp:752）为引入两旗标前的
/// 旧快照，本函数按**更新上游**移植：`LuauXpcallFixMessageYieldPath` 与
/// `LuauResumeRestoreCcalls` 均已登记进 fflag 注册表（默认 false）。默认（旗关）
/// 路径在循环体内还原 `n_ccalls = old`，使 handler 续体窗口内 `lua_yield` 的
/// yieldable 检查被放行——与本地 oracle 的瞬态差异（终态四种旗标组合下恒等）；
/// 旗开路径与更新上游逐行对应。sync-cpp 时以更新上游为准，勿折叠回旧快照。
///
/// # Safety
/// 收尾在每次 `lua_d_rawrunprotected` 内驱动 handler 帧续体，`debugprotectederror` 回调可抛错/Break；
/// 结束时按 `status` 经 `luaD_seterrorobj`/`expandstacklimit` 归位错误对象与栈顶。cpp/VM/src/ldo.cpp:752 resume_finish。
/// `l` 须为存活协程状态；`status`/`old_n_ccalls` 须为本次 `resume_start` 成功后
/// 驱动 `resume` 受保护调用返回的状态与取回的 `n_ccalls` 基线——三者构成
/// resume_start/resume 驱动/resume_finish 的成对协议，缺环即 ccount 失衡。
pub(crate) unsafe fn resume_finish(l: *mut LuaState, mut status: i32, old_n_ccalls: i32) -> i32 {
  // SAFETY: 契约保证 `L`/协程 resume 链存活，循环内 resume_findhandler 取回的 handler 帧与 status 设置均限于该协程帧界
  //
  // r13-w1b 逐点定性（w6d 口径保留面）：本体的 `n_ccalls`/`base_ccalls` 读写、
  // `isactive` 落笔、`status` 两处写入、收尾 else-if 的 Ok 谓词与末行原值回传均
  // 无既有门面（LuaState 无 ccount/status 写面；status() 门面自带 non-repr→Ok 兜底，
  // 仅对 Break 谓词逐位等价，Ok 谓词与裸 u8 回传不可换用），恢复点动作位原样保留；
  // `(*l).gs_ref().cb` 为 global_State 链读数，r16-b1 收编经 gs_ref 只读门面（见其契约），非栈顶门面/边界原语覆盖面。
  // 收编共三处读数：循环内 Break 谓词经既有 status() 门面；错误收尾与栈顶归位两处
  // top 现读经 top_slot(0) 读数原语（见行内注）。
  unsafe {
    while status != LuaStatus::Ok as i32 {
      let ch = resume_findhandler(l);
      if ch.is_null() {
        break;
      }

      if lua_isyieldable(&*l) != 0
        && let Some(debugprotectederror) = (*l).gs_ref().cb.debugprotectederror
      {
        debugprotectederror(l);

        // 收编：Break 谓词经既有 status() 门面（from_repr(Break) 当且仅当字段为 6，
        // non-repr 兜底 Ok 亦 ≠Break——谓词逐位等价；读数位点不变，仍现读场域）
        if (*l).status() == LuaStatus::Break {
          status = LuaStatus::Ok as i32;
          break;
        }
      }

      if fflag::LuauXpcallFixMessageYieldPath.get() {
        (*l).base_ccalls = old_n_ccalls as u16;
      } else {
        (*l).n_ccalls = old_n_ccalls as u16;
        (*l).base_ccalls = (*l).n_ccalls;
      }

      (*l).status = status as u8;
      // SAFETY: 细粒度恢复入参契约——`ch` 为 `resume_findhandler` 取回的存活 handler 帧
      // （为空已在上文跳出循环），转 `void*` 透传，满足 `resume_handle` 的 `ud` 前置
      status = lua_d_rawrunprotected(l, Some(resume_handle), ch as *mut c_void);
    }

    if fflag::LuauResumeRestoreCcalls.get() {
      (*l).n_ccalls = (old_n_ccalls - 1) as u16;
    } else {
      (*l).base_ccalls = (*l).base_ccalls.wrapping_sub(1);
      (*l).n_ccalls = (*l).base_ccalls;
    }

    (*l).base_ccalls = (*l).n_ccalls;
    (*l).isactive = false;

    if status != LuaStatus::Ok as i32 {
      (*l).status = status as u8;
      // 收编：顶槽地址读数经 top_slot(0) 边界原语（镜像 cpp `L->top` 读数形，
      // off=0 即保留顶槽，位点现读不变）
      lua_d_seterrorobj(l, status, (*l).top_slot(0));
      // 保留（错误收尾恢复动作·同形单点）：seterrorobj 刚重定 top，本行必须现读
      // 其结果场并落 ci 帧界——场到场拷贝无算术操作数，cpp 同形 `ci->top = L->top;`
      (*(*l).ci).top = (*l).top;
    } else if (*l).status == LuaStatus::Ok as u8 {
      // 收编：宏参顶槽地址读数经 top_slot(0) 边界原语——宏体三处展开均在语句内
      // 现读同一 `(*l).top`（其间仅写 ci->top，无 top 落笔），读数与原裸字段一致
      expandstacklimit!(l, (*l).top_slot(0));
    }

    (*l).status as i32
  }
}
