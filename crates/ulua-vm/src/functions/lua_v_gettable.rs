//! Source: `VM/src/lvmutils.cpp:102-180` (hand-ported)

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::tms::TMS,
  functions::{
    call_t_mres::call_t_mres, lua_g_indexerror::lua_g_indexerror,
    lua_g_missingmembererror::lua_g_missingmembererror, lua_h_get::lua_h_get,
    lua_t_gettmbyobj::lua_t_gettmbyobj,
  },
  macros::{
    classvalue::classvalue, fasttm::fasttm, gval_2_slot::gval2slot, lua_g_runerror::lua_g_runerror,
    lua_o_nilobject::LUA_O_NILOBJECT, lua_r_lookupmemberatoffset::luaR_lookupmemberatoffset,
    maxtagloop::MAXTAGLOOP, objectvalue::objectvalue, setobj_2_s::setobj_2_s,
  },
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 指向存活 `LuaState`；`t`/`key` 指向对齐可读的 `TValue`（`key` 取 `*mut` 与 cpp
/// 签名一致，实际只读）；`val` 为当前帧可写栈槽且栈顶余一次 TM 调用空间（`call_t_mres`
/// 会经该帧压栈）。cpp lvmutils.cpp:196.
pub unsafe fn lua_v_gettable(l: *mut LuaState, mut t: *const TValue, key: *mut TValue, val: StkId) {
  // 局部 const 指针即可满足全部读取点，收敛参数处的 `*mut` 回转
  let key = key as *const TValue;
  // Safety: 契约保证 `l` 为存活调用帧、TValue/StkId 指针可读/可写且对齐，块内取值、TM 调用与报错路径均在该帧栈界内
  unsafe {
    // 保留计数重复：MAXTAGLOOP 是 __index 链防失控的重试预算上限，不是数组下标；
    // 每轮沿元方法链把 t 换成下一级 TM 对象再走，无可迭代的数据序列
    for _ in 0..MAXTAGLOOP {
      let tm: *const TValue;
      if (*t).is_table() {
        let h = (*t).as_table_ptr();

        let res = lua_h_get(h, key);

        if res != LUA_O_NILOBJECT {
          (*l).cachedslot = gval2slot!(h, res);
        }

        if !(*res).is_nil() {
          setobj_2_s!(l, val, res);
          return;
        }

        tm = fasttm(l, (*h).metatable, TMS::TmIndex);
        if tm.is_null() {
          setobj_2_s!(l, val, res);
          return;
        }
      } else if fflag::DebugLuauUserDefinedClassesRuntime.get() && (*t).is_object() {
        let inst = objectvalue!(t);
        let offsettval = lua_h_get((*(*inst).lclass).memberstooffset, key);

        if (*offsettval).is_nil() {
          lua_g_missingmembererror(l, t, key);
        }

        LUAU_ASSERT!((*offsettval).is_number());
        let offset = (*offsettval).as_number() as i32;
        setobj_2_s!(l, val, luaR_lookupmemberatoffset!(inst, offset));
        return;
      } else if fflag::DebugLuauUserDefinedClassesRuntime.get() && (*t).is_class() {
        let lco = classvalue!(t);
        let res = lua_h_get((*lco).memberstooffset, key);

        if (*res).is_nil() {
          lua_g_missingmembererror(l, t, key);
        }

        LUAU_ASSERT!((*res).is_number());
        let offset = (*res).as_number() as i32;
        LUAU_ASSERT!(offset >= 0 && offset < (*lco).numberofallmembers);

        if offset < (*lco).numberofinstancemembers {
          lua_g_missingmembererror(l, t, key);
        }

        let static_idx = (offset - (*lco).numberofinstancemembers) as usize;
        let static_slot = (*lco).staticmembers.add(static_idx);

        setobj_2_s!(l, val, static_slot);
        return;
      } else {
        tm = lua_t_gettmbyobj(l, t, TMS::TmIndex);
        if (*tm).is_nil() {
          lua_g_indexerror(l, t, key);
        }
      }

      if (*tm).is_function() {
        call_t_mres(l, val, tm, t, key);
        return;
      }
      t = tm;
    }
    lua_g_runerror!(l, "'__index' chain too long; possible loop");
  }
}

/// # Safety
/// C ABI 导出壳：逐参数原样透传，前置条件与 [`lua_v_gettable`] 相同。
pub unsafe extern "C-unwind" fn lua_v_gettable_export(
  l: *mut LuaState,
  t: *const TValue,
  key: *mut TValue,
  val: StkId,
) {
  // Safety: 导出壳原样转发同契约 `lua_v_gettable`；l/t/key/val 满足其前置
  unsafe {
    lua_v_gettable(l, t, key, val);
  }
}
