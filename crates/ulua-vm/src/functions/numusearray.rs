use crate::{functions::c_slice, macros::maxbits::MAXBITS, records::lua_table::LuaTable};

/// cpp `numusearray`（ltable.cpp）：按 2 的幂区间统计数组部分的非 nil 元素，
/// 计数累加进 `nums[lg]`，返回数组部分已用槽位数。
///
/// cpp 的 `int* nums` 出参数组在 Rust 侧以 `&mut [i32]` 借用传递。
///
/// # Safety
/// `t` 须为存活 `LuaTable`，其 `array` 指针非空且元素数恰为 `(*t).sizearray`（本函数按 `[0,sizearray)` 只读切片，
/// 全程不写数组、不分配、不抛错）；`nums` 借用长度须 ≥`MAXBITS+1`，因 `nums[lg]` 的 lg 取自 `0..=MAXBITS`。
/// cpp VM/src/ltable.cpp:455
pub(crate) unsafe fn numusearray(t: *const LuaTable, nums: &mut [i32]) -> i32 {
  unsafe {
    let sizearray = (*t).sizearray;
    // 数组部分整体只读，切片化后区间统计无需手工游标
    let array = c_slice((*t).array, sizearray as usize);

    let mut ause: i32 = 0; // `nums' 的累加和
    let mut i: i32 = 1; // 1 基游标，遍历全部数组键
    let mut lg: i32 = 0;
    let mut ttlg: i32 = 1; // 2^lg

    while lg <= MAXBITS {
      let mut lim: i32 = ttlg;

      if lim > sizearray {
        lim = sizearray; // adjust upper limit
        if i > lim {
          break; // no more elements to count
        }
      }

      // count elements in range (2^(lg-1), 2^lg]
      let lc = array[(i - 1) as usize..lim as usize]
        .iter()
        .filter(|&v| !v.is_nil())
        .count() as i32;
      i = lim + 1;

      nums[lg as usize] += lc;
      ause += lc;

      lg += 1;
      ttlg *= 2;
    }

    ause
  }
}
