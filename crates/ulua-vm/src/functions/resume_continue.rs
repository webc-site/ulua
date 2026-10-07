use core::ptr::addr_of;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    luau_execute::{luau_execute, tier_reentry_hot},
    luau_finishop::luau_finishop,
    luau_poscall::luau_poscall,
  },
  macros::{
    curr_func::curr_func, lua_callinfo_handle::LUA_CALLINFO_HANDLE,
    lua_callinfo_native::LUA_CALLINFO_NATIVE, lua_callinfo_opyield::LUA_CALLINFO_OPYIELD,
    scheduled_reentry::SCHEDULED_REENTRY,
  },
  records::{closure::CClosure, lua_state::LuaState},
};

/// 展开 Luau/C 混合栈、逐帧运行续体直至挂起/出错（cpp `resume_continue`）。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// `l` 须为存活协程状态，且 `resume_start`→驱动→本函数的成对协议已建立：ci 栈
/// 上仅含恢复流程构造的续体帧（C `cont` 或 Lua 主体），`base_ccalls == n_ccalls`
/// 由循环内 `LUAU_ASSERT!` 把守。
#[inline(always)]
pub(crate) unsafe fn resume_continue(l: *mut LuaState) {
  // r14-p3 逐点定性（w6d 口径保留面）：循环条件与续体后判定中 `==Ok as u8` 谓词、
  // SCHEDULED_REENTRY（0x7f）原值消费谓词两处均不可换 status() 门面——non-repr
  // 0x7f→Ok 兜底恰改这两类谓词真值（判例见 r13-w1b resume_finish Ok 谓词保留与
  // 本票收编判据「仅全部消费点为判别值谓词」），保留裸字段比较形；status 判别值
  // 写面（=Ok as u8）无门面保留；ccount 族断言、ci/base_ci 帧链现读与 ci flags 帧面
  // （HANDLE 清位、OPYIELD 读判）属帧 ABI 覆盖面外点位，保留。收编两处：续体返回
  // 后 Break/Yield 双判别值谓词经既有 status() 门面（w1b resume_finish 真值表论证
  // 同款）；结果窗起点地址经 top_slot(-n) 现读（本波裁决，见行前注）。
  unsafe {
    // unroll Luau/C combined stack, processing continuations
    while ((*l).status == LuaStatus::Ok as u8 || (*l).status == SCHEDULED_REENTRY as u8)
      && (*l).ci > (*l).base_ci
    {
      LUAU_ASSERT!((*l).base_ccalls == (*l).n_ccalls);

      (*l).status = LuaStatus::Ok as u8;

      let cl = curr_func!(l);

      if (*cl).is_c != 0 {
        // C continuation; we expect this to be followed by Lua continuations
        let c = addr_of!((*cl).inner.c).cast::<CClosure>();
        let cont_opt = (*c).cont;
        LUAU_ASSERT!(cont_opt.is_some());

        if let Some(cont) = cont_opt {
          // continuation can use non-protected calls again
          (*(*l).ci).flags &= !(LUA_CALLINFO_HANDLE as u32);

          let n = cont(l, 0);

          let status = (*l).status;
          if status == LuaStatus::Break as u8 || status == LuaStatus::Yield as u8 {
            break;
          }

          if (*l).status == SCHEDULED_REENTRY as u8 {
            continue;
          }

          // 收编（本波 r14-p3 裁决）：结果窗起点地址经 top_slot(-n) 边界原语——
          // top_slot 为 inline(always) 现读场域顶，位点不变即允许同位换形；禁
          // 预绑定/缓存（cont 可再入执行/搬栈，读数必须现读）。裁决援引：r12
          // 「恢复点后同形单点保留」前案在此收窄为「裸字段整体搬运保留、算术
          // 读数操作数面换形收编」，终形同 w1b resume_finish 顶槽读数收编与
          // lua_v_call_tm 的 top_slot(-n) 负偏移判例——偏移量逐位同值
          luau_poscall(l, (*l).top_slot(-(n as isize)));
        }
      } else {
        let ci_flags = (*(*l).ci).flags;

        if (ci_flags & LUA_CALLINFO_OPYIELD as u32) != 0 {
          luau_finishop(l);
          // OPYIELD 续体可再入执行/搬栈/换帧——重入口径与 cpp 同构，走全量
          // `luau_execute`（从 `L->ci` 全新重建状态）
          luau_execute(l);
        } else if (*l).singlestep || (ci_flags & LUA_CALLINFO_NATIVE as u32) != 0 {
          // 冷门（单步 / 原生入口）：旗谓词与 [`luau_execute`] 入口同款，交全口径
          luau_execute(l);
        } else {
          // 热路：三旗已排除且本分支无再入执行，`cl` 即当前帧闭包——直达派发层
          tier_reentry_hot(l, cl);
        }
      }
    }
  }
}
