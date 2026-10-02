use core::ptr::{addr_of, null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::value_view::ValueView,
  functions::lua_d_check_cstack::lua_d_check_cstack,
  macros::{
    incr_ci::incr_ci, lua_d_checkstack::luaD_checkstack, lua_minstack::LUA_MINSTACK,
    luai_maxccalls::LUAI_MAXCCALLS, setnilvalue::setnilvalue, setobj_2_s::setobj_2_s,
  },
  records::{
    closure::{CClosure, Closure},
    lua_state::LuaState,
  },
  type_aliases::t_value::TValue,
};

/// C++ `LUAU_NOINLINE void luaV_callTM(LuaState* l, int nparams, int res)`.
/// # Safety
/// `l` 必须指向存活 `LuaState`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
#[inline(never)] // 对应 cpp LUAU_NOINLINE：控制栈帧体积，勿内联
pub unsafe fn lua_v_call_tm(l: *mut LuaState, nparams: i32, res: i32) {
  // SAFETY: 契约保证 `l` 为存活调用帧、tm/左值/右值按约定可读可写，块内重入调用不越本帧栈界
  //
  // r13-w1b 逐点定性（w6d 口径保留面）：n_ccalls 增减两处读数、stack_last 界缘
  // 断言、`(*l).base` 帧建立/回退落笔与 `(*l).ci` 现读回退均无 LuaState 门面，
  // 保留；帧回退三件套（ci/base/top 一并回读父帧 cip 场域）系既有 B2-2b 判例保留
  // 注记（不得翻案，见其行前注）。收编三处 top 读数：入口顶基址、建立面契约断言
  // 的顶侧操作数、结果拷贝消费窗（均 top_slot 原语，位点现读不变）。
  unsafe {
    (*l).n_ccalls += 1;

    if (*l).n_ccalls >= LUAI_MAXCCALLS as u16 {
      lua_d_check_cstack(l);
    }

    luaD_checkstack!(l, LUA_MINSTACK);

    // 收编：顶槽地址读数经 top_slot(0) 边界原语（镜像 cpp `StkId top = L->top;`
    // 绑定形——luaD_checkstack! 扩容先行之后原位刻读，与旧点位同位同值）
    let top = (*l).top_slot(0);
    let fun = top.sub(nparams as usize).sub(1);

    let ci = incr_ci!(l);
    (*ci).func = fun;
    (*ci).base = fun.add(1);
    // 保留：CallInfo 帧建立面场域写（`top` 已是本函数体上方单次预绑定的窗读，
    // 无可收编重读）；与豁免面 luau_setupcci/luau_callhook 同款建立点，属帧 ABI 本体
    (*ci).top = top.add(LUA_MINSTACK as usize);
    (*ci).savedpc = null_mut();
    (*ci).flags = 0;
    (*ci).nresults = if res >= 0 { 1 } else { 0 };
    LUAU_ASSERT!((*ci).top <= (*l).stack_last);

    // cpp `lua_assert(ttisfunction(ci->func))` + `lua_assert(clvalue(ci->func)->isc)`：
    // §11 pass B 把这段 tag→payload 读链收敛为一次 [`ValueView`] 读取——Function 变体本身
    // 即 `ttisfunction` 判据，`is_c` 断言在臂内。随后只透传 `*const Closure` 裸指针：被调
    // C 闭包可写闭包体、可触发 GC 甚至搬栈，视图借用不外溢（同 value_view 寿命契约）。
    let cl = match ValueView::from_tvalue(&*fun) {
      ValueView::Function(cl) => {
        LUAU_ASSERT!(cl.is_c != 0);
        cl as *const Closure
      }
      // 非函数 tag：cpp 由上述 lua_assert 兜底，release 下仍按 `clvalue!` 读 union 成员；
      // 断言原样保留，故该臂与旧链在 debug 下行为一致
      _ => {
        LUAU_ASSERT!((*fun).is_function());
        (*fun).as_closure_ptr()
      }
    };

    (*l).base = fun.add(1);
    // 收编：断言顶侧读数经 top_slot(0) 原语（同位现读、值恒等）；base 侧无门面，
    // 按 w6d 口径保留裸读
    LUAU_ASSERT!((*l).top_slot(0) == (*l).base.add(nparams as usize));

    let c = addr_of!((*cl).inner.c).cast::<CClosure>();
    let func = (*c).f;
    // f 非空由 CClosure 构造契约保证（lua_pushcclosure 必设 .f，cpp luaD_callnoyield 同款直接调用）
    let n = func.expect("CClosure.f 由 lua_pushcclosure 构造契约保证非空")(l);
    LUAU_ASSERT!(n >= 0); // yields should have been blocked by n_ccalls

    // ci is our callinfo, cip is our parent
    // note that we read l->ci again since it may have been reallocated by the call
    let cip = (*l).ci.sub(1);

    // copy return value into parent stack
    if res >= 0 {
      if n > 0 {
        setobj_2_s!(
          l,
          (*cip).base.add(res as usize),
          // 收编：源侧裸偏移读数经 top_slot(-n) 边界原语——func(l) 可搬栈，本点位
          // 即派即用现读场域顶（同 loadsafe/resolve_import_safe 的 r12-w9b 判例，
          // 不预绑定窗）；契约 `n >= 0` 断言先行下与原 `top.sub(n)` 逐位同值
          (*l).top_slot(-(n as isize)) as *const TValue
        );
      } else {
        setnilvalue!((*cip).base.add(res as usize));
      }
    }

    (*l).ci = cip;
    (*l).base = (*cip).base;
    // 保留：帧回退三件套的恢复动作本体（ci/base/top 一并回读父帧 cip 场域，
    // 时序即正确性）；无算术操作数可收编，形制同 copy_results_pop_frame 的
    // B2-2b 保留裁决
    (*l).top = (*cip).top;

    (*l).n_ccalls -= 1;
  }
}
