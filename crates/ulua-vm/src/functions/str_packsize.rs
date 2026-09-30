use crate::{
  enums::k_option::KOption,
  functions::{getdetails::getdetails, getnum::FmtCursor, initheader::initheader},
  macros::{lua_lib_fn::lua_lib_fn, maxssize::MAXSSIZE},
  records::{header::Header, lua_state::LuaState},
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `LuaState`.
pub(crate) unsafe fn str_packsize(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 存活且格式串实参为可读串数据，getdetails/getnum 解析仅在该串界内推进
  unsafe {
    let mut h = Header::default();
    let mut fmt = FmtCursor::from_ptr((*l).check_bytes(1).as_ptr().cast());
    let mut totalsize: usize = 0;

    initheader(l, &mut h);

    while fmt.cur() != 0 {
      let (opt, size, ntoalign) = getdetails(&mut h, totalsize, &mut fmt);

      (*l).arg_check(
        opt != KOption::Kstring && opt != KOption::Kzstr,
        1,
        "variable-length format",
      );

      let total_option_size = (size + ntoalign) as usize;
      (*l).arg_check(
        totalsize <= MAXSSIZE as usize - total_option_size,
        1,
        "format result too large",
      );

      totalsize += total_option_size;
    }

    (*l).push_integer(totalsize as i32);
    1
  }
}

lua_lib_fn!(pub(crate) fn str_packsize, str_packsize_arm);
