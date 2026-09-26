use core::ptr::null;

use ulua_vm::{
  functions::{
    lua_l_newmetatable::lua_l_newmetatable, lua_pushcclosurek::lua_pushcclosurek,
    lua_setfield::lua_setfield,
  },
  macros::lua_setglobal::lua_setglobal,
  records::lua_state::LuaState,
  type_aliases::lua_c_function::LuaCFunction,
};

use crate::common::functions::{
  cstr::cstr, int_64_add::int_64_add, int_64_ctor::int_64_ctor, int_64_div::int_64_div,
  int_64_eq::int_64_eq, int_64_idiv::int_64_idiv, int_64_index::int_64_index, int_64_le::int_64_le,
  int_64_lt::int_64_lt, int_64_mod::int_64_mod, int_64_mul::int_64_mul,
  int_64_newindex::int_64_newindex, int_64_pow::int_64_pow, int_64_sub::int_64_sub,
  int_64_tostring::int_64_tostring, int_64_unm::int_64_unm,
};

/// 在栈顶元表上登记元方法：压入 C 函数并写入 `name` 字段。
unsafe fn set_meta_method(l: *mut LuaState, name: &'static [u8], f: LuaCFunction) {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    lua_pushcclosurek(l, f, null(), 0, None);
    lua_setfield(l, -2, name.as_ptr().cast());
  }
}

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_userdata_setup(l: *mut LuaState) {
  // Safety: `l` 为本用例存活的 LuaState；建 int64 metatable 并置于栈顶，
  // 后续 set_meta_method 都以该栈顶元表为目标。
  unsafe { lua_l_newmetatable(l, cstr(b"int64\0")) };

  // Safety: `l` 存活且栈顶为 int64 元表；set_meta_method 每次压一个 `extern "C-unwind"`
  // 桩再 setfield 消费，栈形不变，名字均为 NUL 结尾静态串。索引/比较元方法组。
  unsafe {
    set_meta_method(l, b"__index\0", Some(int_64_index));
    set_meta_method(l, b"__newindex\0", Some(int_64_newindex));
    set_meta_method(l, b"__eq\0", Some(int_64_eq));
    set_meta_method(l, b"__lt\0", Some(int_64_lt));
    set_meta_method(l, b"__le\0", Some(int_64_le));
  }

  // Safety: 同上——栈顶仍是该元表，逐条登记算术元方法。
  unsafe {
    set_meta_method(l, b"__add\0", Some(int_64_add));
    set_meta_method(l, b"__sub\0", Some(int_64_sub));
    set_meta_method(l, b"__mul\0", Some(int_64_mul));
    set_meta_method(l, b"__div\0", Some(int_64_div));
    set_meta_method(l, b"__idiv\0", Some(int_64_idiv));
  }

  // Safety: 同上——栈顶仍是该元表，登记取模/取负/转串元方法。
  unsafe {
    set_meta_method(l, b"__mod\0", Some(int_64_mod));
    set_meta_method(l, b"__pow\0", Some(int_64_pow));
    set_meta_method(l, b"__unm\0", Some(int_64_unm));
    set_meta_method(l, b"__tostring\0", Some(int_64_tostring));
  }

  // 构造函数注册为全局 `int64`
  // Safety: `l` 存活；pushcclosurek 用 NUL 结尾名字建闭包后 setglobal 消费栈顶。
  unsafe {
    lua_pushcclosurek(l, Some(int_64_ctor), cstr(b"int64\0"), 0, None);
    lua_setglobal(l, cstr(b"int64\0"));
  }
}
