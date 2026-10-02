//! Source: `VM/src/lvmexecute.cpp:147-200` (hand-ported)

use core::{ffi::c_void, mem::zeroed, ptr::null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::lua_g_getline::lua_g_getline,
  macros::{
    lua_d_checkstack::luaD_checkstack, lua_minstack::LUA_MINSTACK, pc_rel::pcRel,
    restorestack::restorestack, savestack::savestack,
  },
  records::{lua_debug::LuaDebug, lua_state::LuaState},
  type_aliases::lua_hook::LuaHook,
};

/// C++ `LUAU_NOINLINE void luau_callhook(LuaState* l, lua_Hook h, void* userdata)`.
/// `userdata` 为 hook 透传的用户数据；`None` 对应 cpp 传 `nullptr`（`ar.userdata` 置空）。
///
/// 帧协议台账（B2-2b）：本函数是「偏移暂存+边界一处重建」形态——savestack!/
/// restorestack! 已收口为 safe 数值互转 fn（`macros/savestack.rs`/`macros/restorestack.rs`，
/// 函数体无解引用），base/top/ci_top 全程以 isize 暂存、hook 重入后同批重建，块内
/// 无跨扩容存活的裸栈指针。残余裸解引用限于 `(*l)`/`(*ci)` 字段读写（CallInfo 裸字段
/// 布局为 `records/call_info.rs` DELIBERATE DEVIATION 裁决，§9.4），故保留 `unsafe fn`。
/// # Safety
/// 调用方须保证：`l` 处于可承接抛错/yield 的受保护帧（hook 内部可重入 VM）；`(*l).ci` 帧存活，
/// base/top/ci.top 均为可被 savestack!/restorestack! 重定位的栈指针，且本函数内 luaD_checkstack
/// 扩容后旧栈指针即失效（故全部以 int 偏移保存）；`hook` 遵守 lua_Hook 协议可安全接收 (l, ar)。
/// cpp lvmexecute.cpp:165 `luau_callhook`
#[inline(never)]
pub unsafe fn luau_callhook(l: *mut LuaState, hook: LuaHook, userdata: Option<*mut c_void>) {
  // SAFETY: 契约保证 `l` 存活且 savestack! 保存的 base/top/ci_top 在 hook 重入调用后仍可恢复，块内不保留跨调用的裸栈指针
  //
  // r14-p3 逐点定性（w6d 口径保留面）：base 帧窗读数（入口偏移暂存参数）与三处
  // 落笔（ci 帧基址拷贝、两处恢复落值）、ci 链与帧面现读读写（func/savedpc/top
  // 各点）、stack_last 界缘断言、top 写面（restorestack 落值）与帧落笔行 RHS 顶读
  //（resume_finish:92 同形单点保留判例，不翻案）均无既有门面，保留。收编两处形：
  // 入口顶槽偏移暂存读数经 top_slot(0) 边界原语——读数位在 luaD_checkstack 扩容
  // 之前，只换形不移位，暂存偏移仍对应旧栈；status 快照与其后比较转 status()
  // 枚举谓词面——全部消费点均为 Yield/Break 判别值 ==/!= 谓词，non-repr 0x7f→Ok
  // 兜底不改谓词真值（w1b resume_finish Break 谓词论证同款），未发现 `!=0`/
  // `==Ok`/原值回传消费形，判据成立；三处 status 写面（清零与两处判别值写）无
  // 门面，原样保留。
  unsafe {
    let base = savestack!(l, (*l).base);
    // 收编：顶槽偏移暂存读数经 top_slot(0) 边界原语（off=0 即顶槽，同址同值的
    // 现读镜像；位点保持在扩容之前不动）
    let top = savestack!(l, (*l).top_slot(0));
    let ci_top = savestack!(l, (*(*l).ci).top);
    // 收编：状态快照落既有 status() 门面（本体单读 status 字段 + from_repr 兜底，
    // 快照位点现读不变；谓词面转换见上方定性注记）
    let status = (*l).status();

    // if the hook is called externally on a paused thread, we need to make
    // sure the paused thread can emit Luau calls
    if status == LuaStatus::Yield || status == LuaStatus::Break {
      (*l).status = 0;
      (*l).base = (*(*l).ci).base;
    }

    let cl = (*(*(*l).ci).func).as_closure_ptr();

    // note: the pc expectations of the hook are matching the general "pc
    // points to next instruction"; however, for the hook to be able to
    // continue execution from the same point, this is called with savedpc at
    // the *current* instruction. this needs to be called before
    // luaD_checkstack in case it fails to reallocate stack
    //
    // savedpc 单读单写：end 比较折叠为同帧局部暂存（本区间无再入点，ci 不会换位），
    // 端指针仅由 code+sizecode 计数派生、从不解引用（参照 B1a getimport/getline
    // 端指针收口先例；pcRel! 调试读数走 code 基址位置差，语义不变）
    let ci = (*l).ci;
    let oldsavedpc = (*ci).savedpc;
    if !oldsavedpc.is_null() {
      let code_end = {
        let l = &(*cl).inner.l;
        (*l.p).code.wrapping_add((*l.p).sizecode as usize)
      };
      if oldsavedpc != code_end {
        (*ci).savedpc = oldsavedpc.add(1);
      }
    }

    luaD_checkstack!(l, LUA_MINSTACK); // ensure minimum stack size
    (*(*l).ci).top = (*l).top.add(LUA_MINSTACK as usize);
    LUAU_ASSERT!((*(*l).ci).top <= (*l).stack_last);

    let mut ar: LuaDebug = zeroed();
    ar.currentline = if (*cl).is_c != 0 {
      -1
    } else {
      let p = {
        let l = &(*cl).inner.l;
        l.p
      };
      // pcRel! 需按裸指针比对 code 基址，仍传 `p`；取行号本身降共享引用（纯读）
      lua_g_getline(&*p, pcRel!((*(*l).ci).savedpc, p))
    };
    // 既有约定（review.md §2）：VM c-API 边界 userdata POD 实参——无 hook 上下文时 null 为 LuaDebug.userdata 合法值，从不解引用，边界体内保留
    ar.userdata = userdata.unwrap_or(null_mut());

    if let Some(hook) = hook {
      hook(l, &mut ar);
    }

    // hook 可再入 VM 改变 l->ci：ci 字段写回按 `(*l).ci` 逐次重读（cpp 同纪律，
    // 不提升缓存）
    (*(*l).ci).savedpc = oldsavedpc;

    (*(*l).ci).top = restorestack!(l, ci_top);
    (*l).top = restorestack!(l, top);

    // note that we only restore the paused state if the hook hasn't yielded by itself
    // 收编：快照与两处现读均转 status() 枚举谓词面——==/!= 判别值谓词在 non-repr
    // →Ok 兜底两侧同真值（0x7f 既非 Yield 亦非 Break），钩子后再现读位点保持现读场域
    if status == LuaStatus::Yield && (*l).status() != LuaStatus::Yield {
      (*l).status = LuaStatus::Yield as u8;
      (*l).base = restorestack!(l, base);
    } else if status == LuaStatus::Break {
      LUAU_ASSERT!((*l).status() != LuaStatus::Break); // hook shouldn't break again

      (*l).status = LuaStatus::Break as u8;
      (*l).base = restorestack!(l, base);
    }
  }
}
