use crate::{
  functions::newkey::newkey,
  macros::luai_numeq::luai_numeq,
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
///
/// `l`/`t`/`key` 须满足 `newkey` 的入约（存活 LuaState/LuaTable/可读 TValue）；命中
/// 数组分支时 `k-1 < sizearray` 已判定，返回值落在 t 的 array 或哈希值槽内。
pub(crate) unsafe fn arrayornewkey(
  l: *mut LuaState,
  t: *mut LuaTable,
  key: *const TValue,
) -> *mut TValue {
  // Safety: 契约保证 t/key 存活可读；数组命中分支的 add(k-1) 已由 (k as u32)-1<sizearray 界检
  unsafe {
    if (*key).is_number() {
      let n = (*key).as_number();
      let k = n as i32;

      if luai_numeq(k as f64, n) && (k as u32).wrapping_sub(1) < (*t).sizearray as u32 {
        return (*t).array.add((k - 1) as usize);
      }
    }

    newkey(l, t, key)
  }
}
