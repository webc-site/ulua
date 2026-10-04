use crate::{
  enums::lua_type::LuaType,
  functions::{
    c_slice, lua_checkstack::lua_checkstack, lua_l_optinteger::lua_l_optinteger,
    lua_rawgeti::lua_rawgeti,
  },
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn, setobj_2_s::setobj_2_s},
  records::{lua_state::LuaState, lua_t_value::TValue},
};

/// # Safety
/// `l` 的存活/独占前提已由 `&mut` 接收者类型承载（r16-v41 收形）；屏障仍保留是因为体内有真实
/// 裸操作：`(*l.base)` 解引用栈基址取表对象裸句柄 `t`（`as_table_ptr`），`(*t).sizearray`/`(*t).array`
/// 须跨过 `lua_checkstack`（只重建栈指针、不挪堆上 `Table`）与快路径整轮 `setobj_2_s!(lp,..)` 写槽
/// 存活——表句柄与 reserved_slots 栈窗与后继 `&mut l` 调用交叠属 p28 锚定形，故依 r16-v29/r16-v38
/// str_gsub 判例在入口一次就地转手裸句柄 `lp = l.as_mut_ptr()`，全程只用 `lp`，与原 `*mut l` 逐位一致。
/// 余下调用序前提：`l` 须处于 `tunpack` 调用的可抛错受保护帧，栈 1 号位为 table（`check_type` 否则抛错
/// 发散），2/3 号位可选整数边界，`lua_checkstack` 失败或结果数超 `INT_MAX` 经 `luaL_error` 发散。
/// cpp `ltablib.cpp:365`。
pub unsafe fn tunpack(l: &mut LuaState) -> i32 {
  // SAFETY: `l` 由 `&mut` 保证有效且独占，转手后的 `lp` 即同一存活帧；表句柄 `t` 借自堆上 Table 对象
  // （lua_checkstack 仅重建栈指针不挪 Table），快路径栈窗写序与原逐格形态一致
  unsafe {
    let lp = l.as_mut_ptr();
    (*lp).check_type(1, LuaType::Table);
    let t = (*(*lp).base).as_table_ptr();

    let i = lua_l_optinteger(&mut *lp, 2, 1);
    let e = (*lp).obj_len(1) as i32;
    let e = lua_l_optinteger(&mut *lp, 3, e);

    if i > e {
      return 0; // empty range
    }

    // `n` here is the element count MINUS ONE. C++ guards on this value
    // (`n >= INT_MAX`) BEFORE adding one, so a full-range request
    // (i = INT_MIN, e = INT_MAX -> n = 0xFFFF_FFFF) is rejected. Adding one first
    // (as the previous port did) wrapped n to 0, passed the guard, and let the
    // push loop overrun the stack into an api_incr_top assert (SIGTRAP).
    let n = (e as u32).wrapping_sub(i as u32); // number of elements minus 1 (avoid overflows)
    if n >= i32::MAX as u32 || lua_checkstack(&mut *lp, n.wrapping_add(1) as i32) == 0 {
      luaL_error!(&mut *lp, "too many results to unpack");
    }
    let n = n + 1; // safe: guard above guarantees n (minus one) < INT_MAX

    // fast-path: direct array-to-stack copy
    if i == 1 && (n as i32) <= (*t).sizearray {
      // SAFETY:快路径已断言 n <= sizearray；lua_checkstack 扩容先行覆盖 n 预留槽
      // （`reserved_slots_mut` 窗契约），栈上 n 个槽位随后由 `advance_top` 提交，
      // 写序与顶抬升时序同原逐格形态。
      for (dst, src) in (*lp)
        .reserved_slots_mut(n as usize)
        .iter_mut()
        .zip(c_slice((*t).array, n as usize))
      {
        setobj_2_s!(lp, dst as *mut TValue, src as *const TValue as *mut TValue);
      }
      (*lp).advance_top(n as usize);
    } else {
      // push arg[i..e - 1] (to avoid overflows)：cpp `while current_i < e` 游走
      // 收为区间迭代，末元素单独压栈（i <= e 此前已由空区间早退保证）
      for current_i in i..e {
        lua_rawgeti(&mut *lp, 1, current_i);
      }
      lua_rawgeti(&mut *lp, 1, e); // push last element
    }

    n as i32
  }
}

lua_lib_fn!(pub fn tunpack @ref, tunpack_arm);
