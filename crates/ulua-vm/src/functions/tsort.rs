use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_g_readonlyerror::check_writable, lua_h_getn::lua_h_getn,
    lua_v_lessthan::lua_v_lessthan_export, sort_func::sort_func, sort_rec::sort_rec,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
  type_aliases::sort_predicate::SortPredicate,
};

/// # Safety
/// `l` 的存活/独占前提已由 `&mut` 接收者类型承载（r16-v41 收形）；屏障仍保留是因为 1 号槽表裸句柄
/// `t` 须跨整趟 `sort_rec` 递归存活，而 `sort_rec` 每层比较都可能落入用户谓词 `sort_func`（任意
/// Lua）：谓词一旦改写本表（插删触发 rehash/数组段重分配即搬移其存储），`sort_rec`/`sort_swap`
/// 沿用的数组/节点窗即悬空并被继续读写。「谓词不改表」是 `sort_rec` 既有入约、亦属调用方无从外证的
/// 内部不变量，失守即 UB，故本体不降安全（形制照抄 tunpack r16-v41 判例：入口一次就地转手
/// `lp = l.as_mut_ptr()`，全程只用 `lp`，与原 `*mut l` 逐位一致）。
/// 余下调用序前提：`l` 须处于可抛错受保护帧——栈 1 号位为 table（`check_type` 否则发散），2 号位
/// 可选 function（非 nil 即 `check_type(2, Function)` 复核），只读表经 `check_writable` 发散，
/// `set_top(2)` 后帧顶恰含两参（谓词压栈窗由此预留）。
/// cpp/VM/src/ltablib.cpp:550 tsort。
pub unsafe fn tsort(l: &mut LuaState) -> i32 {
  // SAFETY: `lp` 为 `&mut` 就地转手的同一存活帧；`t` 借自 check_type 刚钉住的 1 号槽存活表，
  // 其跨调用存续即上方契约所载。
  unsafe {
    let lp = l.as_mut_ptr();
    (*lp).check_type(1, LuaType::Table);

    let t = (*(*lp).base).as_table_ptr();
    let n = lua_h_getn(t);

    check_writable(lp, t);

    let mut pred: SortPredicate = Some(lua_v_lessthan_export);
    if !(*lp).is_none_or_nil(2) {
      (*lp).check_type(2, LuaType::Function);
      pred = Some(sort_func);
    }
    (*lp).set_top(2);

    if n > 0 {
      sort_rec(lp, t, 0, n - 1, n, pred);
    }

    0
  }
}

lua_lib_fn!(pub fn tsort @ref, tsort_arm);
