use crate::{
  functions::{
    adjustasize::adjustasize, computesizes::computesizes, countint::countint,
    numusearray::numusearray, numusehash::numusehash, resize::resize,
  },
  macros::{maxbits::MAXBITS, nvalue::nvalue, ttisnumber::ttisnumber},
  records::{lua_state::lua_State, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn rehash(l: *mut lua_State, t: *mut LuaTable, ek: *const TValue) {
  unsafe {
    let mut nums = [0i32; (MAXBITS + 1) as usize];
    // 数组部分元素数（同时也是已计数的整数键数）
    let mut nasize = numusearray(t, &mut nums);
    let mut totaluse = nasize;
    // 哈希部分：ause 计入数组部分，totaluse 计入总用量
    let (ause, hashuse) = numusehash(t, &mut nums);
    nasize += ause;
    totaluse += hashuse;

    if ttisnumber!(ek) {
      nasize += countint(nvalue!(ek), &mut nums);
    }
    totaluse += 1;

    // 计算数组部分最优大小
    let (na, new_nasize) = computesizes(&nums, nasize);
    let mut nasize = new_nasize;
    let mut nh = totaluse - na;

    // enforce the boundary invariant; for performance, only do hash lookups if we must
    let nadjusted = adjustasize(t, nasize, ek);
    // count how many extra elements belong to array part instead of hash part
    let aextra = nadjusted - nasize;

    if aextra != 0 {
      // 那些额外元素不再需要哈希部分的槽位；且因哈希节点是数组节点的两倍大，
      // 省下的内存可交回数组部分（size 变更后需再次强制边界不变式）
      nh -= aextra;
      nasize = adjustasize(t, nadjusted + aextra, ek);
    }

    resize(l, t, nasize, nh);
  }
}
