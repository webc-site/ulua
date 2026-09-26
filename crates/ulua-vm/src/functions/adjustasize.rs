use crate::{
  functions::{arrayindex::arrayindex, lua_h_getnum::lua_h_getnum},
  macros::dummynode::dummynode,
  records::lua_table::LuaTable,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `t` 须为存活 `LuaTable`：读 `(*t).node`/`(*t).sizearray` 判定数组边界，并沿 `lua_h_getnum(t,size+1)` 探测
/// 数组段（`size` 增长受 `tbound`（node 非 dummynode 或 size<sizearray）与 `size!=i32::MAX` 约束，避免越界/溢出）；
/// `ek` 允许 NULL（此时 ekindex=-1 不探测），非空时须指向存活 `TValue` 且仅当为数字才解引用取值。纯查询，不写表、不分配。
/// cpp VM/src/ltable.cpp:679
pub(crate) unsafe fn adjustasize(t: *mut LuaTable, mut size: i32, ek: *const TValue) -> i32 {
  unsafe {
    let tbound = (*t).node != dummynode.cast_mut() || size < (*t).sizearray;
    let ekindex = if !ek.is_null() && (*ek).is_number() {
      arrayindex((*ek).as_number())
    } else {
      -1
    };

    // move the array size up until the boundary is guaranteed to be inside the array part.
    // Stop at INT_MAX: the array can't be larger, and `size + 1` there overflows `int`
    // (UB in C++ Luau / a panic with overflow-checks on — found by the run fuzz target).
    while size != i32::MAX
      && (size + 1 == ekindex || (tbound && !(*lua_h_getnum(t, size + 1)).is_nil()))
    {
      size += 1;
    }

    size
  }
}
