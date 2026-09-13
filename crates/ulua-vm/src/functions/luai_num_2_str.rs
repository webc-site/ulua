use core::ffi::c_char;

use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_unlikely::LUAU_UNLIKELY};

use crate::{
  functions::{
    printexp::printexp, printspecial::printspecial, printunsignedrev::printunsignedrev,
    schubfach::schubfach, trimzero::trimzero,
  },
  macros::{fastmemcpy::fastmemcpy, fastmemset::fastmemset},
  records::decimal::Decimal,
};

pub(crate) unsafe fn luai_num_2_str(mut buf: *mut c_char, n: f64) -> *mut c_char {
  unsafe {
    // IEEE-754
    let bits = n.to_bits();
    let sign = (bits >> 63) as i32;
    let exponent = ((bits >> 52) & 2047) as i32;
    let fraction = bits & ((1u64 << 52) - 1);

    // specials
    if LUAU_UNLIKELY!(exponent == 0x7ff) {
      return printspecial(buf, sign, fraction);
    }

    // sign bit
    *buf = '-' as c_char;
    buf = buf.add(sign as usize);

    // zero
    if exponent == 0 && fraction == 0 {
      *buf = '0' as c_char;
      return buf.add(1);
    }

    // convert binary to decimal using Schubfach
    let d: Decimal = schubfach(exponent, fraction);
    LUAU_ASSERT!(d.s < 1e17 as u64);

    // print the decimal to a temporary buffer; we'll need to insert the decimal point and figure out the format
    let mut decbuf = [0i8; 40];
    let decend = decbuf.as_mut_ptr().add(20) as *mut c_char; // significand needs at most 17 digits
    let dec = printunsignedrev(decend, d.s);

    let declen = decend.offset_from(dec) as i32;
    LUAU_ASSERT!(declen <= 17);

    let dot = declen + d.k;

    // the limits are somewhat arbitrary but changing them may require changing fastmemset/fastmemcpy sizes below
    if (-5..=21).contains(&dot) {
      // fixed point format
      if dot <= 0 {
        *buf = '0' as c_char;
        *buf.add(1) = '.' as c_char;

        fastmemset!(buf.add(2), '0', -dot, 5);
        fastmemcpy!(buf.add(2 + (-dot) as usize), dec, declen, 17);

        trimzero(buf.add(2 + (-dot) as usize + declen as usize))
      } else if dot == declen {
        // no dot
        fastmemcpy!(buf, dec, dot, 17);

        buf.add(dot as usize)
      } else if dot < declen {
        // dot in the middle
        fastmemcpy!(buf, dec, dot, 16);

        *buf.add(dot as usize) = '.' as c_char;

        fastmemcpy!(
          buf.add(dot as usize + 1),
          dec.add(dot as usize),
          declen - dot,
          16
        );

        trimzero(buf.add(declen as usize + 1))
      } else {
        // no dot, zero padding
        fastmemcpy!(buf, dec, declen, 17);
        fastmemset!(buf.add(declen as usize), '0', dot - declen, 8);

        buf.add(dot as usize)
      }
    } else {
      // scientific format
      *buf = *dec;
      *buf.add(1) = '.' as c_char;
      fastmemcpy!(buf.add(2), dec.add(1), declen - 1, 16);

      let mut exp = trimzero(buf.add(declen as usize + 1));

      if *exp.sub(1) == '.' as c_char {
        exp = exp.sub(1);
      }

      printexp(exp, dot - 1)
    }
  }
}

#[unsafe(export_name = "ulua_luai_num2str")]
pub(crate) unsafe fn luai_num2str(buf: *mut c_char, n: f64) -> *mut c_char {
  unsafe { luai_num_2_str(buf, n) }
}
