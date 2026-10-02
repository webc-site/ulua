use core::{ffi::c_void, ptr::addr_of_mut};

use crate::{
  enums::lua_status::LuaStatus,
  functions::lua_d_rawrunprotected_ldo::lua_d_rawrunprotected,
  macros::{
    api_check::api_check, condhardstacktests::condhardstacktests,
    expandstacklimit::expandstacklimit, luai_maxcstack::LUAI_MAXCSTACK,
    stacklimitreached::stacklimitreached,
  },
  records::{call_context_lapi::CallContext, lua_state::LuaState},
};

/// `lua_checkstack`（cpp `lapi.cpp` 同名）：为调用方预留 `size` 层栈头寸，成功返 1、
/// 越界/扩不动返 0。调用序契约（正确性，非内存安全）：`l` 须为存活可栈操作 `LuaState`、
/// `size >= 0`（`api_check!` 兜底）；扩容路径经 `lua_d_rawrunprotected` 再入受保护帧或
/// `lua_d_reallocstack` 重建栈指针，`expandstacklimit!` 落笔 `ci->top`——裸指针面收进
/// 本实现体内（r16-v3 引用形前移，unsafe 不再外包给调用方）。
pub fn lua_checkstack(l: &mut LuaState, size: i32) -> i32 {
  // SAFETY: 契约保证 `l` 存活且 size 非负；top/base/stack_last 裸指针读写与受保护帧
  // 再入均属实现本体，跨调用不留旧栈指针。
  unsafe {
    api_check!(l, size >= 0);

    let mut res = 1;
    // r16-b2 收编：顶-基槽距读数落既有 get_top 门面——其本体 slot_distance(base, top)
    // 即被替代式 `top.offset_from(base) as i32` 的同址同宽镜像（谓词内现读、短路次序
    // 不变）；isize→i32 折形在现域无截差（栈槽距受 LUAI_MAXSTACK 约束），与 i32 常量
    // LUAI_MAXCSTACK 的加比较两侧同型、方向逐位等价
    if size > LUAI_MAXCSTACK || (l.get_top() + size) > LUAI_MAXCSTACK {
      res = 0; // stack overflow
    } else if size > 0 {
      if stacklimitreached(&*l, size) {
        let mut ctx = CallContext { size };
        // there could be no memory to extend the stack
        if lua_d_rawrunprotected(
          l,
          Some(CallContext::run_mut),
          addr_of_mut!(ctx) as *mut c_void,
        ) != LuaStatus::Ok as i32
        {
          return 0;
        }
      } else {
        condhardstacktests!(lua_d_reallocstack(l, (*l).stacksize - EXTRA_STACK, 0));
      }

      // 扩容后按新栈顶经 `top_slot` 读数原语求目标界缘槽地址（reallocstack 已在
      // 上方分支完成，此处重读与原 `top.add(size)` 求值位点一致），再交宏抬 ci 可写界
      let wanttop = (*l).top_slot(size as isize);
      expandstacklimit!(l, wanttop);
    }
    res
  }
}
