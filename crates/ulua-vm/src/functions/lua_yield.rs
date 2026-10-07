use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_throw_ldo::lua_d_throw, lua_g_pusherror::lua_g_pusherror_bytes},
  macros::api_check::api_check,
  records::lua_state::LuaState,
};

const ERR_YIELD_ACROSS_C_CALL: &[u8] = b"attempt to yield across metamethod/C-call boundary";

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline(always)]
pub unsafe fn lua_yield(l: *mut LuaState, nresults: i32) -> i32 {
  unsafe {
    api_check!(l, nresults >= 0);
    // r16-b2 收编：顶-基槽距读数落既有 get_top 门面（本体 slot_distance(base, top) 即
    // 被替代式同址同宽镜像）。api_check! 系 debug 期断言、release 编译掉 ⇒ 换形等价
    // 平凡真（r14 p1 判例②）；nresults 本为 i32，isize 式与 i32 式在现域逐值同真值、
    // 比较方向不变
    api_check!(l, nresults <= (*l).get_top());

    if (*l).n_ccalls > (*l).base_ccalls {
      lua_g_pusherror_bytes(&mut *l, ERR_YIELD_ACROSS_C_CALL);
      lua_d_throw(l, LuaStatus::ErrRun as i32);
    }

    // 保留（yield 结果窗恢复点·同形单点）：上方错误路径可抛，非错误路径此处为对
    // 场域的唯一一次现读——cpp 同形 `L->base = L->top - nresults;` 单点最小读面
    (*l).base = (*l).top.offset(-(nresults as isize));
    (*l).status = LuaStatus::Yield as u8;
    -1
  }
}
