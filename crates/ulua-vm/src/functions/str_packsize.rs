use crate::{
  enums::k_option::KOption,
  functions::{getdetails::getdetails, getnum::FmtCursor, initheader::initheader},
  macros::{lua_lib_fn::lua_lib_fn, maxssize::MAXSSIZE},
  records::{header::Header, lua_state::LuaState},
};

/// packsize 布局核算切片核（真实逻辑）：cpp `str_packsize`（lstrlib.cpp:1490）的
/// 选项循环、`variable-length format`/`format result too large` 抛出点与
/// totalsize 累加序逐点对齐 oracle；`fmt_bytes` 为 1 号格式串 payload 切片，
/// 游标读窗入约见 `getnum.rs`（串尾经 `at` 归一为 NUL，无裸窗）。
///
/// # Safety
/// `l` 须为可抛错受保护帧内存活的 `LuaState`：`initheader` 接线 `h.l` 后，
/// `getdetails`/`arg_check` 报错内核经其解引用抛出并 unwind（不返回）。
unsafe fn str_packsize_ref(l: &mut LuaState, fmt_bytes: &[u8]) -> i32 {
  // SAFETY: `l` 由垫片契约保证存活独占驱动，`h.l` 接至同一存活帧；
  // getdetails/getnum 解析仅在 payload 界内推进（越尾经 `at` 归一为 NUL）
  unsafe {
    let mut h = Header::default();
    let mut fmt = FmtCursor::from_slice(fmt_bytes);
    let mut totalsize: usize = 0;

    initheader(l, &mut h);

    while fmt.cur() != 0 {
      let (opt, size, ntoalign) = getdetails(&mut h, totalsize, &mut fmt);

      l.arg_check(
        opt != KOption::Kstring && opt != KOption::Kzstr,
        1,
        "variable-length format",
      );

      let total_option_size = (size + ntoalign) as usize;
      l.arg_check(
        totalsize <= MAXSSIZE as usize - total_option_size,
        1,
        "format result too large",
      );

      totalsize += total_option_size;
    }

    l.push_integer(totalsize as i32);
    1
  }
}

/// C-ABI 镜像垫片（一行委托 [`str_packsize_ref`]）：本面消费点仅 `lua_lib_fn!`
/// 生成的注册臂 `str_packsize_arm`（luaopen_string.rs:7 导入、:33 STRLIB
/// "packsize" 注册），全仓实测无其余消费面 ⇒ 保一行形（r12-w5s `byteoffset`
/// 垫片先例）。
///
/// # Safety
/// `l` 须为可抛错受保护帧内存活的 `LuaState`：栈槽 #1 为格式串实参（非串经
/// `check_bytes` 抛 "string expected"），其余义务单源 [`str_packsize_ref`]；
/// `&mut *l` 的引用重建窗口即本次调用。
pub(crate) unsafe fn str_packsize(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 存活独占驱动；payload 切片借用自栈槽 #1 串体，
  // 本次调用内有效
  unsafe { str_packsize_ref(&mut *l, (*l).check_bytes(1)) }
}

lua_lib_fn!(pub(crate) fn str_packsize, str_packsize_arm);
