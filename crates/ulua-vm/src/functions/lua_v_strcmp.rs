use core::{cmp::Ordering, slice::from_raw_parts};

use crate::{macros::getstr::getstr, records::t_string::tstring};

/// # Safety
/// `ls/rs` 须指向存活 interned `tstring`（cpp lvmutils.cpp:400）：`getstr` 取回的数据指针
/// 与 `len` 字段一致且尾字节恒有 NUL 兜底，故首字节单读与 `min(len)` memcmp 均在串界内。
pub unsafe fn lua_v_strcmp(ls: *const tstring, rs: *const tstring) -> i32 {
  if ls == rs {
    return 0;
  }

  // Safety: 契约保证两侧 tstring 为存活串对象，getstr 取回的数据与长度一致且 memcmp 仅在串界内比较
  unsafe {
    let l = getstr(ls);
    let r = getstr(rs);

    // always safe to read one character because even empty strings are nul terminated
    let bl = *l as u8;
    let br = *r as u8;

    if bl != br {
      return (bl as i32) - (br as i32);
    }

    let ll = (*ls).len as usize;
    let lr = (*rs).len as usize;
    let lmin = if ll < lr { ll } else { lr };

    // u8 切片字典序比较与 C memcmp 语义一致（unsigned char 逐字节比较），
    // 编译期可下沉为 memcmp/bcmp，无需 C ABI
    let ord = from_raw_parts(l, lmin).cmp(from_raw_parts(r, lmin));

    match ord {
      Ordering::Less => -1,
      Ordering::Greater => 1,
      Ordering::Equal => {
        if ll == lr {
          0
        } else if ll < lr {
          -1
        } else {
          1
        }
      }
    }
  }
}
