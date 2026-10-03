//! Source: `VM/src/lvmutils.cpp:102-180` (hand-ported)

use core::{mem::MaybeUninit, ptr::null_mut};

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::tms::TMS,
  functions::{
    call_t_mres::call_t_mres,
    index_chain_cache::{INDEX_CHAIN_MAX, index_chain_fill, index_chain_probe},
    lua_g_indexerror::lua_g_indexerror,
    lua_g_missingmembererror::lua_g_missingmembererror,
    lua_h_get::lua_h_get,
    lua_t_gettmbyobj::lua_t_gettmbyobj,
  },
  macros::{
    classvalue::classvalue, fasttm::fasttm, gval_2_slot::gval2slot, lua_g_runerror::lua_g_runerror,
    lua_o_nilobject::LUA_O_NILOBJECT, lua_r_lookupmemberatoffset::luaR_lookupmemberatoffset,
    maxtagloop::MAXTAGLOOP, objectvalue::objectvalue, setobj_2_s::setobj_2_s,
  },
  records::{lua_state::LuaState, lua_table::LuaTable, slot::Slot, t_string::tstring},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 指向存活 `LuaState`；`t`/`key` 为对齐可读的槽句柄（只读侧：本函数经其仅使用
/// 读面，`key` 原 cpp 签名取 `*mut` 实际只读）；`val` 为当前帧可写栈槽句柄且栈顶余
/// 一次 TM 调用空间（`call_t_mres` 会经该帧压栈）。句柄跨调用存续服从「扩容先行、
/// 借用后派生」不变量——本函数内不重取栈基址，槽地址在调用期间不得迁移。
/// cpp lvmutils.cpp:196.
pub unsafe fn lua_v_gettable(l: *mut LuaState, mut t: Slot<'_>, key: Slot<'_>, val: Slot<'_>) {
  // SAFETY: 契约保证 `l` 为存活调用帧、槽句柄可读/可写且对齐，块内取值、TM 调用与报错路径均在该帧栈界内
  unsafe {
    // ---- J4 多点链缓存：资格前置（首表 + 字符串键 + 非空元表）一次判定，probe 与
    // 慢路伴随记录共用；任一不满足则原路直行，零行为差异 ----
    let mut rec_ok = t.get().is_table() && key.get().is_string();
    let mut mt0: *mut LuaTable = null_mut();
    let mut k: *mut tstring = null_mut();
    let mut t0: *mut LuaTable = null_mut();
    if rec_ok {
      let h = t.get().as_table_ptr();
      k = key.get().as_string_ptr();
      mt0 = (*h).metatable;
      rec_ok = !mt0.is_null();
      t0 = h;
      // 命中即「恒等守卫走查 + owner 单次哈希」，写回与 cachedslot 回填同原慢路语义
      if rec_ok && index_chain_probe(l, h, k, val.as_ptr()) {
        return;
      }
    }
    // 慢路伴随记录：逐级解析到的链表（全表链走到非 nil 命中才回填）。
    // MaybeUninit：仅前 rec_len 个槽被写入后被 fill 读取，免 64B 定值栈写
    let mut rec: [MaybeUninit<*mut LuaTable>; INDEX_CHAIN_MAX] =
      [MaybeUninit::uninit(); INDEX_CHAIN_MAX];
    let mut rec_len = 0usize;

    // 保留计数重复：MAXTAGLOOP 是 __index 链防失控的重试预算上限，不是数组下标；
    // 每轮沿元方法链把 t 换成下一级 TM 对象再走，无可迭代的数据序列
    for _ in 0..MAXTAGLOOP {
      let tm: *const TValue;
      if t.get().is_table() {
        let h = t.get().as_table_ptr();

        let res = lua_h_get(h, key.get());

        if res != LUA_O_NILOBJECT {
          (*l).cachedslot = gval2slot!(h, res);
        }

        if !(*res).is_nil() {
          setobj_2_s!(l, val.as_ptr(), res);
          // 回填：非首表命中（rec_len ≥ 1）且全链为表——缓存「守卫链 + owner」
          if rec_ok && rec_len > 0 {
            index_chain_fill(t0, mt0, k, &rec, rec_len);
          }
          return;
        }

        tm = fasttm(l, (*h).metatable, TMS::TmIndex);
        if tm.is_null() {
          setobj_2_s!(l, val.as_ptr(), res);
          return;
        }

        // 链记录：tm 为表则续记，函数元方法/超深/杂型 → 本调用不可缓存
        if rec_ok {
          if (*tm).is_table() && rec_len < INDEX_CHAIN_MAX {
            rec[rec_len].write((*tm).as_table_ptr());
            rec_len += 1;
          } else {
            rec_ok = false;
          }
        }
      } else if fflag::DebugLuauUserDefinedClassesRuntime.get() && t.get().is_object() {
        let inst = objectvalue!(t.as_const_ptr());
        let offsettval = lua_h_get((*(*inst).lclass).memberstooffset, key.get());

        if (*offsettval).is_nil() {
          lua_g_missingmembererror(l, t.as_const_ptr(), key.as_const_ptr());
        }

        LUAU_ASSERT!((*offsettval).is_number());
        let offset = (*offsettval).as_number() as i32;
        setobj_2_s!(l, val.as_ptr(), luaR_lookupmemberatoffset!(inst, offset));
        return;
      } else if fflag::DebugLuauUserDefinedClassesRuntime.get() && t.get().is_class() {
        let lco = classvalue!(t.as_const_ptr());
        let res = lua_h_get((*lco).memberstooffset, key.get());

        if (*res).is_nil() {
          lua_g_missingmembererror(l, t.as_const_ptr(), key.as_const_ptr());
        }

        LUAU_ASSERT!((*res).is_number());
        let offset = (*res).as_number() as i32;
        LUAU_ASSERT!(offset >= 0 && offset < (*lco).numberofallmembers);

        if offset < (*lco).numberofinstancemembers {
          lua_g_missingmembererror(l, t.as_const_ptr(), key.as_const_ptr());
        }

        let static_idx = (offset - (*lco).numberofinstancemembers) as usize;
        let static_slot = (*lco).staticmembers.add(static_idx);

        setobj_2_s!(l, val.as_ptr(), static_slot);
        return;
      } else {
        rec_ok = false;
        tm = lua_t_gettmbyobj(l, t.as_const_ptr(), TMS::TmIndex);
        if (*tm).is_nil() {
          lua_g_indexerror(l, t.as_const_ptr(), key.as_const_ptr());
        }
      }

      if (*tm).is_function() {
        call_t_mres(l, val.as_ptr(), tm, t.as_const_ptr(), key.as_const_ptr());
        return;
      }
      // SAFETY: `tm` 已在上方被解引用判型（非空、对齐、元对象存活期内可读），
      // 沿 __index 链只读存续；重派生句柄后下一轮仅使用读面，与原裸指针形态同址同序。
      t = Slot::from_raw(tm.cast_mut());
    }
    lua_g_runerror!(l, "'__index' chain too long; possible loop");
  }
}

/// # Safety
/// C ABI 导出壳：签名与符号不动，在裸指针边界处显式重建句柄后转调 [`lua_v_gettable`]，
/// 前置条件与该函数相同。
pub unsafe extern "C-unwind" fn lua_v_gettable_export(
  l: *mut LuaState,
  t: *const TValue,
  key: *mut TValue,
  val: StkId,
) {
  // SAFETY: 导出壳原样转调同契约 `lua_v_gettable`；l/t/key/val 满足其前置。
  // 句柄重建归本壳：`t` 为只读存活 TValue（被调方仅用读面，`from_ref` 纪律一致）；
  // `key`/`val` 为栈槽/受栈持有内存，调用期间不迁移，解引用窗口止于本调用。
  unsafe {
    lua_v_gettable(
      l,
      Slot::from_ref(&*t),
      Slot::from_raw(key),
      Slot::from_raw(val),
    );
  }
}
