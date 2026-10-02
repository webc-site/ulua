/// NUL 结尾字节串（`*const c_char` 契约调用点 `.as_ptr().cast()`；§10 不引入 C 字符串类型）。
use core::ptr::addr_of_mut;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_c_barrierback::lua_c_barrierback, resume_error::resume_error},
  macros::{api_check::api_check, isblack::isblack, luai_maxccalls::LUAI_MAXCCALLS},
  records::{gc_object::GCObject, lua_state::LuaState},
};
const ERR_NOT_SUSPENDED: &[u8] = b"cannot resume non-suspended coroutine\0";
const ERR_C_STACK_OVERFLOW: &[u8] = b"C stack overflow\0";

/// 协程恢复的入口校验与 ccount 建档（cpp `resume_start`，与 `resume_finish` 成对）。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// `l` 须为待恢复协程状态且栈上实参不少于 `nargs`（由本函数 `api_check!` 核验）；
/// `from` 为发起恢复的外层协程，允许为空（主状态直接恢复无外层），非空时须存活。
pub(crate) unsafe fn resume_start(l: *mut LuaState, from: *mut LuaState, nargs: i32) -> i32 {
  // r14-p2 逐点定性（w6d 口径保留面）：判别值谓词之后同式的裸 `!=0` 谓词（0x7f→Ok 兜底
  // 改写其真值，w1b 红线）与 ci/base_ci 帧面同形校验、ccount 族（n_ccalls 读写递增、
  // base_ccalls 建档、isactive 落笔——LuaState 无 ccount/isactive 门面）、gclist 写侧
  // 取址（GC 链字段无门面，barrierback 入参即字段地址本体）均无既有门面，全数 C 保留；
  // resume_error 实参窗与返回面为帧建立/动作本体，照旧不碰。
  // 收编共三点：栈槽距校验读数落既有 get_top 门面，Yield/Break 两处判别值谓词经
  // 既有 status() 门面（真值表照 resume_finish r13-w1b 判例抄形，见行内注）。
  unsafe {
    api_check!(l, nargs >= 0);
    // 收编：顶-基槽距读数落既有 get_top 门面——其本体 slot_distance(base, top) 即
    // 被替代式 `top.offset_from(base) as i32` 的同址同宽镜像（现读位点不变）；
    // isize→i32 折形在现域无截差（协程栈槽距受 LUAI_MAXSTACK 约束、远小于 i32::MAX），
    // 比较两侧同为 i32 后与原 isize 式同真值；api_check! 系 debug 期断言，release
    // 编译掉后行为恒等
    api_check!(l, (*l).get_top() >= nargs);

    // 收编：Yield/Break 判别值谓词经既有 status() 门面（from_repr 对 1..6 逐值精确；
    // non-repr 值含 0x7f 兜底 Ok 亦 ≠Yield/≠Break——谓词真值表在全部 u8 值域逐位等价，
    // 照 resume_finish r13-w1b 同款论证；下一行的裸 `!=0` 谓词域外值真值相反，不可
    // 换用，原样保留）
    if (*l).status() != LuaStatus::Yield
      && (*l).status() != LuaStatus::Break
      && ((*l).status != 0 || (*l).ci != (*l).base_ci)
    {
      return resume_error(l, ERR_NOT_SUSPENDED.as_ptr().cast(), nargs);
    }

    (*l).n_ccalls = if !from.is_null() { (*from).n_ccalls } else { 0 };
    if (*l).n_ccalls as i32 >= LUAI_MAXCCALLS {
      return resume_error(l, ERR_C_STACK_OVERFLOW.as_ptr().cast(), nargs);
    }

    (*l).n_ccalls = (*l).n_ccalls.wrapping_add(1);
    (*l).base_ccalls = (*l).n_ccalls;
    (*l).isactive = true;

    let o = l as *mut GCObject;
    if isblack!(o) {
      lua_c_barrierback(&mut *l, o, addr_of_mut!((*l).gclist));
    }

    LuaStatus::Ok as i32
  }
}
