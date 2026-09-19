use core::ptr::null;

use crate::{
  enums::tms::TMS,
  functions::lua_t_gettm::lua_t_gettm,
  records::{global_state::global_State, lua_t_value::TValue, lua_table::LuaTable},
};

/// cpp `ltm.h:43` `#define gfasttm(g, et, e)` 对应。
///
/// `e` 为 `TMS`：枚举判别式即位号，杜绝把任意 i32 位移进来的 UB 面。
///
/// # Safety
///
/// 若访问 `g`，它必须指向有效的 `global_State`。
/// `et` 非空时必须指向有效的 `LuaTable`。
#[inline(always)]
pub(crate) unsafe fn gfasttm(g: *mut global_State, et: *mut LuaTable, e: TMS) -> *const TValue {
  unsafe {
    if et.is_null() {
      null()
    } else {
      let tmcache = (*et).tmcache;
      // tmcache 位已置位表示「该元方法不存在」的缓存，直接返回 NULL（cpp 同款 fast 缺席判断）
      if (tmcache as u32 & (1u32 << (e as u32))) != 0 {
        null()
      } else {
        // tmname 覆盖 TMS 除 TmN 外的全部判别式，越界仅可能来自误传 TmN：
        // 索引带运行时边界检查，宁可 panic 也不越界读
        lua_t_gettm(et, e, (*g).tmname[e as usize])
      }
    }
  }
}
