use crate::{
  functions::c_slice,
  macros::{maxbits::MAXBITS, ttisnil::ttisnil},
  records::lua_table::LuaTable,
  type_aliases::t_value::TValue,
};

/// cpp `numusearray`（ltable.cpp）：按 2 的幂区间统计数组部分的非 nil 元素，
/// 计数累加进 `nums[lg]`，返回数组部分已用槽位数。
///
/// cpp 的 `int* nums` 出参数组在 Rust 侧以 `&mut [i32]` 借用传递。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
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
        .filter(|&v| !ttisnil!(v as *const TValue))
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
