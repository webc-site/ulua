use core::{ffi::c_char, ptr::null};

use crate::{
  functions::lua_getlocal::resolve_local,
  macros::{api_check::api_check, getstr::getstr, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
};

/// r16-v24 收形：首参转 `&mut LuaState`——`l` 的存活与独占由类型承载。体内保留点：
/// `resolve_local` 转调以一次 `&mut *l` 就地重建引用（借用窗仅在当句内，仓内既有判例
/// 形制）；`(*ci).base.offset(…)` 帧窗基址裸读与 `getstr((*var).varname)` 局部变量名
/// 裸读为 r13-w1c 逐点定性保留面，原形原位保留。`api_check!` 宏（release 编译掉的
/// debug 断言）与 `setobj_2_s!` 宏（仅取 `(*$l).global` 纯指针字段读）对 `l` 的取用
/// 经隐式 deref 同址同宽、时序不变；`get_top`/`top_slot`/`reanchor_top` 三处方法调用的
/// 接收者由 `(*l).` 等价改写为 `l.`（`&mut` 隐式 deref 同义，避 `clippy::explicit_auto_deref`，
/// 位点与时序全数不变）。以上真实裸指针操作的前提由调用方给出，故 `unsafe fn` 屏障
/// 保留、不降为安全 `fn`（判例同 r16-v21 lua_touserdatatagged_ref）。
///
/// # Safety
///
/// `l` 的存活与独占由 `&mut LuaState` 承载；`level`/`n` 须满足 C 参考实现的前置条件，
/// 且待写入值已压在栈顶。
pub unsafe fn lua_setlocal(l: &mut LuaState, level: i32, n: i32) -> *const c_char {
  // SAFETY: 契约保证当前帧至少 1 个可写槽且 level 在调用栈深度内（`l` 的存活/独占由
  // `&mut LuaState` 承载），变量名/valid 读取受 proto 局部信息界定
  unsafe {
    // r16-b2 收编：顶-基槽距读数落既有 get_top 门面（本体 slot_distance(base, top) 即
    // 被替代式同址同宽镜像）。api_check! 系 debug 期断言、release 编译掉 ⇒ 换形等价
    // 平凡真（r14 p1 判例②）；isize→i32 在现域无截差、比较方向不变
    api_check!(l, l.get_top() >= 1);

    // var 为 null 时同样弹栈：cpp 原版无条件 pop，返回值仅为变量名
    let Some((ci, var)) = resolve_local(&mut *l, level, n) else {
      return null();
    };

    // 栈顶待写入值的单槽窗口经 `top_slot` 读数原语一次预绑定
    // （resolve_local 纯读帧信息，不改栈顶字段）
    let value = l.top_slot(-1);
    if !var.is_null() {
      setobj_2_s!(l, (*ci).base.offset((*var).reg as isize), value);
    }

    l.reanchor_top(value); // pop value

    if !var.is_null() {
      getstr((*var).varname)
    } else {
      null()
    }
  }
}
