use core::ffi::c_uchar;

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{luau_assert::LUAU_ASSERT, luau_insn_ops::luau_insn_op},
};

use crate::{
  functions::{c_slice, c_slice_mut, lua_g_getline::lua_g_getline},
  macros::lua_m_newarray::luaM_newarray,
  records::{lua_state::LuaState, proto::Proto},
  type_aliases::instruction::Instruction,
};

/// C++ `void luaG_breakpoint(LuaState* l, Proto* p, int line, bool enable)`.
///
/// 写侧保持 `*mut Proto` 裸句柄（不强行 safe 化造假）：`luaM_newarray!` 扩容可能触发
/// GC/抛错，`ondisable` 宿主回调以 `(l, p)` 重入 VM——两者都可能移动/改写运行期状态，
/// `&mut Proto` 排他借用跨这些点不可成立，旧指针失效类契约只能在边界上以文档收口。
///
/// # Safety
/// `l` 必须指向存活 `LuaState`（含非空 `global`）；`p` 必须指向存活 `Proto` 且在整个调用
/// 窗口内存活——包括 `luaM_newarray!` 分配（可 GC/抛错）与 `ondisable(l, p)` 回调重入之后；
/// `code`/`sizecode` 自洽，`debuginsn` 为空或覆盖 `sizecode` 项；`lineinfo` 满足
/// `lua_g_getline` 前置；`execdata` 非空时宿主计数器状态有效。
pub(crate) unsafe fn lua_g_breakpoint(l: *mut LuaState, p: *mut Proto, line: i32, enable: bool) {
  // SAFETY: 契约保证 `l`/`p` 存活贯穿调用；块内对 `code`/`debuginsn` 的原位写全部经
  // `c_slice(_mut)` 折叠为切片下标写，且派生借用在跨分配/跨回调点之前终止（指针重取）。
  unsafe {
    let ondisable = (*l).gs_ref().ecb.disable;

    if !(*p).lineinfo.is_null() && (ondisable.is_some() || (*p).execdata.is_null()) {
      // 先扫后改：命中下标经 find_map 按值带出，code 的共享切片视图在下标确定后即止，
      // 之后的原位写不再与任何派生切片借用重叠（消除原「边遍历边裸写」的别名冲突）。
      // 判定顺序与 cpp/luaG_breakpoint 逐位一致：PREPVARARGS 先短路跳过（该行不调
      // lua_g_getline），首个同行指令命中后只改一处并停止（原循环的 break）。
      let sizecode = (*p).sizecode as usize;
      let hit = c_slice((*p).code, sizecode)
        .iter()
        .enumerate()
        .find_map(|(i, &insn)| {
          (luau_insn_op(insn) != LuauOpcode::LOP_PREPVARARGS as u32
            && lua_g_getline(&*p, i as i32) == line)
            .then_some(i)
        });

      if let Some(i) = hit {
        if (*p).debuginsn.is_null() {
          // 分配可能触发 GC/抛错：旧 `debuginsn`/`code` 借用不可跨此点存续，故写入前先重取
          (*p).debuginsn = luaM_newarray!(l, (*p).sizecode, c_uchar, (*p).hdr.memcat);
          // debuginsn 快照：源 code 切片 zip 目标，单次遍历
          for (d, &snap) in c_slice_mut((*p).debuginsn, sizecode)
            .iter_mut()
            .zip(c_slice((*p).code, sizecode))
          {
            *d = luau_insn_op(snap) as c_uchar;
          }
        }

        let op = if enable {
          LuauOpcode::LOP_BREAK as u32
        } else {
          // i 源自本函数按 sizecode 扫出的合法指令下标，debuginsn 恒与之等长
          c_slice((*p).debuginsn, sizecode)[i] as u32
        };

        // 原位 patch：只动 opcode 字节、保留参数位（cpp `p->code[i] &= ~0xff; |= op`）
        let code = c_slice_mut((*p).code, sizecode);
        code[i] &= !(0xff as Instruction);
        code[i] |= op as Instruction;
        LUAU_ASSERT!(luau_insn_op(code[i]) == op);

        if enable
          && !(*p).execdata.is_null()
          && let Some(ondisable) = ondisable
        {
          // 宿主回调以裸句柄重入 VM（运行期开放边界），派生自 `code` 的借用已在上方止息
          ondisable(l, p);
        }
      }
    }

    for &sub in c_slice((*p).p, (*p).sizep as usize) {
      lua_g_breakpoint(l, sub, line, enable);
    }
  }
}
