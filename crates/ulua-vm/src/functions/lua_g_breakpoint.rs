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

/// # Safety
/// `l` 必须指向存活 `LuaState`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
/// C++ `void luaG_breakpoint(LuaState* l, Proto* p, int line, bool enable)`.
pub(crate) unsafe fn lua_g_breakpoint(l: *mut LuaState, p: *mut Proto, line: i32, enable: bool) {
  // Safety: 契约保证 `l` 为存活调用帧；块内仅构造 breakpoint 错误对象并经 luaG 抛出路径处理，不返回
  unsafe {
    let ondisable = (*(*l).global).ecb.disable;

    if !(*p).lineinfo.is_null() && (ondisable.is_some() || (*p).execdata.is_null()) {
      // 先扫后改：命中下标经 find_map 按值带出，code 的共享切片视图在下标确定后即止，
      // 之后的原位写不再与任何派生切片借用重叠（消除原「边遍历边裸写」的别名冲突）。
      // 判定顺序与 cpp/luaG_breakpoint 逐位一致：PREPVARARGS 先短路跳过（该行不调
      // lua_g_getline），首个同行指令命中后只改一处并停止（原循环的 break）。
      let hit = c_slice((*p).code, (*p).sizecode as usize)
        .iter()
        .enumerate()
        .find_map(|(i, &insn)| {
          (luau_insn_op(insn) != LuauOpcode::LOP_PREPVARARGS as u32
            && lua_g_getline(p, i as i32) == line)
            .then_some(i)
        });

      if let Some(i) = hit {
        if (*p).debuginsn.is_null() {
          (*p).debuginsn = luaM_newarray!(l, (*p).sizecode, c_uchar, (*p).hdr.memcat);
          // debuginsn 快照：源 code 切片 zip 目标，单次遍历
          for (d, &snap) in c_slice_mut((*p).debuginsn, (*p).sizecode as usize)
            .iter_mut()
            .zip(c_slice((*p).code, (*p).sizecode as usize))
          {
            *d = luau_insn_op(snap) as c_uchar;
          }
        }

        let op = if enable {
          LuauOpcode::LOP_BREAK as u32
        } else {
          // i 源自本函数按 sizecode 扫出的合法指令下标，debuginsn 恒与之等长
          *(*p).debuginsn.add(i) as u32
        };

        *(*p).code.add(i) &= !(0xff as Instruction);
        *(*p).code.add(i) |= op as Instruction;
        LUAU_ASSERT!(luau_insn_op(*(*p).code.add(i)) == op);

        if enable
          && !(*p).execdata.is_null()
          && let Some(ondisable) = ondisable
        {
          ondisable(l, p);
        }
      }
    }

    for &sub in c_slice((*p).p, (*p).sizep as usize) {
      lua_g_breakpoint(l, sub, line, enable);
    }
  }
}
