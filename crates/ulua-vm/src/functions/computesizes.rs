use ulua_common::macros::luau_assert::LUAU_ASSERT;
pub(crate) unsafe fn computesizes(nums: *const i32, narray: *mut i32) -> i32 {
  unsafe {
    let mut twotoi: i32;
    let mut a: i32 = 0;
    let mut na: i32 = 0;
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    twotoi = 1;

    while twotoi / 2 < *narray {
      if *nums.add(i as usize) > 0 {
        a += *nums.add(i as usize);
        if a > twotoi / 2 {
          n = twotoi;
          na = a;
        }
      }
      if a == *narray {
        break;
      }
      i += 1;
      twotoi *= 2;
    }

    *narray = n;
    LUAU_ASSERT!(*narray / 2 <= na && na <= *narray);
    na
  }
}
