/// NUL 结尾字节串（`*const c_char` 契约调用点 `.as_ptr().cast()`；§10 不引入 C 字符串类型）。
use core::{ffi::c_void, ptr::addr_of};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_d_throw_ldo::lua_d_throw, lua_g_pusherror::lua_g_pusherror, luau_poscall::luau_poscall,
    luau_precall::luau_precall, resume_continue::resume_continue,
  },
  macros::{
    curr_func::curr_func, isyielded::isyielded, lua_callinfo_return::LUA_CALLINFO_RETURN,
    lua_multret::LUA_MULTRET, pcrlua::PCRLUA, scheduled_reentry::SCHEDULED_REENTRY,
  },
  records::{closure::CClosure, lua_state::LuaState},
  type_aliases::stk_id::StkId,
};
const ERR_DEAD_COROUTINE: &[u8] = b"cannot resume dead coroutine\0";

/// 协程首次/续跑驱动的粗粒度恢复回调（cpp `resume`，经 `luaD_rawrunprotected` 进入）。
///
/// # Safety
/// 本回调经 `lua_d_rawrunprotected` 建立的受保护帧内运行，可 `lua_d_throw`/`lua_g_pusherror` unwind；
/// 转调 `luau_precall`/`resume_continue` 会驱动整段协程体，栈可重分配。cpp/VM/src/ldo.cpp:547 resume。
/// `l` 须为存活协程状态；`ud` 须为粗粒度恢复入参：`lua_resume` 传入的首实参栈槽
/// `top - nargs`（`StkId`），实参个数由 `resume_start` 的 `api_check!` 先核。
/// 与细粒度恢复 `resume_handle` 的差异及理由：本回调重放整个协程体（首次进入
/// 或从挂起点继续），`ud` 是首实参栈槽；`resume_handle` 仅推进出错 handler 帧的
/// 续体，`ud` 是 handler `CallInfo` 帧。恢复粒度不同，两者 `ud` 不可互换。
pub(crate) unsafe extern "C-unwind" fn resume(l: *mut LuaState, ud: *mut c_void) {
  // r14-p2 逐点定性（w6d 口径保留面，本文件 A=0 纯注记、零收编）：status 读数三处
  // 判别谓词均保留 C 形——`==Ok` 正谓词（其值域经 resume_start 契约限定为 {0(Ok),
  // 1(Yield), 6(Break)}：start 仅放行 Yield/Break 或 Ok 且 ci/base_ci 同形校验后建档，
  // 该域内谓词与 status() 门面真值表同值——此为值域论证备用；但门面 non-repr→Ok
  // 兜底对域外值（如 0x7f）把判真折成判假，逐位等价论证不成立，沿 r13 台账 Ok
  // 谓词保守口径保留）、`==SCHEDULED_REENTRY` 哨兵谓词（0x7f 为 non-repr 值，门面
  // 折叠即丢哨兵，必保留）、`!=Ok` 否定谓词（0x7f→Ok 兜底把判真折成判假，形同
  // resume_handle:113 判例，保留）；status 写面（挂起态归零恢复动作）无门面，保留。
  // base/ci/base_ci/flags 帧面读写作（入口契约断言、首实参窗基准、重入归基址、
  // RETURN 旗落笔与 pcall 帧回读）无既有门面且属 CallInfo 帧面 B 红线，照旧不翻案；
  // 形参 l 经 curr_func!/isyielded! 宏仅透传状态指针，宏体读数不计本文件普查。
  unsafe {
    let mut first_arg = ud as StkId;

    if (*l).status == LuaStatus::Ok as u8 {
      LUAU_ASSERT!((*l).ci == (*l).base_ci && first_arg >= (*l).base);
      if first_arg == (*l).base {
        lua_g_pusherror(l, ERR_DEAD_COROUTINE.as_ptr().cast());
        lua_d_throw(l, LuaStatus::ErrRun as i32);
      }

      let precallresult = luau_precall(l, first_arg.offset(-1), LUA_MULTRET);

      if (*l).status == SCHEDULED_REENTRY as u8 {
        first_arg = (*l).base;
      } else {
        if precallresult != PCRLUA {
          return;
        }

        (*(*l).ci).flags |= LUA_CALLINFO_RETURN as u32;
      }
    }

    if (*l).status != LuaStatus::Ok as u8 {
      LUAU_ASSERT!(first_arg >= (*l).base);
      LUAU_ASSERT!(isyielded(&*l));
      (*l).status = LuaStatus::Ok as u8;

      let cl = curr_func!(l);

      if (*cl).is_c != 0 {
        let c = addr_of!((*cl).inner.c).cast::<CClosure>();
        if (*c).cont.is_none() {
          luau_poscall(l, first_arg);
        } else {
          (*l).base = (*(*l).ci).base;
        }
      } else {
        (*l).base = (*(*l).ci).base;
      }
    }

    resume_continue(l);
  }
}
