//! Source: `VM/src/lstate.cpp:148-180` (hand-ported)

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    c_slice_mut, lua_d_realloc_ci::lua_d_realloc_ci, lua_d_reallocstack::lua_d_reallocstack,
    lua_f_close::lua_f_close,
  },
  macros::{
    api_check::api_check, basic_ci_size::BASIC_CI_SIZE, basic_stack_size::BASIC_STACK_SIZE,
    extra_stack::EXTRA_STACK, lua_minstack::LUA_MINSTACK, setnilvalue::setnilvalue,
  },
  records::lua_state::LuaState,
};

/// `lua_resetthread`：把已完成/挂起/出错的 coroutine 线程重置回可复用态。
/// 调用序契约（正确性，非内存安全；r16-v3 引用形前移，`l` 存活由 `&mut` 类型承载）：
/// `l` 须为非活动（`!(*l).isactive`）的 coroutine 线程，其 `stack`/`base_ci`/`ci`/`end_ci`/`size_ci`/`stacksize`
/// 自洽：`status != Ok` 时须已退到 `ci == base_ci`（`api_check`）；`lua_f_close` 关闭 open upvalue、
/// `lua_d_realloc_ci`/`luaD_reallocstack` 收缩并重排栈帧，末尾对 `stack..stack+stacksize` 逐槽置 nil。
/// cpp/VM/src/lstate.cpp:170 lua_resetthread。
pub fn lua_resetthread(l: &mut LuaState) {
  // SAFETY: 契约保证 `l` 为非活动协程且栈/帧场自洽，重置面仅覆写本线程自有场域与 base_ci 首帧
  //
  // r13-w1c 逐点定性（w6d 口径保留面·本票收编 0 点）：本体 `(*l).` 命中全部为
  // LuaState 状态字段读写字面——isactive/status/ci/base_ci/stack/size_ci/base/
  // n_ccalls/base_ccalls/stacksize 均无既有门面（LuaState 无 ccount/status/base
  // 写面；status() 门面自带 non-repr→Ok 兜底，仅对 Break 谓词逐位等价，本处
  // Ok 谓词与 `!= LuaStatus::Ok as u8` 不可换用——判例见 r13-w1b resume_finish/
  // lua_d_pcall 同款保留）。`(*l).base = (*(*l).ci).base` 帧建立落笔与 ci 链现读
  // 属帧 ABI 本体；CallInfo remap 三写（func/base/top）与 setnilvalue!((*ci).func)
  // 系 w7a1 红线保留面（其行前注记在案，不得翻案）。唯一栈顶算术点
  // `(*l).reanchor_top((*(*l).ci).base)` 系 r12-w9b 既有收编，本票不动其形制。
  unsafe {
    api_check!(l, !(*l).isactive);
    api_check!(
      l,
      (*l).status != LuaStatus::Ok as u8 || (*l).ci == (*l).base_ci
    );

    // close upvalues before clearing anything
    lua_f_close(l, (*l).stack);

    // clear call frames
    let ci = (*l).base_ci;
    (*ci).func = (*l).stack;
    (*ci).base = (*ci).func.add(1);
    // r12-w7a1 定性保留：`(*ci).top` 为 CallInfo 裸字段（帧可写界初始化），
    // 非 LuaState 栈顶槽算术——records/slot.rs 边界红线明载「CallInfo 裸字段与
    // 帧内算术不落句柄」，字段落库仍用裸 StkId，此处不收编。
    (*ci).top = (*ci).base.add(LUA_MINSTACK as usize);
    setnilvalue!((*ci).func);
    (*l).ci = ci;
    if (*l).size_ci != BASIC_CI_SIZE {
      lua_d_realloc_ci(l, BASIC_CI_SIZE);
    }
    // clear thread state
    (*l).status = LuaStatus::Ok as u8;
    (*l).base = (*(*l).ci).base;
    (*l).reanchor_top((*(*l).ci).base);
    (*l).n_ccalls = 0;
    (*l).base_ccalls = 0;
    // clear thread stack
    if (*l).stacksize != BASIC_STACK_SIZE + EXTRA_STACK {
      lua_d_reallocstack(l, BASIC_STACK_SIZE, 0);
    }
    // SAFETY:stack 数组长度为 stacksize（reallocstack 后仍保持一致）。
    for slot in c_slice_mut((*l).stack, (*l).stacksize as usize) {
      setnilvalue!(slot);
    }
  }
}
