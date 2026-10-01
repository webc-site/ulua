//! Source: `VM/src/lstrlib.cpp:1407`

use core::{
  ffi::{c_double, c_float},
  mem::size_of,
};

use memchr::memchr;

use crate::{
  enums::k_option::KOption,
  functions::{
    copywithendian::copywithendian, getdetails::getdetails, getnum::FmtCursor,
    initheader::initheader, lua_l_addchar::lua_l_addchar, lua_l_addlstring::lua_l_addlstring,
    lua_l_buffinit::lua_l_buffinit, lua_l_checklstring::lua_l_checklstring_ref,
    lua_l_pushresult::lua_l_pushresult, packint::packint,
  },
  macros::{lua_lib_fn::lua_lib_fn, maxintsize::MAXINTSIZE},
  records::{ftypes::Ftypes, header::Header, lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

pub const LUAL_PACKPADBYTE: u8 = 0x00;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn str_pack(l: *mut LuaState) -> i32 {
  unsafe {
    let mut b = LuaLStrbuf::new();
    let mut h = Header::default();
    let mut fmt = FmtCursor::from_ptr((*l).check_bytes(1).as_ptr().cast()); // format string
    let mut arg = 1; // current argument to pack
    let mut totalsize: usize = 0; // accumulate total size of result
    initheader(l, &mut h);
    (*l).push_nil(); // mark to separate arguments from string buffer
    lua_l_buffinit(&mut *l, &mut b);
    while fmt.cur() != 0 {
      let (opt, size, ntoalign) = getdetails(&mut h, totalsize, &mut fmt);
      totalsize += (ntoalign + size) as usize;
      // 保留计数重复：ntoalign 是布局对齐算出的填充字节数，循环变量不参与取数，
      // 每轮仅向缓冲区追加同一个 pad 字节，无数据序列可迭代
      for _ in 0..ntoalign {
        lua_l_addchar(&mut b, LUAL_PACKPADBYTE);
      }
      arg += 1;
      match opt {
        KOption::Kint | KOption::Kuint => {
          // 有/无符号双臂仅差溢出校验式与 packint 符号填充位
          let signed = opt == KOption::Kint;
          let n = (*l).check_number(arg) as i64;
          if size < size_of::<i64>() as i32 {
            // need overflow check?
            if signed {
              let lim = 1i64 << ((size * 8) - 1);
              (*l).arg_check(-lim <= n && n < lim, arg, "integer overflow");
            } else {
              (*l).arg_check((n as u64) < (1u64 << (size * 8)), arg, "unsigned overflow");
            }
          }
          packint(
            &mut b,
            n as u64,
            h.islittle,
            size,
            if signed { (n < 0) as i32 } else { 0 },
          );
        }
        KOption::Kfloat => {
          // floating-point options
          let mut u = Ftypes { n: 0.0 };
          let mut buff = [0u8; MAXINTSIZE as usize];
          let n = (*l).check_number(arg); // get argument
          if size as usize == size_of::<c_float>() {
            u.f = n as c_float; // copy it into 'u'
          } else if size as usize == size_of::<c_double>() {
            u.d = n;
          } else {
            u.n = n;
          }
          // move 'u' to final result, correcting endianness if needed
          let (dst, src) = (&mut buff[..size as usize], &u.buff[..size as usize]);
          copywithendian(dst, src, h.islittle);
          lua_l_addlstring(&mut b, dst);
        }
        KOption::Kchar => {
          // fixed-size string
          let s = lua_l_checklstring_ref(&mut *l, arg);
          let len = s.len();
          (*l).arg_check(len <= size as usize, arg, "string longer than given size");
          lua_l_addlstring(&mut b, s); // add string
          // 补零到定宽 size：循环变量不参与取数，纯计数重复
          //（原 `while len < size` 游走收为区间迭代；argcheck 已保证 len <= size）
          for _ in len..(size as usize) {
            lua_l_addchar(&mut b, LUAL_PACKPADBYTE);
          }
        }
        KOption::Kstring => {
          // strings with length count
          let s = lua_l_checklstring_ref(&mut *l, arg);
          let len = s.len();
          (*l).arg_check(
            size >= size_of::<usize>() as i32 || len < (1usize << (size * 8)),
            arg,
            "string length does not fit in given size",
          );
          packint(&mut b, len as u64, h.islittle, size, 0); // pack length
          lua_l_addlstring(&mut b, s);
          totalsize += len;
        }
        KOption::Kzstr => {
          // zero-terminated string
          let s = lua_l_checklstring_ref(&mut *l, arg);
          let len = s.len();
          // 原 cstr_bytes strlen==len 的「无内嵌零」判定收为 memchr 单遍扫描
          (*l).arg_check(memchr(0, s).is_none(), arg, "string contains zeros");
          lua_l_addlstring(&mut b, s);
          lua_l_addchar(&mut b, 0); // add zero at the end
          totalsize += len + 1;
        }
        KOption::Kpadding => {
          lua_l_addchar(&mut b, LUAL_PACKPADBYTE);
          arg -= 1; // undo increment
        }
        KOption::Kpaddalign | KOption::Knop => {
          arg -= 1; // undo increment
        }
      }
    }
    lua_l_pushresult(&mut b);
    1
  }
}

lua_lib_fn!(pub(crate) fn str_pack, str_pack_arm);
