// 边界契约测试：null 系 c-API 合法实参（既有约定 review.md §2）
use ulua_vm::{records::lua_state::LuaState, type_aliases::lua_c_function::LuaCFunction};

use crate::common::functions::{
  int_64_add::int_64_add,
  int_64_ctor::int_64_ctor,
  int_64_div::int_64_div,
  int_64_eq::int_64_eq,
  int_64_idiv::int_64_idiv,
  int_64_index::int_64_index,
  int_64_le::int_64_le,
  int_64_lt::int_64_lt,
  int_64_mod::int_64_mod,
  int_64_mul::int_64_mul,
  int_64_newindex::int_64_newindex,
  int_64_pow::int_64_pow,
  int_64_sub::int_64_sub,
  int_64_tostring::int_64_tostring,
  int_64_unm::int_64_unm,
  safe_api::{pushcclosurek, state_mut},
};

/// 在栈顶元表上登记元方法：压入 C 函数并写入 `name` 字段。
fn set_meta_method(l: *mut LuaState, name: &'static str, f: LuaCFunction) {
  pushcclosurek(l, f, None, 0, None);
  state_mut(l).set_field_str(-2, name);
}

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_userdata_setup(l: *mut LuaState) {
  // 建 int64 metatable 并置于栈顶，后续 set_meta_method 都以该栈顶元表为目标。
  state_mut(l).new_metatable_by_str("int64");

  // 栈顶为 int64 元表；set_meta_method 每次压一个 `extern "C-unwind"`
  // 桩再 setfield 消费，栈形不变，名字均为 NUL 结尾静态串。索引/比较元方法组。
  {
    set_meta_method(l, "__index", Some(int_64_index));
    set_meta_method(l, "__newindex", Some(int_64_newindex));
    set_meta_method(l, "__eq", Some(int_64_eq));
    set_meta_method(l, "__lt", Some(int_64_lt));
    set_meta_method(l, "__le", Some(int_64_le));
  }

  // 同上——栈顶仍是该元表，逐条登记算术元方法。
  {
    set_meta_method(l, "__add", Some(int_64_add));
    set_meta_method(l, "__sub", Some(int_64_sub));
    set_meta_method(l, "__mul", Some(int_64_mul));
    set_meta_method(l, "__div", Some(int_64_div));
    set_meta_method(l, "__idiv", Some(int_64_idiv));
  }

  // 同上——栈顶仍是该元表，登记取模/取负/转串元方法。
  {
    set_meta_method(l, "__mod", Some(int_64_mod));
    set_meta_method(l, "__pow", Some(int_64_pow));
    set_meta_method(l, "__unm", Some(int_64_unm));
    set_meta_method(l, "__tostring", Some(int_64_tostring));
  }

  // 构造函数注册为全局 `int64`：用 NUL 结尾名字建闭包后 setglobal 消费栈顶。
  pushcclosurek(l, Some(int_64_ctor), Some(b"int64\0"), 0, None);
  state_mut(l).set_global_str("int64");
}
