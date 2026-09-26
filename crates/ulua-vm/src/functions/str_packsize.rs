use crate::{
  enums::k_option::KOption,
  functions::{
    getdetails::getdetails, getnum::FmtCursor, initheader::initheader,
    lua_pushinteger::lua_pushinteger,
  },
  macros::{
    lua_l_argcheck::luaL_argcheck, lua_l_checkstring::luaL_checkstring, lua_lib_fn::lua_lib_fn,
    maxssize::MAXSSIZE,
  },
  records::{header::Header, lua_state::LuaState},
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `LuaState`.
pub(crate) unsafe fn str_packsize(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 存活且格式串实参为可读串数据，getdetails/getnum 解析仅在该串界内推进
  unsafe {
    let mut h = Header::default();
    let mut fmt = FmtCursor::from_ptr(luaL_checkstring!(l, 1));
    let mut totalsize: usize = 0;

    initheader(l, &mut h);

    while fmt.cur() != 0 {
      let (opt, size, ntoalign) = getdetails(&mut h, totalsize, &mut fmt);

      luaL_argcheck!(
        l,
        opt != KOption::Kstring && opt != KOption::Kzstr,
        1,
        "variable-length format"
      );

      let total_option_size = (size + ntoalign) as usize;
      luaL_argcheck!(
        l,
        totalsize <= MAXSSIZE as usize - total_option_size,
        1,
        "format result too large"
      );

      totalsize += total_option_size;
    }

    lua_pushinteger(l, totalsize as i32);
    1
  }
}

lua_lib_fn!(pub(crate) fn str_packsize, str_packsize_arm);
